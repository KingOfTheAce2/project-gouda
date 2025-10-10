// This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
// BEAR LLM AI changes - Removed async_openai dependency, using native structs
// MIT License Copyright (c) 2024-present Frank Zhang
use serde::{Deserialize, Serialize};

const OLLAMA_API_BASE: &str = "http://localhost:11434";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OllamaConfig {
    pub api_base: String,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            api_base: OLLAMA_API_BASE.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RawOllamaConfig {
    pub model: Option<String>,
    pub api_base: Option<String>,
}

impl Into<OllamaConfig> for RawOllamaConfig {
    fn into(self) -> OllamaConfig {
        OllamaConfig {
            api_base: self.api_base.unwrap_or(OLLAMA_API_BASE.to_string()),
        }
    }
}
