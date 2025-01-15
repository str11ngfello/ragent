use crate::client::{Client, ClientError, CompletionResponse, EmbeddingResponse};
use anyhow::{anyhow, bail, Context, Result};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAICompletionResponse {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIEmbeddingResponse {
    pub object: String,
    pub data: Vec<EmbeddingData>,
    pub model: String,
    pub usage: EmbeddingUsage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddingData {
    pub object: String,
    pub embedding: Vec<f32>,
    pub index: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddingUsage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}
pub struct OpenAIClient {
    http_client: reqwest::Client,
}

const BASE_URL_COMPLETION: &str = "https://api.openai.com/v1/chat/completions";
const BASE_URL_EMBEDDING: &str = "https://api.openai.com/v1/embeddings";

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

    async fn completion(&self, message: &str) -> Result<Box<dyn CompletionResponse>> {
        let response = self
            .http_client
            .post(BASE_URL_COMPLETION)
            .json(&json!({
                "model": "gpt-4o-mini",
                "messages": [{"role": "user", "content": message}],
                "temperature": 0.7
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
        let final_response: OpenAICompletionResponse =
            serde_json::from_str(&response_text).context("failed to parse completion response in openai client")?;
        Ok(Box::new(final_response))
    }

    async fn embedding(&self, document: &str) -> Result<Box<dyn EmbeddingResponse>> {
        let response = self
            .http_client
            .post(BASE_URL_EMBEDDING)
            .json(&json!({
                "model": "text-embedding-ada-002",
                "input": document,
                "encoding_format": "float"
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

        let final_response: OpenAIEmbeddingResponse =
            serde_json::from_str(&response_text).context("failed to parse embedding response in openai client")?;
        Ok(Box::new(final_response))
    }
}

impl CompletionResponse for OpenAICompletionResponse {
    fn get_message(&self) -> Result<String> {
        if let Some(choice) = self.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow!(ClientError::InvalidResponse))
        }
    }
}

impl EmbeddingResponse for OpenAIEmbeddingResponse {
    fn get_embedding(&self) -> Result<Vec<f32>> {
        if let Some(embedding) = self.data.first().map(|d| d.embedding.clone()) {
            Ok(embedding)
        } else {
            Err(anyhow!(ClientError::InvalidResponse))
        }
    }
}
