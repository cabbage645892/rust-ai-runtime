use std::sync::Arc;

use anyhow::{Result, bail};

use crate::inference::{
    context::{InferenceContext, InferenceResult},
    traits::InferenceModel,
};

pub struct InferenceEngine {
    models: Vec<Arc<dyn InferenceModel>>,
    current: usize,
}

impl InferenceEngine {
    pub fn new() -> Self {
        Self {
            models: Vec::new(),
            current: 0,
        }
    }

    pub fn add_model(&mut self, model: Arc<dyn InferenceModel>) {
        self.models.push(model);
    }

    pub fn set_model(&mut self, index: usize) -> Result<()> {
        if index >= self.models.len() {
            bail!(
                "model index {} out of range, available model count: {}",
                index,
                self.models.len()
            );
        }

        self.current = index;
        Ok(())
    }

    pub fn set_model_by_name(&mut self, name: &str) -> Result<()> {
        let Some(index) = self
            .models
            .iter()
            .position(|model| model.name().eq_ignore_ascii_case(name))
        else {
            bail!("model not found: {name}");
        };

        self.current = index;
        Ok(())
    }

    pub fn current_model_name(&self) -> &str {
        self.models
            .get(self.current)
            .map(|model| model.name())
            .unwrap_or("<no model>")
    }

    pub fn model_names(&self) -> Vec<String> {
        self.models
            .iter()
            .map(|model| model.name().to_string())
            .collect()
    }

    pub fn infer(&self, context: &InferenceContext) -> Result<InferenceResult> {
        let Some(model) = self.models.get(self.current) else {
            bail!("no model has been loaded");
        };

        model.infer(context)
    }
}

impl Default for InferenceEngine {
    fn default() -> Self {
        Self::new()
    }
}
