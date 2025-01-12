use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("unknown client error: {0}")]
    UnknownError(String),
    #[error("no response from client")]
    NoResponseError,
    #[error("client response error ({status}): {message}")]
    ResponseError { status: u16, message: String },
}

pub trait CompletionResponse {
    fn get_message(&self) -> Result<String>;
}

pub trait EmbeddingResponse {
    fn get_embedding(&self) -> Result<String>;
}

pub trait Client {
    fn new(api_key: String) -> Self;
    async fn completion(&self, message: &str) -> Result<Box<dyn CompletionResponse>>;
    async fn embedding(&self, document: &str) -> Result<Box<dyn EmbeddingResponse>>;
}
