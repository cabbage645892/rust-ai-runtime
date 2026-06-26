//! rule_based.rs
//!
//! Rule Based Model
//!
//! 从 JSON 文件加载规则，而不是写死在代码中。

use std::{collections::HashMap, fs};

use anyhow::{Context, Result};

use crate::inference::{
    context::{InferenceContext, InferenceResult},
    traits::InferenceModel,
};

pub struct RuleBasedModel {
    rules: HashMap<String, String>,
}

impl RuleBasedModel {
    /// 从 JSON 文件加载规则
    pub fn load(path: &str) -> Result<Self> {
        let content =
            fs::read_to_string(path).with_context(|| format!("Failed to read {}", path))?;

        let rules: HashMap<String, String> =
            serde_json::from_str(&content).context("Invalid rules.json format")?;

        Ok(Self { rules })
    }

    /// 返回规则数量
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

impl InferenceModel for RuleBasedModel {
    fn name(&self) -> &str {
        "RuleBasedModel"
    }

    fn infer(&self, context: &InferenceContext) -> Result<InferenceResult> {
        let input = context.prompt.to_lowercase();

        for (keyword, response) in &self.rules {
            if input.contains(keyword) {
                return Ok(InferenceResult::new(
                    response.clone(),
                    context.tokens.clone(),
                ));
            }
        }

        Ok(InferenceResult::new(
            "Sorry, I don't understand.".to_string(),
            context.tokens.clone(),
        ))
    }
}
