use crate::client::{Client, ClientError, CompletionResponse, EmbeddingResponse};
use anyhow::{anyhow, bail, Context, Result};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::json;

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

    async fn completion(&self, message: String) -> Result<Box<dyn CompletionResponse>> {
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
            .await?;

        // Check if the response is successful
        if !response.status().is_success() {
            let status_code = response.status().as_u16();
            let error_text = response.text().await?;
            bail!(ClientError::ResponseError { status: status_code, message: error_text });
        }

        let response_text = response.text().await?;
        let final_response: Response = serde_json::from_str(&response_text).context("failed to parse response in anthropic client")?;
        Ok(Box::new(final_response))
    }

    async fn embedding(&self, document: String) -> Result<Box<dyn EmbeddingResponse>> {
        todo!()
    }
}

impl CompletionResponse for Response {
    fn get_message(&self) -> Result<String> {
        if let Some(content) = self.content.get(0) {
            Ok(content.text.clone())
        } else {
            Err(anyhow!(ClientError::NoResponseError))
        }
    }
}
