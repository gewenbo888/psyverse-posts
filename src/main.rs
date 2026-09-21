//! Entry point for the Psyverse Bounty Hunter CLI.
//! It searches GitHub issues that contain bounty related keywords
//! and prints a short summary to stdout.
//!
//! The GitHub personal access token must be provided via the
//! `GITHUB_TOKEN` environment variable (or a `.env` file).

use dotenv::dotenv;
use std::env;
use std::process;

mod github;

#[tokio::main]
async fn main() {
    // Load .env if present and then read the token.
    dotenv().ok();
    let token = match env::var("GITHUB_TOKEN") {
        Ok(t) if !t.is_empty() => t,
        _ => {
            eprintln!("Error: GITHUB_TOKEN environment variable not set.");
            process::exit(1);
        }
    };

    match github::search_bounties(&token).await {
        Ok(issues) => {
            if issues.is_empty() {
                println!("No bounty issues found.");
            } else {
                for issue in issues {
                    println!(
                        "- [{}] {} ({})",
                        issue.repository.full_name,
                        issue.title,
                        issue.html_url
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch bounty issues: {}", e);
            process::exit(1);
        }
    }
}
