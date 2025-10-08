/* This change is Copyright BEAR LLM AI project, which is proprietory. */
// MIT License Copyright (c) 2024-present Frank Zhang
use super::providers::ollama::{config::OllamaConfig, models::OllamaModels};
use async_openai::{Client};
use serde::Serialize;

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct RemoteModel {
    id: String,
}

pub enum ListModelsRequestExecutor<'c> {
    OllamaListModelsRequestExecutor(&'c Client<OllamaConfig>),
}

impl<'c> ListModelsRequestExecutor<'c> {
    pub fn ollama(client: &'c Client<OllamaConfig>) -> Self {
        return ListModelsRequestExecutor::OllamaListModelsRequestExecutor(client);
    }

    pub async fn execute(&self) -> Result<Vec<RemoteModel>, String> {
        match self {
            ListModelsRequestExecutor::OllamaListModelsRequestExecutor(client) => {
                let response = OllamaModels::new(client).list().await.map_err(|err| {
                    log::error!("OllamaListModelsRequestExecutor: {}", err);
                    String::from("Failed to list models")
                })?;
                let result = response
                    .models
                    .iter()
                    .map(|m| RemoteModel { id: m.name.clone() })
                    .collect();
                Ok(result)
            }
        }
    }
}
