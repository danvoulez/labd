//! `logline-lab-local` — the local Act-log (dev-ephemeral durable store) and
//! candidate capture.
//!
//! Honest naming (RELEASE_SCOPE §1, `docs/STORAGE.md`): this is **not** "the
//! truth" and **not** a transport outbox to an external spine. Under the
//! `dev-ephemeral` profile it is a **local append-only Act-log used as a
//! development spine** — durable for local development, but explicitly
//! *unregistered and non-publication-grade*. A future `TransportOutbox` (for
//! shipping to an external spine) is a separate concern and does not exist yet.
//!
//! Acts are appended here and deduplicated by content hash. The `replayed` flag
//! records whether an entry has been replayed into the in-process query spine; it
//! lives *around* the Act, never inside it.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use logline_act::Act;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LocalError {
    #[error("act hashing failed: {0}")]
    Hash(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("json error: {0}")]
    Json(String),
}

/// One entry in the local Act-log.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActLogEntry {
    pub content_hash: String,
    pub act: Act,
    /// Whether this entry has been replayed into the in-process query spine.
    /// (Reserved to also mean "shipped" once a `TransportOutbox` to an external
    /// spine exists.) State is *around* the Act, never inside it.
    #[serde(alias = "synced")]
    pub replayed: bool,
}

/// Result of appending an Act to the local Act-log.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EmitOutcome {
    /// A new entry was stored.
    Stored(String),
    /// The same content hash already existed; nothing was duplicated.
    Duplicate(String),
}

impl EmitOutcome {
    pub fn content_hash(&self) -> &str {
        match self {
            EmitOutcome::Stored(h) | EmitOutcome::Duplicate(h) => h,
        }
    }
    pub fn is_duplicate(&self) -> bool {
        matches!(self, EmitOutcome::Duplicate(_))
    }
}

/// The local append-only Act-log. Optionally file-backed for durability across
/// runs. Under `dev-ephemeral` this is the development spine's durable record —
/// honest, but non-publication-grade.
#[derive(Debug, Default)]
pub struct LocalActLog {
    entries: BTreeMap<String, ActLogEntry>,
    path: Option<PathBuf>,
}

impl LocalActLog {
    /// An in-memory-only Act-log (no durability).
    pub fn in_memory() -> Self {
        Self::default()
    }

    /// Open (or create) a file-backed Act-log at `path`, loading prior entries.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, LocalError> {
        let path = path.as_ref().to_path_buf();
        let mut entries = BTreeMap::new();
        if path.exists() {
            let text = std::fs::read_to_string(&path).map_err(|e| LocalError::Io(e.to_string()))?;
            for line in text.lines().filter(|l| !l.trim().is_empty()) {
                let entry: ActLogEntry =
                    serde_json::from_str(line).map_err(|e| LocalError::Json(e.to_string()))?;
                entries.insert(entry.content_hash.clone(), entry);
            }
        }
        Ok(Self {
            entries,
            path: Some(path),
        })
    }

    /// Append an Act to the log. Idempotent by content hash: appending the same
    /// Act twice keeps a single entry.
    pub fn emit(&mut self, act: &Act) -> Result<EmitOutcome, LocalError> {
        let hash = act.content_hash().map_err(|e| LocalError::Hash(e.to_string()))?;
        if self.entries.contains_key(&hash) {
            return Ok(EmitOutcome::Duplicate(hash));
        }
        self.entries.insert(
            hash.clone(),
            ActLogEntry {
                content_hash: hash.clone(),
                act: act.clone(),
                replayed: false,
            },
        );
        self.persist()?;
        Ok(EmitOutcome::Stored(hash))
    }

    /// All entries, ordered by content hash.
    pub fn list(&self) -> Vec<&ActLogEntry> {
        self.entries.values().collect()
    }

    /// Entries not yet replayed into the query spine.
    pub fn unreplayed(&self) -> Vec<&ActLogEntry> {
        self.entries.values().filter(|e| !e.replayed).collect()
    }

    /// Mark an entry replayed after the spine has it.
    pub fn mark_replayed(&mut self, content_hash: &str) -> Result<(), LocalError> {
        if let Some(entry) = self.entries.get_mut(content_hash) {
            entry.replayed = true;
        }
        self.persist()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get(&self, content_hash: &str) -> Option<&ActLogEntry> {
        self.entries.get(content_hash)
    }

    fn persist(&self) -> Result<(), LocalError> {
        let Some(path) = &self.path else {
            return Ok(());
        };
        let mut buf = String::new();
        for entry in self.entries.values() {
            let line = serde_json::to_string(entry).map_err(|e| LocalError::Json(e.to_string()))?;
            buf.push_str(&line);
            buf.push('\n');
        }
        std::fs::write(path, buf).map_err(|e| LocalError::Io(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample_act(did: &str) -> Act {
        Act::new(
            json!("lab.operator"),
            json!(did),
            json!({"k": "v"}),
            json!("2026-06-06T00:00:00Z"),
            json!("none"),
            json!("record"),
            json!("carry_blocked"),
            json!("reject"),
            json!("candidate"),
        )
    }

    /// Local append stores an Act durably in the dev-ephemeral Act-log.
    #[test]
    fn local_append_stores_durably() {
        let dir = std::env::temp_dir().join(format!("llk-actlog-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("a6.jsonl");
        let _ = std::fs::remove_file(&path);

        let mut log = LocalActLog::open(&path).unwrap();
        let outcome = log.emit(&sample_act("declare_lab")).unwrap();
        assert!(matches!(outcome, EmitOutcome::Stored(_)));
        assert_eq!(log.len(), 1);

        // Durability: reopen and the entry is still there.
        let reopened = LocalActLog::open(&path).unwrap();
        assert_eq!(reopened.len(), 1);
        assert!(reopened.get(outcome.content_hash()).is_some());
        let _ = std::fs::remove_file(&path);
    }

    /// Re-appending the same Act is idempotent.
    #[test]
    fn reappend_is_idempotent() {
        let mut log = LocalActLog::in_memory();
        let act = sample_act("declare_lab");
        let first = log.emit(&act).unwrap();
        let second = log.emit(&act).unwrap();
        assert!(matches!(first, EmitOutcome::Stored(_)));
        assert!(second.is_duplicate());
        assert_eq!(first.content_hash(), second.content_hash());
        assert_eq!(log.len(), 1);

        log.emit(&sample_act("declare_other")).unwrap();
        assert_eq!(log.len(), 2);
    }
}
