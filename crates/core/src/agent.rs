//! Agent contract: every one of the 13 micro-agents implements [`Agent`].

use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Stable identifier for each of the 13 agents in the matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentId {
    // War line 1 — Sourcing
    TopicRadar,
    BountyHunter,
    MemoryBank,
    // War line 2 — Production
    ScriptSmith,
    SketchArtist,
    VoiceAlchemist,
    DeterministicRenderer,
    // War line 3 — Distribution
    VideoTranscoder,
    DualMatrixRouter,
    CreatorMatrix,
    FamilyMatrix,
    // War line 4 — Cashflow
    TrafficCashInjector,
    SovereignPay,
    RevenueDashboard,
}

impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::TopicRadar => "TopicRadar",
            Self::BountyHunter => "BountyHunter",
            Self::MemoryBank => "MemoryBank",
            Self::ScriptSmith => "ScriptSmith",
            Self::SketchArtist => "SketchArtist",
            Self::VoiceAlchemist => "VoiceAlchemist",
            Self::DeterministicRenderer => "DeterministicRenderer",
            Self::VideoTranscoder => "VideoTranscoder",
            Self::DualMatrixRouter => "DualMatrixRouter",
            Self::CreatorMatrix => "CreatorMatrix",
            Self::FamilyMatrix => "FamilyMatrix",
            Self::TrafficCashInjector => "TrafficCashInjector",
            Self::SovereignPay => "SovereignPay",
            Self::RevenueDashboard => "RevenueDashboard",
        };
        write!(f, "{name}")
    }
}

/// Errors that any agent may surface.
#[derive(Debug, Error)]
pub enum AgentError {
    #[error("upstream agent `{0}` failed: {1}")]
    Upstream(AgentId, String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("external service error: {0}")]
    External(String),
    #[error("rate limited, retry after {0}s")]
    RateLimited(u64),
    #[error("agent `{0}` not configured")]
    NotConfigured(AgentId),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result alias used across the pipeline.
pub type AgentResult<T> = Result<T, AgentError>;

/// Immutable context handed to every agent invocation.
#[derive(Debug, Clone)]
pub struct AgentContext {
    /// Unique run id for this pipeline execution.
    pub run_id: Uuid,
    /// The agent that is about to execute.
    pub agent: AgentId,
    /// Monotonic step index within the run (0-based).
    pub step: u32,
    /// Free-form key/value metadata (e.g. `github_token`, `ffmpeg_path`).
    pub metadata: std::collections::BTreeMap<String, String>,
}

impl AgentContext {
    pub fn new(agent: AgentId, step: u32) -> Self {
        Self {
            run_id: Uuid::new_v4(),
            agent,
            step,
            metadata: std::collections::BTreeMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Opaque output envelope. Each agent serialises its domain payload into
/// `payload` so the orchestrator can forward it without knowing the shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentOutput {
    pub agent: AgentId,
    pub run_id: Uuid,
    pub step: u32,
    pub payload: serde_json::Value,
    pub elapsed_ms: u64,
}

/// The single trait every micro-agent must implement.
pub trait Agent: Send + Sync {
    fn id(&self) -> AgentId;
    fn run(&self, ctx: &AgentContext, input: &serde_json::Value) -> AgentResult<AgentOutput>;
}
