use crate::client::{Client, ClientError, CompletionResponse, EmbeddingResponse};
use anyhow::{anyhow, Context, Result};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub choices: Vec<Choice>,
    pub created: u64,
    pub id: String,
    pub model: String,
    pub object: String,
    pub service_tier: String,
    pub system_fingerprint: String,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub finish_reason: String,
    pub index: u32,
    pub logprobs: Option<serde_json::Value>, // Assuming logprobs can be any JSON value
    pub message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub content: String,
    pub refusal: Option<serde_json::Value>, // Assuming refusal can be any JSON value
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub completion_tokens: u32,
    pub completion_tokens_details: CompletionTokensDetails,
    pub prompt_tokens: u32,
    pub prompt_tokens_details: PromptTokensDetails,
    pub total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionTokensDetails {
    pub accepted_prediction_tokens: u32,
    pub audio_tokens: u32,
    pub reasoning_tokens: u32,
    pub rejected_prediction_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PromptTokensDetails {
    pub audio_tokens: u32,
    pub cached_tokens: u32,
}

pub struct OpenAIClient {
    http_client: reqwest::Client,
}

const BASE_URL: &str = "https://api.openai.com/v1/chat/completions";

impl Client for OpenAIClient {
    fn new(api_key: String) -> Self {
        let builder = reqwest::Client::builder();
        let mut header_map = HeaderMap::new();
        header_map.insert("content-type", HeaderValue::from_static("application/json"));
        header_map.insert("authorization", HeaderValue::from_str(&format!("Bearer {}", api_key)).expect("unable to build bearer header"));
        let http_client = builder
            .default_headers(header_map)
            .build()
            .expect("could not build http client");
        OpenAIClient { http_client }
    }

    async fn completion(&self, message: String) -> Result<Box<dyn CompletionResponse>> {
        let response = self
            .http_client
            .post(BASE_URL)
            .json(&json!({
                "model": "gpt-4o-mini",
                "messages": [{"role": "user", "content": message}],
                "temperature": 0.7
            }))
            .send()
            .await?;

        // Check if the response is successful
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("OpenAI API error: {}", error_text));
        }

        let response_text = response.text().await?;
        let final_response: Response = serde_json::from_str(&response_text).context("failed to parse response in openai client")?;
        Ok(Box::new(final_response))
    }

    async fn embedding(&self, document: String) -> Result<Box<dyn EmbeddingResponse>> {
        todo!()
    }
}

impl CompletionResponse for Response {
    fn get_message(&self) -> Result<String> {
        if let Some(choice) = self.choices.get(0) {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow!(ClientError::NoResponseError))
        }
    }
}
