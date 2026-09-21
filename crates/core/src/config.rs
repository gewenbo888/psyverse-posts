use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Top-level configuration for the Psyverse Cash Matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsyverseConfig {
    pub sourcing: SourcingConfig,
    pub production: ProductionConfig,
    pub distribution: DistributionConfig,
    pub cashflow: CashflowConfig,
    pub pipeline: PipelineConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcingConfig {
    pub github_token: String,
    pub algora_api_key: String,
    pub scan_interval_secs: u64,
    pub min_bounty_usd: f64,
    pub max_bounty_usd: f64,
    pub target_repos: Vec<String>,
    pub memory_db_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionConfig {
    pub llm_provider: String,
    pub llm_api_key: String,
    pub llm_model: String,
    pub tts_provider: String,
    pub tts_api_key: String,
    pub render_output_dir: String,
    pub target_resolution: String,
    pub target_fps: u32,
    pub max_video_duration_secs: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionConfig {
    pub creator_matrix: MatrixConfig,
    pub family_matrix: MatrixConfig,
    pub platforms: Vec<PlatformConfig>,
    pub publish_interval_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixConfig {
    pub account_id: String,
    pub api_token: String,
    pub physical_isolation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConfig {
    pub name: String,
    pub api_endpoint: String,
    pub api_key: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashflowConfig {
    pub wechat_tip_qr_path: String,
    pub web3_wallet_address: String,
    pub revenue_db_path: String,
    pub dashboard_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    pub max_concurrent_runs: usize,
    pub retry_count: u32,
    pub stage_timeout_secs: u64,
    pub state_db_path: String,
}

impl PsyverseConfig {
    pub fn load_from_env() -> anyhow::Result<Self> {
        Ok(Self {
            sourcing: SourcingConfig {
                github_token: std::env::var("GITHUB_TOKEN")
                    .unwrap_or_else(|_| String::new()),
                algora_api_key: std::env::var("ALGORA_API_KEY")
                    .unwrap_or_else(|_| String::new()),
                scan_interval_secs: 300,
                min_bounty_usd: 50.0,
                max_bounty_usd: 1000.0,
                target_repos: vec![
                    "calcom/cal.com".into(),
                    "supabase/supabase".into(),
                    "langchain-ai/langchain".into(),
                ],
                memory_db_path: "./data/memory_bank.db".into(),
            },
            production: ProductionConfig {
                llm_provider: std::env::var("LLM_PROVIDER")
                    .unwrap_or_else(|_| "openai".into()),
                llm_api_key: std::env::var("LLM_API_KEY")
                    .unwrap_or_else(|_| String::new()),
                llm_model: std::env::var("LLM_MODEL")
                    .unwrap_or_else(|_| "gpt-4".into()),
                tts_provider: std::env::var("TTS_PROVIDER")
                    .unwrap_or_else(|_| "elevenlabs".into()),
                tts_api_key: std::env::var("TTS_API_KEY")
                    .unwrap_or_else(|_| String::new()),
                render_output_dir: "./output/videos".into(),
                target_resolution: "3840x2160".into(),
                target_fps: 60,
                max_video_duration_secs: 300,
            },
            distribution: DistributionConfig {
                creator_matrix: MatrixConfig {
                    account_id: std::env::var("CREATOR_ACCOUNT_ID")
                        .unwrap_or_else(|_| String::new()),
                    api_token: std::env::var("CREATOR_API_TOKEN")
                        .unwrap_or_else(|_| String::new()),
                    physical_isolation: true,
                },
                family_matrix: MatrixConfig {
                    account_id: std::env::var("FAMILY_ACCOUNT_ID")
                        .unwrap_or_else(|_| String::new()),
                    api_token: std::env::var("FAMILY_API_TOKEN")
                        .unwrap_or_else(|_| String::new()),
                    physical_isolation: true,
                },
                platforms: Vec::new(),
                publish_interval_secs: 3600,
            },
            cashflow: CashflowConfig {
                wechat_tip_qr_path: "./assets/wechat_tip_qr.png".into(),
                web3_wallet_address: std::env::var("WEB3_WALLET")
                    .unwrap_or_else(|_| String::new()),
                revenue_db_path: "./data/revenue.db".into(),
                dashboard_port: 8080,
            },
            pipeline: PipelineConfig {
                max_concurrent_runs: 4,
                retry_count: 3,
                stage_timeout_secs: 600,
                state_db_path: "./data/pipeline_state.db".into(),
            },
        })
    }
}
