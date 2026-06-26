use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub version: String,
    pub vocab_size: usize,
    pub layers: usize,
}

pub struct ModelManager {
    current_model: Option<ModelInfo>,
}

impl ModelManager {
    pub fn new() -> Self {
        Self {
            current_model: None,
        }
    }

    pub fn load_model(&mut self, path: &str) -> Result<()> {
        let content = fs::read_to_string(path)?;

        let model: ModelInfo = serde_json::from_str(&content)?;

        self.current_model = Some(model);

        Ok(())
    }

    pub fn get_model(&self) -> Option<&ModelInfo> {
        self.current_model.as_ref()
    }

    pub fn is_loaded(&self) -> bool {
        self.current_model.is_some()
    }
}
impl Default for ModelManager {
    fn default() -> Self {
        Self::new()
    }
}
