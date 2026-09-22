//! TopicRadar — scans trending sources and scores topics.

use std::time::Instant;

use psyverse_core::agent::{Agent, AgentContext, AgentError, AgentId, AgentOutput, AgentResult};
use psyverse_core::topic::{Topic, TopicScore};
use serde_json::json;

pub struct TopicRadarAgent {
    /// Sources to poll (e.g. Zhihu hot, Weibo, GitHub trending).
    sources: Vec<String>,
}

impl TopicRadarAgent {
    pub fn new(sources: Vec<String>) -> Self {
        Self { sources }
    }
}

impl Default for TopicRadarAgent {
    fn default() -> Self {
        Self::new(vec![
            "zhihu_hot".into(),
            "weibo_hot".into(),
            "github_trending".into(),
        ])
    }
}

impl Agent for TopicRadarAgent {
    fn id(&self) -> AgentId {
        AgentId::TopicRadar
    }

    fn run(&self, ctx: &AgentContext, input: &serde_json::Value) -> AgentResult<AgentOutput> {
        let start = Instant::now();

        // In production this would fan-out HTTP requests to each source.
        // Here we demonstrate the scoring pipeline with a deterministic
        // stub so the rest of the pipeline can be exercised offline.
        let max_topics: u32 = input
            .get("max_topics")
            .and_then(|v| v.as_u64())
            .unwrap_or(10) as u32;

        let topics: Vec<Topic> = (0..max_topics)
            .map(|i| Topic {
                id: format!("topic-{i:04}"),
                title: format!("Trending topic #{i}"),
                source: self.sources.first().cloned().unwrap_or_default(),
                url: format!("https://example.com/hot/{i}"),
                score: TopicScore {
                    heat: (100 - (i as u8 * 7) % 50).min(100),
                    novelty: (80 - (i as u8 * 5) % 40).min(100),
                    monetization: (90 - (i as u8 * 3) % 30).min(100),
                },
                tags: vec!["ai".into(), "tech".into()],
                discovered_at: chrono::Utc::now().to_rfc3339(),
            })
            .collect();

        // Sort by composite score descending.
        let mut topics = topics;
        topics.sort_by(|a, b| b.score.composite().partial_cmp(&a.score.composite()).unwrap());

        let payload = json!({ "topics": topics });
        Ok(AgentOutput {
            agent: self.id(),
            run_id: ctx.run_id,
            step: ctx.step,
            payload,
            elapsed_ms: start.elapsed().as_millis() as u64,
        })
    }
}
