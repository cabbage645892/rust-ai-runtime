use std::sync::Arc;

use anyhow::Result;

use crate::{
    inference::{context::InferenceContext, engine::InferenceEngine},
    tokenizer::{Tokenizer, WordTokenizer},
};

pub fn run_demo(engine: Arc<InferenceEngine>) -> Result<()> {
    println!("\n========== AI Runtime Demo ==========\n");

    let mut tokenizer = WordTokenizer::new();

    let prompt = "Rust is memory safe";
    let tokens = tokenizer.encode(prompt);

    let context = InferenceContext::new(prompt.to_string(), tokens);

    println!("Prompt: {}", prompt);
    println!("Tokens: {:?}", context.tokens);

    let result = engine.infer(&context)?;

    println!("\nResponse:\n{}", result.response);

    Ok(())
}
