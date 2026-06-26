use std::{
    fs,
    sync::Arc,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, bail};
use tokio::sync::Semaphore;

use rust_ai_runtime::{
    cli::{Cli, Command},
    config::AppConfig,
    demo::run_demo,
    history::{HistoryRecord, HistoryStore},
    inference::{
        context::{InferenceContext, InferenceResult},
        engine::InferenceEngine,
        models::{MarkovModel, RuleBasedModel},
    },
    runtime::RuntimeManager,
    tokenizer::{Tokenizer, WordTokenizer},
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("===================================");
    println!(" Rust AI Runtime Toolkit ");
    println!("===================================\n");

    let cli = Cli::parse()?;
    let config = AppConfig::load("config/settings.toml");

    match cli.command {
        Command::Help => Cli::print_help(),

        Command::Demo => {
            run_demo_command(&config).await?;
        }

        Command::Infer { model, prompt } => {
            infer_command(&config, &model, &prompt)?;
        }

        Command::Batch { model, input } => {
            batch_command(&config, &model, &input)?;
        }

        Command::Benchmark {
            model,
            prompt,
            requests,
            workers,
        } => {
            benchmark_command(&config, &model, &prompt, requests, workers).await?;
        }

        Command::History { limit } => {
            history_command(&config, limit)?;
        }
    }

    Ok(())
}

async fn run_demo_command(config: &AppConfig) -> Result<()> {
    let engine = build_engine(config, "rule")?;
    let engine = Arc::new(engine);

    let runtime = RuntimeManager::new(Arc::clone(&engine), config.worker_count);

    runtime.start().await?;
    run_demo(engine)?;

    Ok(())
}

fn infer_command(config: &AppConfig, model: &str, prompt: &str) -> Result<()> {
    let engine = Arc::new(build_engine(config, model)?);
    let history = HistoryStore::new(&config.history_path);

    let started = Instant::now();
    let result = infer_once(&engine, prompt)?;
    let elapsed_ms = started.elapsed().as_millis() as u64;

    println!("Model: {}", engine.current_model_name());
    println!("Prompt: {}", prompt);
    println!("Response: {}", result.response);
    println!("Elapsed: {} ms", elapsed_ms);

    history.append(HistoryRecord::new(
        next_history_id(),
        engine.current_model_name(),
        prompt,
        result.response,
        elapsed_ms,
    ))?;

    Ok(())
}

fn batch_command(config: &AppConfig, model: &str, input: &str) -> Result<()> {
    let engine = Arc::new(build_engine(config, model)?);
    let history = HistoryStore::new(&config.history_path);

    let content = fs::read_to_string(input).with_context(|| format!("failed to read {input}"))?;

    for (index, line) in content.lines().enumerate() {
        let prompt = line.trim();

        if prompt.is_empty() {
            continue;
        }

        let started = Instant::now();
        let result = infer_once(&engine, prompt)?;
        let elapsed_ms = started.elapsed().as_millis() as u64;

        println!("\n[{}] Prompt: {}", index + 1, prompt);
        println!("[{}] Response: {}", index + 1, result.response);
        println!("[{}] Elapsed: {} ms", index + 1, elapsed_ms);

        history.append(HistoryRecord::new(
            next_history_id() + index as u64,
            engine.current_model_name(),
            prompt,
            result.response,
            elapsed_ms,
        ))?;
    }

    Ok(())
}

async fn benchmark_command(
    config: &AppConfig,
    model: &str,
    prompt: &str,
    requests: usize,
    workers: usize,
) -> Result<()> {
    if requests == 0 {
        bail!("--requests must be greater than 0");
    }

    let engine = Arc::new(build_engine(config, model)?);
    let semaphore = Arc::new(Semaphore::new(workers.max(1)));

    let started = Instant::now();
    let mut handles = Vec::new();

    for _ in 0..requests {
        let permit = Arc::clone(&semaphore).acquire_owned().await?;
        let engine = Arc::clone(&engine);
        let prompt = prompt.to_string();

        let handle = tokio::spawn(async move {
            let _permit = permit;
            infer_once(&engine, &prompt).map(|result| result.response.len())
        });

        handles.push(handle);
    }

    let mut total_response_bytes = 0usize;

    for handle in handles {
        total_response_bytes += handle.await??;
    }

    let elapsed = started.elapsed();
    let elapsed_secs = elapsed.as_secs_f64();
    let throughput = requests as f64 / elapsed_secs.max(0.001);

    println!("Benchmark finished");
    println!("Model: {}", engine.current_model_name());
    println!("Requests: {}", requests);
    println!("Workers: {}", workers.max(1));
    println!("Prompt: {}", prompt);
    println!("Total response bytes: {}", total_response_bytes);
    println!("Elapsed: {:.3} s", elapsed_secs);
    println!("Throughput: {:.2} req/s", throughput);

    Ok(())
}

fn history_command(config: &AppConfig, limit: usize) -> Result<()> {
    let history = HistoryStore::new(&config.history_path);
    let records = history.list(limit)?;

    if records.is_empty() {
        println!("No history records.");
        return Ok(());
    }

    for record in records {
        println!("\n#{} [{}] {}", record.id, record.model, record.timestamp);
        println!("Prompt: {}", record.prompt);
        println!("Response: {}", record.response);
        println!("Elapsed: {} ms", record.elapsed_ms);
    }

    Ok(())
}

fn build_engine(config: &AppConfig, model_alias: &str) -> Result<InferenceEngine> {
    let mut engine = InferenceEngine::new();

    engine.add_model(Arc::new(RuleBasedModel::load(&config.rules_path)?));
    engine.add_model(Arc::new(MarkovModel::load(&config.markov_corpus_path)?));

    let model_name = normalize_model_name(model_alias)?;
    engine.set_model_by_name(model_name)?;

    Ok(engine)
}

fn normalize_model_name(alias: &str) -> Result<&'static str> {
    match alias.to_lowercase().as_str() {
        "rule" | "rules" | "rule_based" | "rulebased" | "rulebasedmodel" => Ok("RuleBasedModel"),
        "markov" | "markovmodel" => Ok("MarkovModel"),
        _ => bail!("unknown model alias: {alias}. Available models: rule, markov"),
    }
}

fn infer_once(engine: &InferenceEngine, prompt: &str) -> Result<InferenceResult> {
    let mut tokenizer = WordTokenizer::new();
    let tokens = tokenizer.encode(prompt);
    let context = InferenceContext::new(prompt.to_string(), tokens);

    engine.infer(&context)
}

fn next_history_id() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or_default()
}
