//! history.rs
//!
//! 推理历史记录模块。

use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryRecord {
    pub id: u64,
    pub timestamp: u64,
    pub model: String,
    pub prompt: String,
    pub response: String,
    pub elapsed_ms: u64,
}

impl HistoryRecord {
    pub fn new(
        id: u64,
        model: impl Into<String>,
        prompt: impl Into<String>,
        response: impl Into<String>,
        elapsed_ms: u64,
    ) -> Self {
        Self {
            id,
            timestamp: current_unix_timestamp(),
            model: model.into(),
            prompt: prompt.into(),
            response: response.into(),
            elapsed_ms,
        }
    }
}

pub struct HistoryStore {
    path: PathBuf,
}

impl HistoryStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn append(&self, record: HistoryRecord) -> Result<()> {
        let mut records = self.load_all()?;
        records.push(record);
        self.save_all(&records)
    }

    pub fn list(&self, limit: usize) -> Result<Vec<HistoryRecord>> {
        let mut records = self.load_all()?;
        records.sort_by_key(|record| record.timestamp);
        records.reverse();

        if records.len() > limit {
            records.truncate(limit);
        }

        Ok(records)
    }

    fn load_all(&self) -> Result<Vec<HistoryRecord>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.path)
            .with_context(|| format!("failed to read history file: {}", self.path.display()))?;

        if content.trim().is_empty() {
            return Ok(Vec::new());
        }

        let records = serde_json::from_str::<Vec<HistoryRecord>>(&content)
            .with_context(|| format!("invalid history json: {}", self.path.display()))?;

        Ok(records)
    }

    fn save_all(&self, records: &[HistoryRecord]) -> Result<()> {
        ensure_parent_dir(&self.path)?;

        let json =
            serde_json::to_string_pretty(records).context("failed to serialize history records")?;

        fs::write(&self.path, json)
            .with_context(|| format!("failed to write history file: {}", self.path.display()))?;

        Ok(())
    }
}

fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory: {}", parent.display()))?;
    }

    Ok(())
}

fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}
