//! Psyverse Cash Matrix — Core domain types and agent contracts.
//!
//! This crate defines the shared vocabulary for the 13-agent pipeline:
//! topics, bounties, scripts, storyboards, media artifacts, distribution
//! targets, and revenue events.

pub mod agent;
pub mod artifact;
pub mod bounty;
pub mod distribution;
pub mod memory;
pub mod revenue;
pub mod script;
pub mod topic;

pub use agent::{Agent, AgentContext, AgentError, AgentId, AgentOutput, AgentResult};
pub use artifact::{Artifact, ArtifactKind, MediaSpec};
pub use bounty::{Bounty, BountySource, BountyStatus};
pub use distribution::{DistributionTarget, MatrixKind, Platform};
pub use memory::{MemoryEntry, MemoryQuery, MemoryStore};
pub use revenue::{RevenueEvent, RevenueSource};
pub use script::{Script, ScriptSegment, HookType};
pub use topic::{Topic, TopicScore};
