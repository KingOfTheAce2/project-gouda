// This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
// MIT License Copyright (c) 2024-present Frank Zhang
use serde::{Deserialize, Serialize};
use super::providers::ollama::config::OllamaConfig;

const OLLAMA_API_BASE: &str = "http://localhost:11434";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RawOllamaConfig {
    pub model: Option<String>,
    pub api_base: Option<String>,
}

impl From<RawOllamaConfig> for OllamaConfig {
    fn from(raw: RawOllamaConfig) -> Self {
        OllamaConfig {
            api_base: raw.api_base.unwrap_or(OLLAMA_API_BASE.to_string()),
        }
    }
}
