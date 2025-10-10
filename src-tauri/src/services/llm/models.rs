// This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
// BEAR LLM AI changes - Removed async_openai dependency, using direct Ollama config
// MIT License Copyright (c) 2024-present Frank Zhang
use super::providers::ollama::{config::OllamaConfig, models::OllamaModels};
use serde::Serialize;

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct RemoteModel {
    id: String,
}

pub enum ListModelsRequestExecutor {
    OllamaListModelsRequestExecutor(OllamaConfig),
}

impl ListModelsRequestExecutor {
    pub fn ollama(config: &OllamaConfig) -> Self {
        return ListModelsRequestExecutor::OllamaListModelsRequestExecutor(config.clone());
    }

    pub async fn execute(&self) -> Result<Vec<RemoteModel>, String> {
        match self {
            ListModelsRequestExecutor::OllamaListModelsRequestExecutor(config) => {
                let response = OllamaModels::new(config.clone()).list().await.map_err(|err| {
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
