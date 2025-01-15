#![allow(dead_code)]
#![allow(unused_variables)]
use std::env;

use anyhow::{anyhow, Result};
use dotenv::dotenv;
use qdrant_client::qdrant::{CreateCollectionBuilder, Distance, PointStruct, UpsertPointsBuilder, VectorParamsBuilder};
use ragent::client::Client;
use ragent::clients::anthropic::AnthropicClient;
use ragent::clients::openai::OpenAIClient;
use ragent::tool::Tool;
use ragent::tools::adder::{Adder, AdderArgs};
use serde_json::json;
use tokio;

use qdrant_client::{Payload, Qdrant};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    match (Adder {}).run(AdderArgs { x: 10.0, y: 2.0 }) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    let openai_client = OpenAIClient::new(env::var("OPENAI_API_KEY")?);
    // let anthropic_client = AnthropicClient::new(env::var("ANTHROPIC_API_KEY")?);

    // let (openai_response, anthropic_response) =
    //     tokio::join!(openai_client.completion("What is the capital of Texas?"), anthropic_client.completion("What is the capital of Texas?"));

    // println!("OpenAI: {}", openai_response?.get_message()?);
    // println!("Anthropic: {}", anthropic_response?.get_message()?);

    // let openai_response = openai_client
    //     .embedding("apple code is 842u5832923ru243")
    //     .await?;
    // println!("OpenAI: {:?}", openai_response.get_embedding()?);

    let client = Qdrant::from_url("http://localhost:6334").build()?;

    // const COLLECTION_NAME: &str = "test_collection5";
    // // Check if collection exists before creating
    // let collections = client.list_collections().await?;
    // if !collections
    //     .collections
    //     .iter()
    //     .any(|i| i.name == COLLECTION_NAME)
    // {
    //     client
    //         .create_collection(CreateCollectionBuilder::new(COLLECTION_NAME).vectors_config(VectorParamsBuilder::new(1536, Distance::Dot)))
    //         .await?;
    //     println!("Created new collection {COLLECTION_NAME}");
    // }

    // let embedding = openai_response.get_embedding()?;

    // let point = PointStruct::new(
    //     1,
    //     embedding,
    //     Payload::try_from(json!({
    //         "test":"data"
    //     }))?,
    // );
    // let response = client
    //     .upsert_points(UpsertPointsBuilder::new(COLLECTION_NAME, [point]))
    //     .await?;

    // println!("Response: {:?}", response.result.ok_or(anyhow!("No result"))?);

    // let search = client
    //     .scroll(qdrant_client::qdrant::ScrollPoints {
    //         collection_name: COLLECTION_NAME.to_string(),
    //         filter: None,
    //         offset: None,
    //         limit: Some(10),
    //         with_payload: Some(qdrant_client::qdrant::WithPayloadSelector::from(true)),
    //         with_vectors: Some(true.into()),
    //         read_consistency: None,
    //         shard_key_selector: None,
    //         order_by: None,
    //         timeout: None,
    //     })
    //     .await?;
    // println!("Points in collection: {:?}", search);

    Ok(())
}
