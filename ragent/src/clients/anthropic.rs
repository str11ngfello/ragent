use crate::client::{Client, ClientResponse};
use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub id: String,
    #[serde(rename = "type")]
    pub response_type: String,
    pub role: String,
    pub model: String,
    pub content: Vec<Content>,
    pub stop_reason: String,
    pub stop_sequence: Option<String>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub input_tokens: u32,
    pub cache_creation_input_tokens: u32,
    pub cache_read_input_tokens: u32,
    pub output_tokens: u32,
}
pub struct AnthropicClient {
    http_client: reqwest::Client,
}

const BASE_URL: &str = "https://api.anthropic.com/v1/messages";

impl Client for AnthropicClient {
    fn new(api_key: String) -> Self {
        let builder = reqwest::Client::builder();
        let mut header_map = HeaderMap::new();
        header_map.insert("x-api-key", HeaderValue::from_str(&api_key).expect("unable to build bearer header"));
        header_map.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
        header_map.insert("content-type", HeaderValue::from_static("application/json"));
        let http_client = builder
            .default_headers(header_map)
            .build()
            .expect("could not build http client");
        AnthropicClient { http_client }
    }

    async fn complete(&self, message: String) -> Result<Box<dyn ClientResponse>, Box<dyn Error>> {
        let response = self
            .http_client
            .post(BASE_URL)
            .json(&json!({
                "model": "claude-3-5-sonnet-20241022",
                "max_tokens": 1024,
                "messages": [
                    {"role": "user", "content": message}
                ]
            }))
            .send()
            .await?
            .text()
            .await?;

        let final_response: Response = serde_json::from_str(&response).context("failed to parse response in anthropic client")?;
        Ok(Box::new(final_response))
    }
}

impl ClientResponse for Response {
    fn get_message(&self) -> String {
        if let Some(text) = self.content.get(0) {
            text.text.clone()
        } else {
            "asdf".to_string()
        }
    }
}
