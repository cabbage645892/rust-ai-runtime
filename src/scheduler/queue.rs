//! queue.rs
//!
//! 基于 Tokio mpsc channel 的异步任务队列。

use anyhow::{Context, Result};
use tokio::sync::mpsc;

use crate::scheduler::InferenceTask;

#[derive(Clone)]
pub struct TaskQueue {
    sender: mpsc::Sender<InferenceTask>,
}

pub type TaskReceiver = mpsc::Receiver<InferenceTask>;

impl TaskQueue {
    pub fn new(buffer_size: usize) -> (Self, TaskReceiver) {
        let (sender, receiver) = mpsc::channel(buffer_size);
        (Self { sender }, receiver)
    }

    pub async fn submit(&self, task: InferenceTask) -> Result<()> {
        self.sender
            .send(task)
            .await
            .context("failed to submit inference task")?;

        Ok(())
    }
}
