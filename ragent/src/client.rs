use std::error::Error;

pub trait ClientResponse {
    fn get_message(&self) -> String;
}
pub trait Client {
    fn new(api_key: String) -> Self;
    async fn complete(&self, message: String) -> Result<Box<dyn ClientResponse>, Box<dyn Error>>;
}
