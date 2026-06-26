//! markov.rs
//!
//! Markov 推理模型
//!
//! 从文本语料自动构建状态转移表。

use std::{collections::HashMap, fs};

use anyhow::{Context, Result};
use rand::prelude::IndexedRandom;

use crate::inference::{
    context::{InferenceContext, InferenceResult},
    traits::InferenceModel,
};

pub struct MarkovModel {
    transitions: HashMap<String, Vec<String>>,
}

impl MarkovModel {
    /// 从语料加载并构建状态转移表
    pub fn load(path: &str) -> Result<Self> {
        let text = fs::read_to_string(path).with_context(|| format!("Failed to read {}", path))?;

        let mut transitions: HashMap<String, Vec<String>> = HashMap::new();

        for line in text.lines() {
            let words: Vec<String> = line
                .split_whitespace()
                .map(|s| {
                    s.trim_matches(|c: char| !c.is_alphanumeric())
                        .to_lowercase()
                })
                .filter(|s| !s.is_empty())
                .collect();

            if words.len() < 2 {
                continue;
            }

            for pair in words.windows(2) {
                let current = pair[0].clone();
                let next = pair[1].clone();

                transitions.entry(current).or_default().push(next);
            }
        }

        Ok(Self { transitions })
    }

    /// 状态数量
    pub fn state_count(&self) -> usize {
        self.transitions.len()
    }

    /// 随机选择下一个词
    fn next_word(&self, current: &str) -> Option<String> {
        let mut rng = rand::rng();

        self.transitions
            .get(current)
            .and_then(|list| list.choose(&mut rng))
            .cloned()
    }
}

impl InferenceModel for MarkovModel {
    fn name(&self) -> &str {
        "MarkovModel"
    }

    fn infer(&self, context: &InferenceContext) -> Result<InferenceResult> {
        let mut words: Vec<String> = context
            .prompt
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect();

        const MAX_GENERATE: usize = 15;

        for _ in 0..MAX_GENERATE {
            let Some(last) = words.last().cloned() else {
                break;
            };

            let Some(next) = self.next_word(&last) else {
                break;
            };

            words.push(next);
        }

        let response = words.join(" ");

        Ok(InferenceResult::new(response, context.tokens.clone()))
    }
}
