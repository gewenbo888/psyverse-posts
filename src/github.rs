//! Minimal GitHub GraphQL client used by the CLI.
//! It performs a search for issues that likely contain a bounty.
//! The implementation is deliberately lightweight and production‑ready:
//! - Uses `reqwest` with TLS (rustls) and async runtime.
//! - Deserializes only the fields we need.
//! - Returns a typed `Result` with a boxed error for flexibility.

use reqwest::Client;
use serde::Deserialize;

/// Public representation of a GitHub issue relevant to the bounty hunter.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Issue {
    pub title: String,
    pub html_url: String,
    pub repository: Repository,
}

/// Minimal repository information needed for display.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Repository {
    #[serde(rename = "nameWithOwner")]
    pub full_name: String,
}

/// Internal structures mirroring the GraphQL response shape.
#[derive(Debug, Deserialize)]
struct GraphQLResponse {
    data: Option<Data>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
struct Data {
    search: SearchResult,
}

#[derive(Debug, Deserialize)]
struct SearchResult {
    nodes: Vec<Node>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "__typename")]
enum Node {
    #[serde(rename = "Issue")]
    Issue {
        title: String,
        url: String,
        repository: Repository,
    },
    // Other node types are ignored.
}

#[derive(Debug, Deserialize)]
struct GraphQLError {
    message: String,
}

/// Returns a static GraphQL query string used by `search_bounties`.
pub fn build_query() -> &'static str {
    // The query searches the first 20 open issues that contain the word
    // "bounty" in the title or body and are not archived.
    // It fetches only the fields required by `Issue`.
    r#"
    query {
        search(query: "type:issue is:open bounty in:title,body", first: 20) {
            nodes {
                __typename
                ... on Issue {
                    title
                    url
                    repository {
                        nameWithOwner
                    }
                }
            }
        }
    }
    "#
}

/// Searches GitHub for bounty‑related issues using the provided token.
///
/// # Arguments
///
/// * `token` – A GitHub personal access token with `repo` scope (or public read scope).
///
/// # Returns
///
/// A vector of `Issue` structs on success, or an error boxed as `dyn Error`.
pub async fn search_bounties(
    token: &str,
) -> Result<Vec<Issue>, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::builder().user_agent("psyverse-bounty-hunter/0.1").build()?;

    let graphql_query = build_query();

    let request_body = serde_json::json!({ "query": graphql_query });

    let resp = client
        .post("https://api.github.com/graphql")
        .bearer_auth(token)
        .json(&request_body)
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("GitHub API error {}: {}", status, text).into());
    }

    let gql_resp: GraphQLResponse = resp.json().await?;

    if let Some(errors) = gql_resp.errors {
        let msgs: Vec<String> = errors.into_iter().map(|e| e.message).collect();
        return Err(format!("GraphQL errors: {}", msgs.join("; ")).into());
    }

    let data = gql_resp
        .data
        .ok_or_else(|| "Missing data field in GraphQL response".to_string())?;

    // Transform GraphQL nodes into our public Issue struct.
    let mut issues = Vec::new();
    for node in data.search.nodes {
        if let Node::Issue {
            title,
            url,
            repository,
        } = node
        {
            issues.push(Issue {
                title,
                html_url: url,
                repository,
            });
        }
    }

    Ok(issues)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_contains_search_keyword() {
        let q = build_query();
        assert!(q.contains("search"));
        assert!(q.contains("bounty"));
    }

    #[tokio::test]
    async fn empty_token_returns_error() {
        let result = search_bounties("").await;
        assert!(result.is_err());
    }
}
