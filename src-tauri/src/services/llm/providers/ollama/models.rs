// This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
// BEAR LLM AI changes - Removed async_openai dependency, using reqwest directly
// MIT License Copyright (c) 2024-present Frank Zhang
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::config::OllamaConfig;

#[derive(Debug)]
pub struct OllamaModels {
    client: Client,
    config: OllamaConfig,
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
    pub fn new(config: OllamaConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn list(&self) -> Result<OllamaListModelsResponse, reqwest::Error> {
        let url = format!("{}/api/tags", self.config.api_base);
        let response = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}
