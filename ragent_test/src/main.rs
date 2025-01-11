#![allow(dead_code)]
#![allow(unused_variables)]
use std::env;

use dotenv::dotenv;
use ragent::client::Client;
use ragent::clients::anthropic::AnthropicClient;
use ragent::clients::openai::OpenAIClient;
use ragent::tool::Tool;
use ragent::tools::adder::{Adder, AdderArgs};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    match (Adder {}).run(AdderArgs { x: 10.0, y: 2.0 }) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    let client = OpenAIClient::new(env::var("OPENAI_API_KEY")?);
    let client_reponse = client
        .complete("What is the capital of Texas?".to_string())
        .await?;
    println!("{:#?}", client_reponse.get_message());

    let anthropic_client = AnthropicClient::new(env::var("ANTHROPIC_API_KEY")?);
    let client_reponse = anthropic_client
        .complete("What is the capital of Texas?".to_string())
        .await?;
    println!("{:#?}", client_reponse.get_message());

    Ok(())
}
