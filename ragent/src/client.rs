use std::error::Error;

pub trait CompletionResponse {
    fn get_message(&self) -> String;
}

pub trait EmbeddingResponse {
    fn get_embedding(&self) -> String;
}

pub trait Client {
    fn new(api_key: String) -> Self;
    async fn completion(&self, message: String) -> Result<Box<dyn CompletionResponse>, Box<dyn Error>>;
    async fn embedding(&self, document: String) -> Result<Box<dyn EmbeddingResponse>, Box<dyn Error>>;
}
