use std::sync::Arc;

use anyhow::Result;

use crate::{inference::engine::InferenceEngine, scheduler::WorkerPool};

pub struct RuntimeManager {
    engine: Arc<InferenceEngine>,
    pool_size: usize,
}

impl RuntimeManager {
    pub fn new(engine: Arc<InferenceEngine>, pool_size: usize) -> Self {
        Self { engine, pool_size }
    }

    pub async fn start(&self) -> Result<()> {
        println!("Starting Runtime...");
        println!("Worker Pool Size: {}", self.pool_size);
        println!("Current Model: {}", self.engine.current_model_name());

        let pool = WorkerPool::new(self.pool_size);
        let (queue, receiver) = WorkerPool::create_queue();
        let handles = pool.start(receiver, Arc::clone(&self.engine));

        WorkerPool::submit(&queue, 1, "Rust is memory safe").await?;
        WorkerPool::submit(&queue, 2, "Tell me about ownership").await?;
        WorkerPool::submit(&queue, 3, "AI inference with Rust").await?;

        drop(queue);

        for handle in handles {
            handle.await?;
        }

        println!("Runtime Ready.");
        Ok(())
    }
}
