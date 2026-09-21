use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::error::Result;

/// Trait that all 13 agents must implement
#[async_trait]
pub trait Agent: Send + Sync {
    /// Unique agent identifier
    fn name(&self) -> &str;

    /// The pipeline stage this agent belongs to
    fn stage(&self) -> &str;

    /// Execute the agent's core logic
    async fn execute(&self, input: AgentInput) -> Result<AgentOutput>;

    /// Health check
    async fn health_check(&self) -> Result<bool>;

    /// Maximum execution time before timeout
    fn timeout(&self) -> Duration {
        Duration::from_secs(300)
    }
}

/// Standard input to any agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInput {
    pub run_id: String,
    pub payload: serde_json::Value,
    pub context: std::collections::HashMap<String, serde_json::Value>,
}

impl AgentInput {
    pub fn new(run_id: impl Into<String>, payload: serde_json::Value) -> Self {
        Self {
            run_id: run_id.into(),
            payload,
            context: std::collections::HashMap::new(),
        }
    }

    pub fn with_context(
        mut self,
        key: impl Into<String>,
        value: serde_json::Value,
    ) -> Self {
        self.context.insert(key.into(), value);
        self
    }
}

/// Standard output from any agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentOutput {
    pub run_id: String,
    pub agent_name: String,
    pub success: bool,
    pub result: serde_json::Value,
    pub metrics: AgentMetrics,
}

impl AgentOutput {
    pub fn success(run_id: &str, agent_name: &str, result: serde_json::Value) -> Self {
        Self {
            run_id: run_id.to_string(),
            agent_name: agent_name.to_string(),
            success: true,
            result,
            metrics: AgentMetrics::default(),
        }
    }

    pub fn failure(run_id: &str, agent_name: &str, error: &str) -> Self {
        Self {
            run_id: run_id.to_string(),
            agent_name: agent_name.to_string(),
            success: false,
            result: serde_json::json!({"error": error}),
            metrics: AgentMetrics::default(),
        }
    }
}

/// Performance metrics for an agent execution
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentMetrics {
    pub execution_time_ms: u64,
    pub api_calls: u32,
    pub tokens_used: u64,
    pub cost_usd: f64,
}
