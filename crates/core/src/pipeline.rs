use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::agent::{Agent, AgentInput, AgentOutput};
use crate::error::{PsyverseError, Result};
use crate::types::{PipelineStage, PipelineState, RunId};

/// Message passed between pipeline stages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelineMessage {
    StageComplete {
        run_id: RunId,
        stage: PipelineStage,
        output: AgentOutput,
    },
    StageFailed {
        run_id: RunId,
        stage: PipelineStage,
        error: String,
    },
    PipelineComplete {
        run_id: RunId,
        final_state: PipelineState,
    },
    PipelineFailed {
        run_id: RunId,
        error: String,
    },
}

/// Configuration for a single pipeline stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageConfig {
    pub name: String,
    pub stage: PipelineStage,
    pub agent_names: Vec<String>,
    pub parallel: bool,
    pub retry_count: u32,
    pub timeout_secs: u64,
}

/// The main pipeline orchestrator
pub struct Pipeline {
    stages: Vec<StageConfig>,
    agents: HashMap<String, Arc<dyn Agent>>,
    tx: mpsc::Sender<PipelineMessage>,
    rx: mpsc::Receiver<PipelineMessage>,
}

impl Pipeline {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(64);
        Self {
            stages: Vec::new(),
            agents: HashMap::new(),
            tx,
            rx,
        }
    }

    pub fn add_stage(mut self, config: StageConfig) -> Self {
        self.stages.push(config);
        self
    }

    pub fn register_agent(mut self, agent: Arc<dyn Agent>) -> Self {
        let name = agent.name().to_string();
        info!(agent = %name, "Registering agent");
        self.agents.insert(name, agent);
        self
    }

    /// Execute the full pipeline for a given run
    pub async fn execute(&self, run_id: RunId) -> Result<PipelineState> {
        let mut state = PipelineState {
            run_id: run_id.clone(),
            current_stage: PipelineStage::Sourcing,
            started_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            artifacts: HashMap::new(),
            errors: Vec::new(),
        };

        info!(run_id = %run_id, "Starting pipeline execution");

        for stage_config in &self.stages {
            state.current_stage = stage_config.stage.clone();
            state.updated_at = chrono::Utc::now();

            info!(
                stage = %stage_config.name,
                agents = ?stage_config.agent_names,
                "Executing pipeline stage"
            );

            let stage_result = self.execute_stage(&run_id, stage_config).await;

            match stage_result {
                Ok(outputs) => {
                    for output in &outputs {
                        state
                            .artifacts
                            .insert(output.agent_name.clone(), output.result.clone());
                    }
                }
                Err(e) => {
                    error!(stage = %stage_config.name, error = %e, "Stage failed");
                    state.errors.push(format!("{}: {}", stage_config.name, e));
                    state.current_stage = PipelineStage::Failed;
                    state.updated_at = chrono::Utc::now();
                    return Ok(state);
                }
            }
        }

        state.current_stage = PipelineStage::Complete;
        state.updated_at = chrono::Utc::now();
        info!(run_id = %run_id, "Pipeline completed successfully");

        Ok(state)
    }

    async fn execute_stage(
        &self,
        run_id: &RunId,
        config: &StageConfig,
    ) -> Result<Vec<AgentOutput>> {
        let mut outputs = Vec::new();

        if config.parallel {
            // Execute agents in parallel
            let mut handles = Vec::new();
            for agent_name in &config.agent_names {
                let agent = self
                    .agents
                    .get(agent_name)
                    .ok_or_else(|| {
                        PsyverseError::AgentFailed {
                            name: agent_name.clone(),
                            reason: "Agent not registered".into(),
                        }
                    })?
                    .clone();
                let run_id_clone = run_id.clone();
                handles.push(tokio::spawn(async move {
                    let input = AgentInput::new(
                        run_id_clone.to_string(),
                        serde_json::json!({}),
                    );
                    agent.execute(input).await
                }));
            }

            for handle in handles {
                match handle.await {
                    Ok(Ok(output)) => outputs.push(output),
                    Ok(Err(e)) => {
                        return Err(PsyverseError::PipelineStageFailed {
                            stage: config.name.clone(),
                            reason: e.to_string(),
                        });
                    }
                    Err(e) => {
                        return Err(PsyverseError::PipelineStageFailed {
                            stage: config.name.clone(),
                            reason: format!("Task join error: {}", e),
                        });
                    }
                }
            }
        } else {
            // Execute agents sequentially
            for agent_name in &config.agent_names {
                let agent = self
                    .agents
                    .get(agent_name)
                    .ok_or_else(|| {
                        PsyverseError::AgentFailed {
                            name: agent_name.clone(),
                            reason: "Agent not registered".into(),
                        }
                    })?
                    .clone();

                let input = AgentInput::new(
                    run_id.to_string(),
                    serde_json::json!({}),
                );

                let output = agent.execute(input).await?;
                outputs.push(output);
            }
        }

        Ok(outputs)
    }

    /// Subscribe to pipeline messages
    pub fn subscribe(&self) -> mpsc::Receiver<PipelineMessage> {
        // In production, use a broadcast channel
        let (tx, rx) = mpsc::channel(64);
        let _ = tx; // Drop the sender, we just want the receiver pattern
        rx
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}
