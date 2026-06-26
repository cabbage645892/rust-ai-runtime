//! worker.rs
//!
//! Worker：负责执行推理任务。

use std::sync::Arc;

use crate::tokenizer::Tokenizer;

use anyhow::Result;

use crate::{
    inference::{InferenceContext, InferenceEngine},
    tokenizer::WordTokenizer,
};

use super::{InferenceTask, TaskResult};

pub struct Worker {
    id: usize,
    engine: Arc<InferenceEngine>,
}

impl Worker {
    /// 创建 Worker
    pub fn new(id: usize, engine: Arc<InferenceEngine>) -> Self {
        Self { id, engine }
    }

    /// Worker ID
    pub fn id(&self) -> usize {
        self.id
    }

    /// 处理一个推理任务
    pub async fn process(&self, task: InferenceTask) -> Result<()> {
        println!("[Worker {}] Processing Task {}", self.id, task.id);

        // Tokenize
        let mut tokenizer = WordTokenizer::new();

        let tokens = tokenizer.encode(&task.prompt);
        // 构造推理上下文
        let context = InferenceContext::new(task.prompt.clone(), tokens);

        // 调用推理引擎
        let inference = self.engine.infer(&context)?;

        // 构造任务结果
        let result = TaskResult::success(&task, inference.response);

        println!("[Worker {}] Finished Task {}", self.id, result.task_id,);

        println!("[Worker {}] Response:", self.id,);

        println!("{}", result.response);

        println!("[Worker {}] Time: {} ms", self.id, result.elapsed_ms,);

        Ok(())
    }
}
