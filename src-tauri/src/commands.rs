// MIT License Copyright (c) 2024-present Frank Zhang
use serde_json::Value;
use tauri::{
    AppHandle,
    Manager,
    State,
    Wry,
};

use crate::{
    core::handle::KaasHandle,
    errors::KaasError,
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
pub async fn get_settings(handle: AppHandle) -> Result<Value, KaasError> {
    let res = Db::get_settings(&handle.state::<KaasHandle>().db).await?;
    Ok(res)
}

#[tauri::command]
pub async fn update_settings(
    payload: Vec<Setting>,
    handle: AppHandle,
) -> Result<(), KaasError> {
    Db::update_settings(&handle.state::<KaasHandle>().db, payload).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_setting(
    key: SettingKey,
    handle: AppHandle,
) -> Result<Setting, KaasError> {
    let res = Db::get_setting(&handle.state::<KaasHandle>().db, key).await?;
    Ok(res)
}

// --- Models
#[tauri::command]
pub async fn get_models(handle: AppHandle) -> Result<Vec<Model>, KaasError> {
    let res = Db::get_models(&handle.state::<KaasHandle>().db).await?;
    Ok(res)
}

#[tauri::command]
pub async fn create_model(payload: models::Model, handle: AppHandle) -> Result<Model, KaasError> {
    let res = Db::create_model(&handle.state::<KaasHandle>().db, payload).await?;
    Ok(res)
}

#[tauri::command]
pub async fn update_model(
    id: i32,
    payload: models::Model,
    handle: AppHandle,
) -> Result<Model, KaasError> {
    let res = Db::update_model(&handle.state::<KaasHandle>().db, id, payload).await?;
    Ok(res)
}

#[tauri::command]
pub async fn delete_model(id: i32, handle: AppHandle) -> Result<(), KaasError> {
    Db::delete_model(&handle.state::<KaasHandle>().db, id).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_providers(handle: AppHandle) -> Result<Vec<Provider>, KaasError> {
    let res = Db::get_providers(&handle.state::<KaasHandle>().db).await?;
    Ok(res)
}

#[tauri::command]
pub async fn list_remote_models(
    model_id: i32,
    handle: AppHandle,
) -> Result<Vec<RemoteModel>, String> {
    let kaas_handle = handle.state::<KaasHandle>();
    let model = Db::get_model(&kaas_handle.db, model_id)
        .await
        .map_err(|err| err.to_string())?;
    let proxy_setting = Db::get_proxy_setting(&kaas_handle.db)
        .await
        .map_err(|err| err.to_string())?;
    let client = LLMClient::new(model.into(), proxy_setting)?;
    let models = client.models().await?;
    Ok(models)
}

// --- Conversations
#[tauri::command]
pub async fn get_conversations(handle: AppHandle) -> Result<Vec<Conversation>, KaasError> {
    let res = Db::get_conversations(&handle.state::<KaasHandle>().db).await?;
    Ok(res)
}

#[tauri::command]
pub async fn create_conversation(
    payload: conversations::Model,
    handle: AppHandle,
) -> Result<Conversation, KaasError> {
    let res = Db::create_conversation(&handle.state::<KaasHandle>().db, payload).await?;
    Ok(res)
}

#[tauri::command]
pub async fn update_conversation(
    id: i32,
    payload: conversations::Model,
    handle: AppHandle,
) -> Result<Conversation, KaasError> {
    let res = Db::update_conversation(&handle.state::<KaasHandle>().db, id, payload).await?;
    Ok(res)
}

#[tauri::command]
pub async fn delete_conversation(id: i32, handle: AppHandle) -> Result<(), KaasError> {
    Db::delete_conversation(&handle.state::<KaasHandle>().db, id).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_conversation_messages(
    conversation_id: i32,
    handle: AppHandle,
) -> Result<Vec<Message>, KaasError> {
    let res = Db::get_conversation_messages(&handle.state::<KaasHandle>().db, conversation_id).await?;
    Ok(res)
}

// --- Messages
#[tauri::command]
pub async fn create_messages(
    payload: Vec<messages::Model>,
    handle: AppHandle,
) -> Result<Vec<Message>, KaasError> {
    let res = Db::create_messages(&handle.state::<KaasHandle>().db, payload).await?;
    Ok(res)
}

// --- Prompts
#[tauri::command]
pub async fn get_prompts(handle: AppHandle) -> Result<Vec<Prompt>, KaasError> {
    let res = Db::get_prompts(&handle.state::<KaasHandle>().db).await?;
    Ok(res)
}

#[tauri::command]
pub async fn create_prompt(payload: prompts::Model, handle: AppHandle) -> Result<Prompt, KaasError> {
    let res = Db::create_prompt(&handle.state::<KaasHandle>().db, payload).await?;
    Ok(res)
}

#[tauri::command]
pub async fn update_prompt(
    id: i32,
    payload: prompts::Model,
    handle: AppHandle,
) -> Result<Prompt, KaasError> {
    let res = Db::update_prompt(&handle.state::<KaasHandle>().db, id, payload).await?;
    Ok(res)
}

#[tauri::command]
pub async fn delete_prompt(id: i32, handle: AppHandle) -> Result<(), KaasError> {
    Db::delete_prompt(&handle.state::<KaasHandle>().db, id).await?;
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
    let kaas_handle = handle.state::<KaasHandle>();
    let model = Db::get_model(&kaas_handle.db, model_id)
        .await
        .map_err(|err| err.to_string())?;
    let proxy_setting = Db::get_proxy_setting(&kaas_handle.db)
        .await
        .map_err(|err| err.to_string())?;
    let client = LLMClient::new(model.into(), proxy_setting)?;
    let global_settings = GlobalSettings {
        max_tokens: settings::get_max_tokens(&kaas_handle.db)
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
    let kaas_handle = handle.state::<KaasHandle>();
    let model = Db::get_model(&kaas_handle.db, model_id)
        .await
        .map_err(|err| err.to_string())?;
    let proxy_setting = Db::get_proxy_setting(&kaas_handle.db)
        .await
        .map_err(|err| err.to_string())?;
    let client = LLMClient::new(model.into(), proxy_setting)?;
    let global_settings = GlobalSettings {
        max_tokens: settings::get_max_tokens(&kaas_handle.db)
            .await
            .map_err(|err| err.to_string())?,
    };
    let stream = client.chat_stream(messages, options, global_settings).await?;
    Ok(stream)
}