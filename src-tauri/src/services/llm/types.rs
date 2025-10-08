// MIT License Copyright (c) 2024-present Frank Zhang
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RawOllamaConfig {
    pub model: Option<String>,
    pub api_base: Option<String>,
}
