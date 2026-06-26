//! traits.rs
//!
//! 推理模型 Trait

use anyhow::Result;

use super::context::{InferenceContext, InferenceResult};

/// 所有推理模型必须实现的 Trait
pub trait InferenceModel: Send + Sync {
    /// 返回模型名称
    fn name(&self) -> &str;

    /// 执行推理
    fn infer(&self, context: &InferenceContext) -> Result<InferenceResult>;
}
