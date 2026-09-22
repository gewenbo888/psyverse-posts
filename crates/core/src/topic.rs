//! Topic / hot-keyword domain model (TopicRadar output).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: String,
    pub title: String,
    pub source: String,
    pub url: String,
    pub score: TopicScore,
    pub tags: Vec<String>,
    pub discovered_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TopicScore {
    /// 0..=100 composite heat score.
    pub heat: u8,
    /// 0..=100 novelty (inverse of fatigue).
    pub novelty: u8,
    /// 0..=100 monetisation potential.
    pub monetization: u8,
}

impl TopicScore {
    pub fn composite(&self) -> f64 {
        // Weighted: heat 40 %, novelty 30 %, monetization 30 %.
        self.heat as f64 * 0.4 + self.novelty as f64 * 0.3 + self.monetization as f64 * 0.3
    }
}
