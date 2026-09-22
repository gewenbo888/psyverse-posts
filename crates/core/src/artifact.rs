//! Media artifact model (renderer / transcoder output).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: String,
    pub kind: ArtifactKind,
    pub path: String,
    pub spec: MediaSpec,
    pub size_bytes: u64,
    pub checksum_sha256: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactKind {
    Storyboard,
    Audio,
    Video,
    TranscodedVideo,
    Thumbnail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaSpec {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub codec: String,
    pub container: String,
    pub bitrate_kbps: u32,
}
