/* This change is Copyright BEAR LLM AI project, which is proprietory. */
// MIT License Copyright (c) 2024-present Frank Zhang
use async_openai::{config::Config, Client};
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
    types::{RawOllamaConfig, RawOpenAIConfig},
    utils::build_http_client,
};

/// Wrapper of async-openai's Client struct
#[derive(Debug, Clone)]
pub enum LLMClient {
    OllamaClient(Client<OllamaConfig>, Option<String>),
    CustomClient(Client<Config>, Option<String>),
}

impl LLMClient {
    /// Build client from config
    pub fn new(
        config: GenericConfig,
        proxy_setting: Option<ProxySetting>,
    ) -> Result<Self, String> {
        let http_client: reqwest::Client = build_http_client(proxy_setting);
        match config.provider.as_str().into() {
            Providers::Ollama => {
                let raw_config: RawOllamaConfig = serde_json::from_str(&config.config)
                    .map_err(|_| format!("Failed to parse model config: {}", &config.config))?;
                let model = raw_config.model.clone();
                let client = Client::with_config(raw_config.into()).with_http_client(http_client);
                Ok(LLMClient::OllamaClient(client, model))
            }
            Providers::CUSTOM => {
                let raw_config: RawOpenAIConfig = serde_json::from_str(&config.config)
                    .map_err(|_| format!("Failed to parse model config: {}", &config.config))?;
                let model = raw_config.model.clone();
                let client = Client::with_config(raw_config.into()).with_http_client(http_client);
                Ok(LLMClient::CustomClient(client, model))
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
            LLMClient::OllamaClient(client, model) => match model {
                Some(model_str) => {
                    let reply =
                        ChatRequestExecutor::ollama(client, messages, options, global_settings, model_str.to_string())?
                            .execute()
                            .await?;
                    Ok(reply)
                }
                None => Err(format!("Model not set for chat")),
            },
            LLMClient::CustomClient(client, model) => match model {
                Some(model_str) => {
                    let reply =
                        ChatRequestExecutor::custom(client, messages, options, global_settings, model_str.to_string())?
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
            LLMClient::OllamaClient(client, model) => match model {
                Some(model_str) => {
                    let stream =
                        ChatRequestExecutor::ollama(client, messages, options, global_settings, model_str.to_string())?
                            .execute_stream()
                            .await?;
                    Ok(stream)
                }
                None => Err(format!("Model not set for chat")),
            },
            LLMClient::CustomClient(client, model) => match model {
                Some(model_str) => {
                    let stream =
                        ChatRequestExecutor::custom(client, messages, options, global_settings, model_str.to_string())?
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
            LLMClient::OllamaClient(client, _) => {
                let result = ListModelsRequestExecutor::ollama(client).execute().await?;
                Ok(result)
            }
            LLMClient::CustomClient(client, _) => {
                let result = ListModelsRequestExecutor::custom(client).execute().await?;
                Ok(result)
            }
        }
    }
}
