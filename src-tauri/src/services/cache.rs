// This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
// MIT License Copyright (c) 2024-present Frank Zhang
use std::collections::HashMap;
use tauri::{
    AppHandle,
    Manager,
    Wry,
};

const PROMPTS_CACHE_KEY: &str = "prompts";

#[derive(Clone, serde::Serialize)]
pub struct CacheChangeEvent<T> {
    key: String,
    value: T,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Prompt {
    pub name: String,
    pub content: String,
}

impl Prompt {
    pub fn new(name: String, content: String) -> Self {
        Self { name, content }
    }
}

#[tauri::command]
pub fn get_prompts_from_cache(handle: AppHandle<Wry>) -> HashMap<String, Vec<Prompt>> {
    let cache = handle.state::<HashMap<String, Vec<Prompt>>>();
    cache.inner().clone()
}

#[tauri::command]
pub fn set_prompts_to_cache(
    prompts: Vec<Prompt>,
    handle: AppHandle<Wry>,
) -> Result<(), String> {
    let mut cache = handle.state::<HashMap<String, Vec<Prompt>>>().inner().clone();
    cache.insert(PROMPTS_CACHE_KEY.to_string(), prompts);
    // Note: Using emit instead of emit_all for compatibility
    // emit broadcasts to all windows, which is the desired behavior
    let _ = handle.emit(
        "prompts_cache_change",
        CacheChangeEvent {
            key: PROMPTS_CACHE_KEY.to_string(),
            value: cache.get(PROMPTS_CACHE_KEY).unwrap(),
        },
    );
    Ok(())
}