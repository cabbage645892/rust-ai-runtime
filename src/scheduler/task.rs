use std::time::Instant;

/// 任务状态
#[derive(Debug, Clone)]
pub enum TaskStatus {
    Pending,
    Running,
    Finished,
    Failed,
}

/// 推理任务
#[derive(Debug, Clone)]
pub struct InferenceTask {
    pub id: u64,
    pub prompt: String,
    pub created_at: Instant,
}

impl InferenceTask {
    pub fn new(id: u64, prompt: impl Into<String>) -> Self {
        Self {
            id,
            prompt: prompt.into(),
            created_at: Instant::now(),
        }
    }
}

/// 推理结果
#[derive(Debug, Clone)]
pub struct TaskResult {
    pub task_id: u64,
    pub response: String,
    pub status: TaskStatus,
    pub elapsed_ms: u128,
}

impl TaskResult {
    pub fn success(task: &InferenceTask, response: String) -> Self {
        Self {
            task_id: task.id,
            response,
            status: TaskStatus::Finished,
            elapsed_ms: task.created_at.elapsed().as_millis(),
        }
    }

    pub fn failed(task: &InferenceTask, message: impl Into<String>) -> Self {
        Self {
            task_id: task.id,
            response: message.into(),
            status: TaskStatus::Failed,
            elapsed_ms: task.created_at.elapsed().as_millis(),
        }
    }
}
