// This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
// BEAR LLM AI changes - Removed async_openai dependency, using direct error types
// MIT License Copyright (c) 2024-present Frank Zhang
use std::pin::Pin;

use crate::log_utils::warn;
use entity::entities::{
    conversations::{GenericOptions, OllamaOptions},
    messages::MessageDTO,
};
use serde::Serialize;
use tokio_stream::{Stream, StreamExt};

use super::{
    providers::ollama::{
        chat::{
            OllamaChat, OllamaChatCompletionRequest,
            OllamaMessage,
        },
        config::OllamaConfig,
    },
    utils::sum_option,
};

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct BotReply {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub reasoning: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub prompt_token: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub completion_token: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub reasoning_token: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub total_token: Option<u32>,
}

pub type BotReplyStream = Pin<Box<dyn Stream<Item = Result<BotReply, String>> + Send>>;

pub struct GlobalSettings {
    pub max_tokens: u32,
}

pub enum ChatRequestExecutor {
    OllamaChatRequestExecutor(OllamaConfig, OllamaChatCompletionRequest),
}

impl ChatRequestExecutor {
    pub fn ollama(
        config: &OllamaConfig,
        messages: Vec<MessageDTO>,
        options: GenericOptions,
        _global_settings: GlobalSettings,
        model: String,
    ) -> Result<ChatRequestExecutor, String> {
        let request: OllamaChatCompletionRequest;
        // set messages
        let req_messages: Vec<OllamaMessage> = messages
            .into_iter()
            .map(Into::<OllamaMessage>::into)
            .collect();
        // set options
        let options: OllamaOptions = serde_json::from_str(&options.options)
            .map_err(|_| format!("Failed to parse conversation options: {}", &options.options))?;
        // build request
        // Stream must be set to false explictly for Ollama, or it will treat the request as a Stream request
        let stream = options.stream.clone().unwrap_or(false);
        request = OllamaChatCompletionRequest {
            common: super::providers::types::ChatCompletionRequestCommon {
                model: model.to_string(),
                stream: Some(stream),
                ..Default::default()
            },
            messages: req_messages,
            options: Some(options.into()),
            ..Default::default()
        };
        Ok(ChatRequestExecutor::OllamaChatRequestExecutor(
            config.clone(), request,
        ))
    }

    pub async fn execute(&self) -> Result<BotReply, String> {
        let log_tag = "ChatRequest::execute";
        match self {
            ChatRequestExecutor::OllamaChatRequestExecutor(config, request) => {
                let response = OllamaChat::new(config.clone())
                    .create(request.clone())
                    .await
                    .map_err(|err| {
                        log::error!("execute ChatRequest::OllamaChatRequest: {:?}", err);
                        format!("Failed to get chat completion response: {}", err)
                    })?;
                let message: String = match response.message {
                    Some(response_message) => match response_message {
                        OllamaMessage::Assistant(content) => content,
                        _ => {
                            warn(
                                log_tag,
                                "OllamaChat::create returned a non-assistant message",
                            );
                            String::default()
                        }
                    },
                    _ => {
                        warn(log_tag, "OllamaChat::create returned an empty message");
                        String::default()
                    }
                };
                // extract data & build reply
                Ok(BotReply {
                    message,
                    reasoning: None,
                    prompt_token: response.prompt_eval_count,
                    completion_token: response.eval_count,
                    reasoning_token: None,
                    total_token: sum_option(response.prompt_eval_count, response.eval_count),
                })
            }
        }
    }

    pub async fn execute_stream(&self) -> Result<BotReplyStream, String> {
        let log_tag = "ChatRequest::execute_stream";
        match self {
            ChatRequestExecutor::OllamaChatRequestExecutor(config, request) => {
                let response = OllamaChat::new(config.clone())
                    .create_stream(request.clone())
                    .await
                    .map_err(|err| format!("Error creating stream: {}", err.to_string()))?;

                // Parse the streaming response
                let stream = response.bytes_stream();
                let mut is_reasoning = false;

                let result = stream.map(move |chunk_result| {
                    chunk_result
                        .map_err(|e| format!("Stream error: {}", e))
                        .and_then(|chunk| {
                            let text = String::from_utf8_lossy(&chunk);
                            serde_json::from_str::<super::providers::ollama::chat::OllamaChatCompletionResponse>(&text)
                                .map_err(|e| format!("JSON parse error: {}", e))
                        })
                        .map(|response| {
                            let content: String = match response.message {
                                Some(response_message) => match response_message {
                                    OllamaMessage::Assistant(content) => {
                                        // check for reasoning content
                                        // return empty content for <think> and </think>
                                        if content.contains("<think>") {
                                            is_reasoning = true;
                                            String::default()
                                        } else if content.contains("</think>") {
                                            is_reasoning = false;
                                            String::default()
                                        } else {
                                            content
                                        }
                                    }
                                    _ => {
                                        warn(
                                            log_tag,
                                            "OllamaChat::create_stream returned a non-assistant message",
                                        );
                                        String::default()
                                    }
                                },
                                _ => {
                                    // normally the last message of the stream
                                    String::default()
                                }
                            };

                            BotReply {
                                message: if is_reasoning {
                                    String::default()
                                } else {
                                    content.clone()
                                },
                                reasoning: if is_reasoning {
                                    Some(content)
                                } else {
                                    None
                                },
                                prompt_token: response.prompt_eval_count,
                                completion_token: response.eval_count,
                                reasoning_token: None,
                                total_token: sum_option(
                                    response.prompt_eval_count,
                                    response.eval_count,
                                ),
                            }
                        })
                });
                Ok(Box::pin(result))
            }
        }
    }
}
