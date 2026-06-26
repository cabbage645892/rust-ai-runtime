//! cli.rs
//!
//! 命令行参数解析模块。
//! 不依赖 clap，避免额外引入新依赖。

use anyhow::{Result, anyhow, bail};

#[derive(Debug, Clone)]
pub enum Command {
    Demo,
    Infer {
        model: String,
        prompt: String,
    },
    Batch {
        model: String,
        input: String,
    },
    Benchmark {
        model: String,
        prompt: String,
        requests: usize,
        workers: usize,
    },
    History {
        limit: usize,
    },
    Help,
}

#[derive(Debug, Clone)]
pub struct Cli {
    pub command: Command,
}

impl Cli {
    pub fn parse() -> Result<Self> {
        let args: Vec<String> = std::env::args().skip(1).collect();

        if args.is_empty() {
            return Ok(Self {
                command: Command::Help,
            });
        }

        let command = match args[0].as_str() {
            "demo" => Command::Demo,

            "infer" => Command::Infer {
                model: option_value(&args, "--model")?.unwrap_or_else(|| "rule".to_string()),
                prompt: option_value(&args, "--prompt")?
                    .ok_or_else(|| anyhow!("infer requires --prompt"))?,
            },

            "batch" => Command::Batch {
                model: option_value(&args, "--model")?.unwrap_or_else(|| "rule".to_string()),
                input: option_value(&args, "--input")?
                    .ok_or_else(|| anyhow!("batch requires --input"))?,
            },

            "benchmark" => Command::Benchmark {
                model: option_value(&args, "--model")?.unwrap_or_else(|| "rule".to_string()),
                prompt: option_value(&args, "--prompt")?
                    .unwrap_or_else(|| "Rust is memory safe".to_string()),
                requests: parse_usize_option(&args, "--requests", 20)?,
                workers: parse_usize_option(&args, "--workers", 4)?.max(1),
            },

            "history" => Command::History {
                limit: parse_usize_option(&args, "--limit", 10)?,
            },

            "help" | "--help" | "-h" => Command::Help,

            other => bail!("unknown command: {other}"),
        };

        Ok(Self { command })
    }

    pub fn print_help() {
        println!("Rust AI Runtime Toolkit");
        println!();
        println!("USAGE:");
        println!("  cargo run -- demo");
        println!("  cargo run -- infer --model rule --prompt \"hello rust\"");
        println!("  cargo run -- infer --model markov --prompt \"Rust is\"");
        println!("  cargo run -- batch --model rule --input examples/prompts.txt");
        println!("  cargo run -- benchmark --model rule --requests 100 --workers 4");
        println!("  cargo run -- history --limit 10");
        println!();
        println!("MODELS:");
        println!("  rule    RuleBasedModel");
        println!("  markov  MarkovModel");
    }
}

fn option_value(args: &[String], name: &str) -> Result<Option<String>> {
    let Some(index) = args.iter().position(|arg| arg == name) else {
        return Ok(None);
    };

    let Some(value) = args.get(index + 1) else {
        bail!("{name} requires a value");
    };

    if value.starts_with("--") {
        bail!("{name} requires a value");
    }

    Ok(Some(value.clone()))
}

fn parse_usize_option(args: &[String], name: &str, default: usize) -> Result<usize> {
    match option_value(args, name)? {
        Some(value) => value
            .parse::<usize>()
            .map_err(|_| anyhow!("{name} must be a positive integer")),
        None => Ok(default),
    }
}
