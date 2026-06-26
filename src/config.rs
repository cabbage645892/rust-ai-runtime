//! config.rs
//!
//! 轻量级配置读取模块。
//! 为了减少外部依赖，这里实现一个简单的 key = value 配置解析器。

use std::{fs, path::Path};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub rules_path: String,
    pub markov_corpus_path: String,
    pub history_path: String,
    pub worker_count: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            rules_path: "assets/models/rules.json".to_string(),
            markov_corpus_path: "assets/models/markov_corpus.txt".to_string(),
            history_path: "history/history.json".to_string(),
            worker_count: 4,
        }
    }
}

impl AppConfig {
    pub fn load(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();

        let Ok(content) = fs::read_to_string(path) else {
            return Self::default();
        };

        let mut config = Self::default();

        for raw_line in content.lines() {
            let line = raw_line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let Some((key, value)) = line.split_once('=') else {
                continue;
            };

            let key = key.trim();
            let value = value.trim().trim_matches('"');

            match key {
                "rules_path" => config.rules_path = value.to_string(),
                "markov_corpus_path" => config.markov_corpus_path = value.to_string(),
                "history_path" => config.history_path = value.to_string(),
                "worker_count" => {
                    if let Ok(n) = value.parse::<usize>() {
                        config.worker_count = n.max(1);
                    }
                }
                _ => {}
            }
        }

        config
    }
}
