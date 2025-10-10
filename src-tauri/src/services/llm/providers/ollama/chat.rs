// This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
// BEAR LLM AI changes - Removed async_openai dependency, using reqwest directly
// MIT License Copyright (c) 2024-present Frank Zhang
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::config::OllamaConfig;
use crate::services::llm::providers::types;

#[derive(Debug)]
pub struct OllamaChat {
    client: Client,
    config: OllamaConfig,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct OllamaChatCompletionRequest {
    #[serde(flatten)]
    pub common: types::ChatCompletionRequestCommon,
    pub messages: Vec<OllamaMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<OllamaOptions>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OllamaChatCompletionResponse {
    #[serde(flatten)]
    pub common: types::ChatCompletionResponseCommon,
    pub message: Option<OllamaMessage>,
    pub prompt_eval_count: Option<u32>,
    pub eval_count: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OllamaOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl From<entity::entities::conversations::OllamaOptions> for OllamaOptions {
    fn from(options: entity::entities::conversations::OllamaOptions) -> Self {
        Self {
            temperature: options.temperature,
            top_p: options.top_p,
            top_k: options.top_k,
            stream: options.stream,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "role", content = "content")]
pub enum OllamaMessage {
    #[serde(rename = "system")]
    System(String),
    #[serde(rename = "user")]
    User(String),
    #[serde(rename = "assistant")]
    Assistant(String),
}

impl From<entity::entities::messages::MessageDTO> for OllamaMessage {
    fn from(message: entity::entities::messages::MessageDTO) -> Self {
        match message.role.as_str() {
            "system" => Self::System(message.content),
            "user" => Self::User(message.content),
            "assistant" => Self::Assistant(message.content),
            _ => Self::User(message.content),
        }
    }
}

impl OllamaChat {
    pub fn new(config: OllamaConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn create(
        &self,
        request: OllamaChatCompletionRequest,
    ) -> Result<OllamaChatCompletionResponse, reqwest::Error> {
        let url = format!("{}/api/chat", self.config.api_base);
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    pub async fn create_stream(
        &self,
        request: OllamaChatCompletionRequest,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/api/chat", self.config.api_base);
        self.client
            .post(&url)
            .json(&request)
            .send()
            .await
    }
}
