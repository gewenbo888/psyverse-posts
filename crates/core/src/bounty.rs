//! GitHub / Algora bounty domain model (BountyHunter output).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bounty {
    pub id: String,
    pub source: BountySource,
    pub repo: String,
    pub issue_number: u32,
    pub title: String,
    pub body: String,
    pub amount_usd: f64,
    pub currency: String,
    pub status: BountyStatus,
    pub labels: Vec<String>,
    /// ISO-8601 timestamp the bounty was posted.
    pub created_at: String,
    pub url: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BountySource {
    GitHub,
    Algora,
    Polar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BountyStatus {
    Open,
    InProgress,
    Paid,
    Expired,
}
