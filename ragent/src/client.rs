use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("unknown client error")]
    UnknownError,
    #[error("no response from client")]
    NoResponseError,
}

pub trait CompletionResponse {
    fn get_message(&self) -> Result<String>;
}

pub trait EmbeddingResponse {
    fn get_embedding(&self) -> Result<String>;
}

pub trait Client {
    fn new(api_key: String) -> Self;
    async fn completion(&self, message: String) -> Result<Box<dyn CompletionResponse>>;
    async fn embedding(&self, document: String) -> Result<Box<dyn EmbeddingResponse>>;
}
