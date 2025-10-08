/* This change is Copyright BEAR LLM AI project, which is proprietory. */
// MIT License Copyright (c) 2024-present Frank Zhang
use std::pin::Pin;

use crate::log_utils::warn;
use async_openai::{
    config::Config,
    error::OpenAIError,
    Client,
    types::{CreateChatCompletionRequest, ChatMessage},
};
use entity::entities::{
    conversations::{GenericOptions, OllamaOptions, OpenAIOptions},
    messages::MessageDTO,
};
use serde::Serialize;
use tokio_stream::{Stream, StreamExt};

use super::{
    providers::ollama::{
        chat::{
            OllamaChat, OllamaChatCompletionRequest, OllamaChatCompletionResponseStream,
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

pub type BotReplyStream = Pin<Box<dyn Stream<Item = Result<BotReply, OpenAIError>> + Send>>;

pub struct GlobalSettings {
    pub max_tokens: u32,
}

pub enum ChatRequestExecutor<'c> {
    OllamaChatRequestExecutor(&'c Client<OllamaConfig>, OllamaChatCompletionRequest),
    CustomChatRequestExecutor(&'c Client<Config>, CreateChatCompletionRequest),
}

impl<'c> ChatRequestExecutor<'c> {
    pub fn ollama(
        client: &'c Client<OllamaConfig>,
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
            client, request,
        ))
    }

    pub fn custom(
        client: &'c Client<Config>,
        messages: Vec<MessageDTO>,
        options: GenericOptions,
        global_settings: GlobalSettings,
        model: String,
    ) -> Result<ChatRequestExecutor, String> {
        let request: CreateChatCompletionRequest;
        // set messages
        let req_messages: Vec<ChatMessage> = messages
            .into_iter()
            .map(Into::<ChatMessage>::into)
            .collect();
        // set options
        let options: OpenAIOptions = serde_json::from_str(&options.options)
            .map_err(|_| format!("Failed to parse conversation options: {}", &options.options))?;
        // build request
        let stream = options.stream.clone().unwrap_or(false);
        request = CreateChatCompletionRequest {
            model: model.to_string(),
            messages: req_messages,
            stream: Some(stream),
            temperature: options.temperature,
            top_p: options.top_p,
            max_tokens: options.max_tokens,
            presence_penalty: options.presence_penalty,
            frequency_penalty: options.frequency_penalty,
            user: options.user,
            ..Default::default()
        };
        Ok(ChatRequestExecutor::CustomChatRequestExecutor(
            client, request,
        ))
    }

    pub async fn execute(&self) -> Result<BotReply, String> {
        let log_tag = "ChatRequest::execute";
        match self {
            ChatRequestExecutor::OllamaChatRequestExecutor(client, request) => {
                let response = OllamaChat::new(client)
                    .create(request.clone())
                    .await
                    .map_err(|err| {
                        log::error!("execute ChatRequest::OllamaChatRequest: {:?}", err);
                        format!("Failed to get chat completion response: {}", err)
                    })?;
                let message: String = match response.message {
                    Some(response_message) => match response_message {
                        OllamaMessage::Assistant(content) => content.content,
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
            ChatRequestExecutor::CustomChatRequestExecutor(client, request) => {
                let response = client.chat().create(request.clone()).await.map_err(|err| {
                    log::error!("execute ChatRequest::CustomChatRequest: {:?}", err);
                    format!("Failed to get chat completion response: {}", err)
                })?;
                let message = response.choices[0].message.content.clone().unwrap_or_default();
                let prompt_token = response.usage.clone().map(|u| u.prompt_tokens);
                let completion_token = response.usage.clone().map(|u| u.completion_tokens);
                let total_token = response.usage.map(|u| u.total_tokens);
                Ok(BotReply {
                    message,
                    reasoning: None,
                    prompt_token,
                    completion_token,
                    reasoning_token: None,
                    total_token,
                })
            }
        }
    }

    pub async fn execute_stream(&self) -> Result<BotReplyStream, String> {
        let log_tag = "ChatRequest::execute_stream";
        match self {
            ChatRequestExecutor::OllamaChatRequestExecutor(client, request) => {
                let stream: OllamaChatCompletionResponseStream = OllamaChat::new(client)
                    .create_stream(request.clone())
                    .await
                    .map_err(|err| format!("Error creating stream: {}", err.to_string()))?;
                let mut is_reasoning = false;
                let result = stream.map(move |item| {
                    item.map(|response| {
                        let content: String = match response.message {
                            Some(response_message) => match response_message {
                                OllamaMessage::Assistant(content) => {
                                    // check for reasoning content
                                    // return empty content for <think> and </think>
                                    if content.content.contains("<think>") {
                                        is_reasoning = true;
                                        String::default()
                                    } else if content.content.contains("</think>") {
                                        is_reasoning = false;
                                        String::default()
                                    } else {
                                        content.content
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
            ChatRequestExecutor::CustomChatRequestExecutor(client, request) => {
                let stream = client.chat().create_stream(request.clone()).await.map_err(|err| {
                    log::error!("execute_stream ChatRequest::CustomChatRequest: {:?}", err);
                    format!("Error creating stream: {}", err.to_string())
                })?;
                let result = stream.map(|item| {
                    item.map(|response| {
                        let content = response.choices[0].delta.content.clone().unwrap_or_default();
                        BotReply {
                            message: content,
                            reasoning: None,
                            prompt_token: None,
                            completion_token: None,
                            reasoning_token: None,
                            total_token: None,
                        }
                    })
                });
                Ok(Box::pin(result))
            }
        }
    }
}
