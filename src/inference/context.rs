//! context.rs
//!
//! 推理上下文

/// 推理输入
#[derive(Debug, Clone)]
pub struct InferenceContext {
    /// 原始输入文本
    pub prompt: String,

    /// Tokenizer编码后的Token
    pub tokens: Vec<u32>,
}

impl InferenceContext {
    pub fn new(prompt: String, tokens: Vec<u32>) -> Self {
        Self { prompt, tokens }
    }
}

/// 推理输出
#[derive(Debug, Clone)]
pub struct InferenceResult {
    /// 输出文本
    pub response: String,

    /// 输出Token
    pub tokens: Vec<u32>,
}

impl InferenceResult {
    pub fn new(response: String, tokens: Vec<u32>) -> Self {
        Self { response, tokens }
    }
}
