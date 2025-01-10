use crate::utils::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub description: String,
    pub system_prompt: String,
    pub model: String,
    pub history: Vec<String>,
}

impl Agent {
    pub fn new(name: String, description: String, system_prompt: String, model: String) -> Self {
        let id = safe_nanoid();
        Self { id, name, description, system_prompt, model, history: vec![] }
    }

    pub fn from_json(json: &str) -> Option<Self> {
        match serde_json::from_str(&json) {
            Ok(agent) => Some(agent),
            Err(e) => {
                eprintln!("Error deserializing agent: {}", e);
                None
            }
        }
    }

    pub fn prompt(&self, input: String) {
        println!("agentId = {}, input = {}", self.id, input)
    }
}
