#![allow(dead_code)]
#![allow(unused_variables)]
use std::env;

use anyhow::Result;
use dotenv::dotenv;
use ragent::client::Client;
use ragent::clients::anthropic::AnthropicClient;
use ragent::clients::openai::OpenAIClient;
use ragent::tool::Tool;
use ragent::tools::adder::{Adder, AdderArgs};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    match (Adder {}).run(AdderArgs { x: 10.0, y: 2.0 }) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    let openai_client = OpenAIClient::new(env::var("OPENAI_API_KEY")?);
    let anthropic_client = AnthropicClient::new(env::var("ANTHROPIC_API_KEY")?);

    let (openai_response, anthropic_response) =
        tokio::join!(openai_client.completion("What is the capital of Texas?"), anthropic_client.completion("What is the capital of Texas?"));

    println!("OpenAI: {}", openai_response?.get_message()?);
    println!("Anthropic: {}", anthropic_response?.get_message()?);

    Ok(())
}
