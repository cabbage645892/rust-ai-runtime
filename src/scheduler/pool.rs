//! pool.rs
//!
//! Worker Pool：负责启动多个 Worker 并调度任务。

use std::sync::Arc;

use anyhow::Result;
use tokio::{sync::Mutex, task::JoinHandle};

use crate::inference::InferenceEngine;

use super::{InferenceTask, TaskQueue, TaskReceiver, Worker};

pub struct WorkerPool {
    worker_count: usize,
}

impl WorkerPool {
    pub fn new(worker_count: usize) -> Self {
        Self {
            worker_count: worker_count.max(1),
        }
    }

    pub fn start(
        &self,
        receiver: TaskReceiver,
        engine: Arc<InferenceEngine>,
    ) -> Vec<JoinHandle<()>> {
        let receiver = Arc::new(Mutex::new(receiver));
        let mut handles = Vec::new();

        for id in 0..self.worker_count {
            let receiver = Arc::clone(&receiver);
            let engine = Arc::clone(&engine);

            let handle = tokio::spawn(async move {
                let worker = Worker::new(id, engine);

                println!("Worker {} started.", id);

                loop {
                    let task = {
                        let mut rx = receiver.lock().await;
                        rx.recv().await
                    };

                    match task {
                        Some(task) => {
                            if let Err(e) = worker.process(task).await {
                                eprintln!("[Worker {}] Error: {}", id, e);
                            }
                        }
                        None => {
                            println!("Worker {} stopped.", id);
                            break;
                        }
                    }
                }
            });

            handles.push(handle);
        }

        handles
    }

    pub fn create_queue() -> (TaskQueue, TaskReceiver) {
        TaskQueue::new(128)
    }

    pub async fn submit(queue: &TaskQueue, id: u64, prompt: impl Into<String>) -> Result<()> {
        let task = InferenceTask::new(id, prompt);
        queue.submit(task).await
    }
}
