// MIT License Copyright (c) 2024-present Frank Zhang
use async_openai::{
    types::{CreateChatCompletionResponse, CreateChatCompletionStreamResponse},
    Client,
};
use futures::Stream;
use std::pin::Pin;

use super::config::OllamaConfig;

pub type OllamaChatCompletionResponseStream =
    Pin<Box<dyn Stream<Item = Result<CreateChatCompletionStreamResponse, async_openai::error::OpenAIError>> + Send>>;

#[derive(Debug)]
pub struct OllamaChat {
    client: Client<OllamaConfig>,
}

#[derive(serde::Serialize, Debug, Clone, Default)]
pub struct OllamaChatCompletionRequest {
    #[serde(flatten)]
    pub common: super::types::ChatCompletionRequestCommon,
    pub messages: Vec<OllamaMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<OllamaOptions>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct OllamaChatCompletionResponse {
    #[serde(flatten)]
    pub common: super::types::ChatCompletionResponseCommon,
    pub message: Option<OllamaMessage>,
    pub prompt_eval_count: Option<u32>,
    pub eval_count: Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)] pub struct OllamaOptions {
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
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
    pub fn new(client: &Client<OllamaConfig>) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub async fn create(
        &self,
        request: OllamaChatCompletionRequest,
    ) -> Result<CreateChatCompletionResponse, async_openai::error::OpenAIError> {
        self.client.chat().create(request.into()).await
    }

    pub async fn create_stream(
        &self,
        request: OllamaChatCompletionRequest,
    ) -> Result<OllamaChatCompletionResponseStream, async_openai::error::OpenAIError> {
        self.client.chat().create_stream(request.into()).await
    }
}