use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for a content production run
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RunId(pub Uuid);

impl std::fmt::Display for RunId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for RunId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Represents a sourced topic/bounty opportunity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: Uuid,
    pub title: String,
    pub source: TopicSource,
    pub url: String,
    pub bounty_amount: Option<f64>,
    pub bounty_currency: Option<String>,
    pub tags: Vec<String>,
    pub discovered_at: DateTime<Utc>,
    pub priority: u8,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TopicSource {
    GitHubBounty,
    Algora,
    Trending,
    Manual,
}

/// A deduplicated topic after memory bank check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedTopic {
    pub topic: Topic,
    pub novelty_score: f32,
    pub fatigue_penalty: f32,
    pub final_score: f32,
}

/// Script output from ScriptSmith
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub run_id: RunId,
    pub title: String,
    pub hook: String,
    pub segments: Vec<ScriptSegment>,
    pub cta: String,
    pub estimated_duration_secs: u32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSegment {
    pub index: u32,
    pub narration: String,
    pub visual_direction: String,
    pub duration_secs: u32,
}

/// Rendered video frame data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedVideo {
    pub run_id: RunId,
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub duration_secs: u32,
    pub codec: String,
    pub file_size_bytes: u64,
}

/// Transcoded video ready for distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscodedVideo {
    pub run_id: RunId,
    pub source_path: String,
    pub output_path: String,
    pub target_resolution: String,
    pub target_fps: u32,
    pub format: String,
    pub file_size_bytes: u64,
}

/// Distribution target platform
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Bilibili,
    YouTube,
    Douyin,
    Xiaohongshu,
    Weibo,
    Twitter,
    Instagram,
    TikTok,
    Zhihu,
}

/// A distribution task for a specific platform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionTask {
    pub run_id: RunId,
    pub platform: Platform,
    pub matrix: MatrixType,
    pub video_path: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub scheduled_at: Option<DateTime<Utc>>,
}

/// Account matrix type for physical isolation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MatrixType {
    /// Business/tech content matrix
    Creator,
    /// Family/lifestyle content matrix
    Family,
}

/// Distribution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionResult {
    pub run_id: RunId,
    pub platform: Platform,
    pub matrix: MatrixType,
    pub success: bool,
    pub post_url: Option<String>,
    pub error: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
}

/// Revenue record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueRecord {
    pub run_id: RunId,
    pub source: RevenueSource,
    pub amount: f64,
    pub currency: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RevenueSource {
    WeChatTip,
    Web3Payment,
    PlatformRevenue,
    BountyPayment,
}

/// Pipeline state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineState {
    pub run_id: RunId,
    pub current_stage: PipelineStage,
    pub started_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub artifacts: HashMap<String, serde_json::Value>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PipelineStage {
    Sourcing,
    Production,
    Distribution,
    Cashflow,
    Complete,
    Failed,
}
