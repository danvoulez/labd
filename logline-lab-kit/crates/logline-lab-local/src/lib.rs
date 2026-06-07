//! `logline-lab-local` — the local outbox / cache.
//!
//! This is a **provisional cache**, not a source of truth (Operator §13: files
//! are not semantic truth; SQLite is not the spine). Acts are emitted here first,
//! deduplicated by content hash, and later synced to a spine. The local store is
//! a JSON-lines file purely so a Lab can work offline and resume; it never
//! claims to be the official record.

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

/// One entry in the local outbox.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutboxEntry {
    pub content_hash: String,
    pub act: Act,
    /// Whether this entry has been synced to a spine. Sync state is *around* the
    /// Act, never inside it.
    pub synced: bool,
}

/// Result of emitting an Act into the outbox.
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

/// Local outbox/cache. Optionally file-backed for durability across runs.
#[derive(Debug, Default)]
pub struct LocalOutbox {
    entries: BTreeMap<String, OutboxEntry>,
    path: Option<PathBuf>,
}

impl LocalOutbox {
    /// An in-memory-only outbox (no durability).
    pub fn in_memory() -> Self {
        Self::default()
    }

    /// Open (or create) a file-backed outbox at `path`, loading any prior entries.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, LocalError> {
        let path = path.as_ref().to_path_buf();
        let mut entries = BTreeMap::new();
        if path.exists() {
            let text = std::fs::read_to_string(&path).map_err(|e| LocalError::Io(e.to_string()))?;
            for line in text.lines().filter(|l| !l.trim().is_empty()) {
                let entry: OutboxEntry =
                    serde_json::from_str(line).map_err(|e| LocalError::Json(e.to_string()))?;
                entries.insert(entry.content_hash.clone(), entry);
            }
        }
        Ok(Self {
            entries,
            path: Some(path),
        })
    }

    /// Emit an Act into the outbox. Idempotent by content hash (A6/A7): emitting
    /// the same Act twice stores a single entry.
    pub fn emit(&mut self, act: &Act) -> Result<EmitOutcome, LocalError> {
        let hash = act.content_hash().map_err(|e| LocalError::Hash(e.to_string()))?;
        if self.entries.contains_key(&hash) {
            return Ok(EmitOutcome::Duplicate(hash));
        }
        self.entries.insert(
            hash.clone(),
            OutboxEntry {
                content_hash: hash.clone(),
                act: act.clone(),
                synced: false,
            },
        );
        self.persist()?;
        Ok(EmitOutcome::Stored(hash))
    }

    /// All entries, ordered by content hash.
    pub fn list(&self) -> Vec<&OutboxEntry> {
        self.entries.values().collect()
    }

    /// Entries not yet synced to a spine.
    pub fn unsynced(&self) -> Vec<&OutboxEntry> {
        self.entries.values().filter(|e| !e.synced).collect()
    }

    /// Mark an entry synced after a spine accepts it.
    pub fn mark_synced(&mut self, content_hash: &str) -> Result<(), LocalError> {
        if let Some(entry) = self.entries.get_mut(content_hash) {
            entry.synced = true;
        }
        self.persist()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get(&self, content_hash: &str) -> Option<&OutboxEntry> {
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

    /// A6 — Local emit stores Act in local cache/outbox (with durability).
    #[test]
    fn a6_local_emit_stores() {
        let dir = std::env::temp_dir().join(format!("llk-outbox-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("a6.jsonl");
        let _ = std::fs::remove_file(&path);

        let mut outbox = LocalOutbox::open(&path).unwrap();
        let outcome = outbox.emit(&sample_act("declare_lab")).unwrap();
        assert!(matches!(outcome, EmitOutcome::Stored(_)));
        assert_eq!(outbox.len(), 1);

        // Durability: reopen and the entry is still there.
        let reopened = LocalOutbox::open(&path).unwrap();
        assert_eq!(reopened.len(), 1);
        assert!(reopened.get(outcome.content_hash()).is_some());
        let _ = std::fs::remove_file(&path);
    }

    /// A7 — Re-emitting same Act is idempotent.
    #[test]
    fn a7_reemit_idempotent() {
        let mut outbox = LocalOutbox::in_memory();
        let act = sample_act("declare_lab");
        let first = outbox.emit(&act).unwrap();
        let second = outbox.emit(&act).unwrap();
        assert!(matches!(first, EmitOutcome::Stored(_)));
        assert!(second.is_duplicate());
        assert_eq!(first.content_hash(), second.content_hash());
        assert_eq!(outbox.len(), 1);

        // A genuinely different Act is a separate entry.
        outbox.emit(&sample_act("declare_other")).unwrap();
        assert_eq!(outbox.len(), 2);
    }
}
