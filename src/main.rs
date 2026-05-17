use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use dotenvy::dotenv;
use std::{env, process};

mod ui;
mod tools;
mod agent;
mod error;

use agent::AgentExecutor;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,

    /// Say yes to all
    #[arg(short = 'y', long)]
    yes: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    pretty_env_logger::init();

    let args = Args::parse();

    let base_url = env::var("OPENROUTER_BASE_URL")
        .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
        eprintln!("OPENROUTER_API_KEY is not set");
        process::exit(1);
    });

    let model = env::var("LOCAL_TEST_MODEL").unwrap_or("anthropic/claude-haiku-4.5".to_string());

    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    let client = Client::with_config(config);
    
    let executor = AgentExecutor::new(client, model, args.yes);
    if let Err(e) = executor.run(args.prompt).await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    Ok(())
}
