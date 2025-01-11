#![allow(dead_code)]
#![allow(unused_variables)]
use std::env;

use dotenv::dotenv;
mod adder;
use adder::{Adder, AdderArgs};
use ragent::client::Client;
use ragent::clients::openai::OpenAIClient;
use ragent::tool::Tool;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // match (Adder {}).run(AdderArgs { x: 10.0, y: 2.0 }) {
    //     Ok(result) => println!("Result: {}", result),
    //     Err(e) => println!("Error: {}", e),
    // }

    let client = OpenAIClient::new(env::var("OPENAI_API_KEY")?);
    let client_reponse = client
        .complete("What is the capital of Texas?".to_string())
        .await?;
    println!("{:#?}", client_reponse.get_message());

    Ok(())
}
