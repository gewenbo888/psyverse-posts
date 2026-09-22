//! MemoryBank — deduplication & anti-fatigue store.

use std::collections::HashMap;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub fingerprint: String,
    pub title: String,
    pub first_seen: String,
    pub last_seen: String,
    pub hit_count: u32,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryQuery {
    pub title: String,
    pub tags: Vec<String>,
}

/// In-memory (single-process) implementation. Swap for SQLite / Redis in
/// production by implementing the same trait.
#[derive(Debug, Default)]
pub struct MemoryStore {
    entries: Mutex<HashMap<String, MemoryEntry>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if the title has been seen before (duplicate).
    pub fn is_duplicate(&self, query: &MemoryQuery) -> bool {
        let fp = Self::fingerprint(query);
        let map = self.entries.lock().unwrap();
        map.contains_key(&fp)
    }

    /// Record a new (or refresh an existing) entry.
    pub fn record(&self, query: &MemoryQuery, now: &str) {
        let fp = Self::fingerprint(query);
        let mut map = self.entries.lock().unwrap();
        match map.get_mut(&fp) {
            Some(e) => {
                e.last_seen = now.to_string();
                e.hit_count += 1;
            }
            None => {
                map.insert(
                    fp,
                    MemoryEntry {
                        fingerprint: fp,
                        title: query.title.clone(),
                        first_seen: now.to_string(),
                        last_seen: now.to_string(),
                        hit_count: 1,
                        tags: query.tags.clone(),
                    },
                );
            }
        }
    }

    /// Fatigue score: 0 = fresh, 100 = extremely over-used.
    pub fn fatigue(&self, query: &MemoryQuery) -> u8 {
        let fp = Self::fingerprint(query);
        let map = self.entries.lock().unwrap();
        match map.get(&fp) {
            Some(e) => {
                // Simple heuristic: saturate at 100 after 10 hits.
                (e.hit_count as u8).saturating_mul(10).min(100)
            }
            None => 0,
        }
    }

    fn fingerprint(q: &MemoryQuery) -> String {
        let mut h = Sha256::new();
        h.update(q.title.to_lowercase().as_bytes());
        for t in &q.tags {
            h.update(t.to_lowercase().as_bytes());
        }
        hex::encode(h.finalize())
    }
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new()
    }
}
