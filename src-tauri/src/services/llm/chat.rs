/* This change is Copyright BEAR LLM AI project, which is proprietory. */
// MIT License Copyright (c) 2024-present Frank Zhang
use std::pin::Pin;

use crate::log_utils::warn;
use async_openai::{
    error::OpenAIError,
    Client,
};
use entity::entities::{
    conversations::{GenericOptions, OllamaOptions},
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
        }
    }
}
