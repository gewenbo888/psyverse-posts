use thiserror::Error;

#[derive(Debug, Error)]
pub enum PsyverseError {
    #[error("Agent '{name}' failed: {reason}")]
    AgentFailed { name: String, reason: String },

    #[error("Pipeline stage '{stage}' failed: {reason}")]
    PipelineStageFailed { stage: String, reason: String },

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("HTTP request failed: {0}")]
    HttpError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("External service '{service}' unavailable: {reason}")]
    ServiceUnavailable { service: String, reason: String },

    #[error("Timeout after {seconds}s waiting for {operation}")]
    Timeout { seconds: u64, operation: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, PsyverseError>;
