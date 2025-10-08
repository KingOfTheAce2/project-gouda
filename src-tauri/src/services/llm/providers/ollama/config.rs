// MIT License Copyright (c) 2024-present Frank Zhang
use async_openai::config::Config;
use serde::{Deserialize, Serialize};

const OLLAMA_API_BASE: &str = "http://localhost:11434/v1";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OllamaConfig {
    pub api_base: String,
    pub api_key: String,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            api_base: OLLAMA_API_BASE.to_string(),
            api_key: "ollama".to_string(),
        }
    }
}

impl Config for OllamaConfig {
    fn api_base(&self) -> &str {
        &self.api_base
    }

    fn api_key(&self) -> &str {
        &self.api_key
    }

    fn query(&self) -> Vec<(&str, &str)> {
        vec![]
    }

    fn headers(&self) -> std::collections::HashMap<String, String> {
        std::collections::HashMap::new()
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
            api_key: "ollama".to_string(),
        }
    }
}