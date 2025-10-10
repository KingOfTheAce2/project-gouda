// This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
// BEAR LLM AI changes - Removed async_openai dependency, using direct Ollama config
// MIT License Copyright (c) 2024-present Frank Zhang
use entity::entities::{
    conversations::GenericOptions,
    messages::MessageDTO,
    models::{GenericConfig, Providers},
    settings::ProxySetting,
};
use reqwest;

use super::{
    chat::{BotReply, BotReplyStream, ChatRequestExecutor, GlobalSettings},
    models::{ListModelsRequestExecutor, RemoteModel},
    providers::ollama::config::OllamaConfig,
    types::RawOllamaConfig,
    utils::build_http_client,
};

/// LLM Client for various providers
#[derive(Debug, Clone)]
pub enum LLMClient {
    OllamaClient(OllamaConfig, Option<String>),
}

impl LLMClient {
    /// Build client from config
    pub fn new(
        config: GenericConfig,
        proxy_setting: Option<ProxySetting>,
    ) -> Result<Self, String> {
        let _http_client: reqwest::Client = build_http_client(proxy_setting);
        match config.provider.as_str().into() {
            Providers::Ollama => {
                let raw_config: RawOllamaConfig = serde_json::from_str(&config.config)
                    .map_err(|_| format!("Failed to parse model config: {}", &config.config))?;
                let model = raw_config.model.clone();
                let ollama_config: OllamaConfig = raw_config.into();
                Ok(LLMClient::OllamaClient(ollama_config, model))
            }
            _ => Err("Unsupported provider".to_string()),
        }
    }

    pub async fn chat(
        &self,
        messages: Vec<MessageDTO>,
        options: GenericOptions,
        global_settings: GlobalSettings,
    ) -> Result<BotReply, String> {
        match self {
            LLMClient::OllamaClient(config, model) => match model {
                Some(model_str) => {
                    let reply =
                        ChatRequestExecutor::ollama(config, messages, options, global_settings, model_str.to_string())?
                            .execute()
                            .await?;
                    Ok(reply)
                }
                None => Err(format!("Model not set for chat")),
            },
        }
    }

    pub async fn chat_stream(
        &self,
        messages: Vec<MessageDTO>,
        options: GenericOptions,
        global_settings: GlobalSettings,
    ) -> Result<BotReplyStream, String> {
        match self {
            LLMClient::OllamaClient(config, model) => match model {
                Some(model_str) => {
                    let stream =
                        ChatRequestExecutor::ollama(config, messages, options, global_settings, model_str.to_string())?
                            .execute_stream()
                            .await?;
                    Ok(stream)
                }
                None => Err(format!("Model not set for chat")),
            },
        }
    }

    pub async fn models(&self) -> Result<Vec<RemoteModel>, String> {
        match self {
            LLMClient::OllamaClient(config, _) => {
                let result = ListModelsRequestExecutor::ollama(config).execute().await?;
                Ok(result)
            }
        }
    }
}
