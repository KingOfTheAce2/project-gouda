// This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
// BEAR LLM AI changes - Removed unused imports State and Wry
// MIT License Copyright (c) 2024-present Frank Zhang
use serde_json::Value;
use tauri::{
    AppHandle,
    Manager,
};

use crate::{
    core::handle::BearLlmAiHandle,
    errors::BearLlmAiError,
    services::{
        db::Db,
        llm::{
            chat::{BotReply, BotReplyStream, GlobalSettings},
            client::LLMClient,
            models::RemoteModel,
        },
    },
};
use entity::entities::{
    conversations::{self, Conversation, GenericOptions},
    messages::{self, Message, MessageDTO},
    models::{self, Model, Provider},
    prompts::{self, Prompt},
    settings::{self, Setting, SettingKey},
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

// --- Settings
#[tauri::command]
pub async fn get_settings(handle: AppHandle) -> Result<Value, BearLlmAiError> {
    let res = Db::get_settings(&handle.state::<BearLlmAiHandle>().db).await?;
    Ok(res)
}

#[tauri::command]
pub async fn update_settings(
    payload: Vec<Setting>,
    handle: AppHandle,
) -> Result<(), BearLlmAiError> {
    Db::update_settings(&handle.state::<BearLlmAiHandle>().db, payload).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_setting(
    key: SettingKey,
    handle: AppHandle,
) -> Result<Setting, BearLlmAiError> {
    let res = Db::get_setting(&handle.state::<BearLlmAiHandle>().db, key).await?;
    Ok(res)
}

// --- Models
#[tauri::command]
pub async fn get_models(handle: AppHandle) -> Result<Vec<Model>, BearLlmAiError> {
    let res = Db::get_models(&handle.state::<BearLlmAiHandle>().db).await?;
    Ok(res)
}

#[tauri::command]
pub async fn create_model(payload: models::Model, handle: AppHandle) -> Result<Model, BearLlmAiError> {
    let res = Db::create_model(&handle.state::<BearLlmAiHandle>().db, payload).await?;
    Ok(res)
}

#[tauri::command]
pub async fn update_model(
    id: i32,
    payload: models::Model,
    handle: AppHandle,
) -> Result<Model, BearLlmAiError> {
    let res = Db::update_model(&handle.state::<BearLlmAiHandle>().db, id, payload).await?;
    Ok(res)
}

#[tauri::command]
pub async fn delete_model(id: i32, handle: AppHandle) -> Result<(), BearLlmAiError> {
    Db::delete_model(&handle.state::<BearLlmAiHandle>().db, id).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_providers(handle: AppHandle) -> Result<Vec<Provider>, BearLlmAiError> {
    let res = Db::get_providers(&handle.state::<BearLlmAiHandle>().db).await?;
    Ok(res)
}

#[tauri::command]
pub async fn list_remote_models(
    model_id: i32,
    handle: AppHandle,
) -> Result<Vec<RemoteModel>, String> {
    let bear_llm_ai_handle = handle.state::<BearLlmAiHandle>();
    let model = Db::get_model(&bear_llm_ai_handle.db, model_id)
        .await
        .map_err(|err| err.to_string())?;
    let proxy_setting = Db::get_proxy_setting(&bear_llm_ai_handle.db)
        .await
        .map_err(|err| err.to_string())?;
    let client = LLMClient::new(model.into(), proxy_setting)?;
    let models = client.models().await?;
    Ok(models)
}

// --- Conversations
#[tauri::command]
pub async fn get_conversations(handle: AppHandle) -> Result<Vec<Conversation>, BearLlmAiError> {
    let res = Db::get_conversations(&handle.state::<BearLlmAiHandle>().db).await?;
    Ok(res)
}

#[tauri::command]
pub async fn create_conversation(
    payload: conversations::Model,
    handle: AppHandle,
) -> Result<Conversation, BearLlmAiError> {
    let res = Db::create_conversation(&handle.state::<BearLlmAiHandle>().db, payload).await?;
    Ok(res)
}

#[tauri::command]
pub async fn update_conversation(
    id: i32,
    payload: conversations::Model,
    handle: AppHandle,
) -> Result<Conversation, BearLlmAiError> {
    let res = Db::update_conversation(&handle.state::<BearLlmAiHandle>().db, id, payload).await?;
    Ok(res)
}

#[tauri::command]
pub async fn delete_conversation(id: i32, handle: AppHandle) -> Result<(), BearLlmAiError> {
    Db::delete_conversation(&handle.state::<BearLlmAiHandle>().db, id).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_conversation_messages(
    conversation_id: i32,
    handle: AppHandle,
) -> Result<Vec<Message>, BearLlmAiError> {
    let res = Db::get_conversation_messages(&handle.state::<BearLlmAiHandle>().db, conversation_id).await?;
    Ok(res)
}

// --- Messages
#[tauri::command]
pub async fn create_messages(
    payload: Vec<messages::Model>,
    handle: AppHandle,
) -> Result<Vec<Message>, BearLlmAiError> {
    let res = Db::create_messages(&handle.state::<BearLlmAiHandle>().db, payload).await?;
    Ok(res)
}

// --- Prompts
#[tauri::command]
pub async fn get_prompts(handle: AppHandle) -> Result<Vec<Prompt>, BearLlmAiError> {
    let res = Db::get_prompts(&handle.state::<BearLlmAiHandle>().db).await?;
    Ok(res)
}

#[tauri::command]
pub async fn create_prompt(payload: prompts::Model, handle: AppHandle) -> Result<Prompt, BearLlmAiError> {
    let res = Db::create_prompt(&handle.state::<BearLlmAiHandle>().db, payload).await?;
    Ok(res)
}

#[tauri::command]
pub async fn update_prompt(
    id: i32,
    payload: prompts::Model,
    handle: AppHandle,
) -> Result<Prompt, BearLlmAiError> {
    let res = Db::update_prompt(&handle.state::<BearLlmAiHandle>().db, id, payload).await?;
    Ok(res)
}

#[tauri::command]
pub async fn delete_prompt(id: i32, handle: AppHandle) -> Result<(), BearLlmAiError> {
    Db::delete_prompt(&handle.state::<BearLlmAiHandle>().db, id).await?;
    Ok(())
}

// --- Chat
#[tauri::command]
pub async fn chat_completions(
    model_id: i32,
    messages: Vec<MessageDTO>,
    options: GenericOptions,
    handle: AppHandle,
) -> Result<BotReply, String> {
    let bear_llm_ai_handle = handle.state::<BearLlmAiHandle>();
    let model = Db::get_model(&bear_llm_ai_handle.db, model_id)
        .await
        .map_err(|err| err.to_string())?;
    let proxy_setting = Db::get_proxy_setting(&bear_llm_ai_handle.db)
        .await
        .map_err(|err| err.to_string())?;
    let client = LLMClient::new(model.into(), proxy_setting)?;
    let global_settings = GlobalSettings {
        max_tokens: settings::get_max_tokens(&bear_llm_ai_handle.db)
            .await
            .map_err(|err| err.to_string())?,
    };
    let reply = client.chat(messages, options, global_settings).await?;
    Ok(reply)
}

#[tauri::command]
pub async fn chat_completions_stream(
    model_id: i32,
    messages: Vec<MessageDTO>,
    options: GenericOptions,
    handle: AppHandle,
) -> Result<BotReplyStream, String> {
    let bear_llm_ai_handle = handle.state::<BearLlmAiHandle>();
    let model = Db::get_model(&bear_llm_ai_handle.db, model_id)
        .await
        .map_err(|err| err.to_string())?;
    let proxy_setting = Db::get_proxy_setting(&bear_llm_ai_handle.db)
        .await
        .map_err(|err| err.to_string())?;
    let client = LLMClient::new(model.into(), proxy_setting)?;
    let global_settings = GlobalSettings {
        max_tokens: settings::get_max_tokens(&bear_llm_ai_handle.db)
            .await
            .map_err(|err| err.to_string())?,
    };
    let stream = client.chat_stream(messages, options, global_settings).await?;
    Ok(stream)
}
