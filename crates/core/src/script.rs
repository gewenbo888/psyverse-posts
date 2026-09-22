//! Script / storyboard domain model (ScriptSmith + SketchArtist output).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub id: String,
    pub topic_id: String,
    pub title: String,
    pub hook: String,
    pub hook_type: HookType,
    pub segments: Vec<ScriptSegment>,
    pub cta: String,
    pub estimated_duration_sec: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookType {
    Question,
    Shock,
    Story,
    Data,
    Challenge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSegment {
    pub index: u32,
    pub narration: String,
    pub visual_direction: String,
    pub duration_sec: u32,
}
