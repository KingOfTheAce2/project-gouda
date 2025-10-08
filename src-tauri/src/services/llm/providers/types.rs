// MIT License Copyright (c) 2024-present Frank Zhang
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Debug, Clone, Default)]
pub struct ChatCompletionRequestCommon {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<ChatCompletionStreamOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct ChatCompletionStreamOptions {
    pub include_usage: bool,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ChatCompletionResponseCommon {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens_details: Option<CompletionTokensDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u32>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct CompletionTokensDetails {
    pub reasoning_tokens: Option<u32>,
}
