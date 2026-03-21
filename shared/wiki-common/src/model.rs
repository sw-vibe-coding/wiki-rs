use serde::{Deserialize, Serialize};

/// A wiki page with content and timestamps.
///
/// Timestamps are Unix seconds (0 = unknown/legacy).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WikiPage {
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub created_at: u64,
    #[serde(default)]
    pub updated_at: u64,
}

impl WikiPage {
    pub fn new(title: &str, content: &str, now: u64) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}
