//! Distribution matrix model (DualMatrixRouter / CreatorMatrix / FamilyMatrix).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Bilibili,
    Douyin,
    Xiaohongshu,
    Weibo,
    YouTube,
    Twitter,
    TikTok,
    Instagram,
    Zhihu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatrixKind {
    /// Commercial / tech audience.
    Creator,
    /// Family / lifestyle audience (physically isolated accounts).
    Family,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionTarget {
    pub platform: Platform,
    pub matrix: MatrixKind,
    pub account_id: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub artifact_id: String,
}
