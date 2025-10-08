// MIT License Copyright (c) 2024-present Frank Zhang
use async_openai::Client;
use serde::{
    Deserialize,
    Serialize,
};

use super::config::OllamaConfig;

#[derive(Debug)]
pub struct OllamaModels {
    client: Client<OllamaConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaModel {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaListModelsResponse {
    pub models: Vec<OllamaModel>,
}

impl OllamaModels {
    pub fn new(client: &Client<OllamaConfig>) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub async fn list(&self) -> Result<OllamaListModelsResponse, async_openai::error::OpenAIError> {
        self.client.models().list().await
    }
}