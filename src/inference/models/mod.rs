//! inference/models/mod.rs
//!
//! 推理模型模块

pub mod markov;
pub mod rule_based;

pub use markov::MarkovModel;
pub use rule_based::RuleBasedModel;
