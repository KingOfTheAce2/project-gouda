// MIT License Copyright (c) 2024-present Frank Zhang
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_log::fern::colors::ColoredLevelConfig;

fn main() {
    let context = tauri::generate_context!();
    let log = tauri_plugin_log::Builder::new()
        .level(log::LevelFilter::Info)
        .with_colors(ColoredLevelConfig::default());

    #[cfg(debug_assertions)]
    let log = log.level(log::LevelFilter::Debug);

    tauri::Builder::default()
        .plugin(log.build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
            .setup(bear_llm_ai_lib::init::init)
              .invoke_handler(tauri::generate_handler![
                bear_llm_ai_lib::commands::get_settings,
                bear_llm_ai_lib::commands::update_settings,
                bear_llm_ai_lib::commands::get_setting,
                bear_llm_ai_lib::commands::get_models,
                bear_llm_ai_lib::commands::create_model,
                bear_llm_ai_lib::commands::update_model,
                bear_llm_ai_lib::commands::delete_model,
                bear_llm_ai_lib::commands::get_providers,
                bear_llm_ai_lib::commands::list_remote_models,
                bear_llm_ai_lib::commands::get_conversations,
                bear_llm_ai_lib::commands::create_conversation,
                bear_llm_ai_lib::commands::update_conversation,
                bear_llm_ai_lib::commands::delete_conversation,
                bear_llm_ai_lib::commands::get_conversation_messages,
                bear_llm_ai_lib::commands::create_messages,
                bear_llm_ai_lib::commands::get_prompts,
                bear_llm_ai_lib::commands::create_prompt,
                bear_llm_ai_lib::commands::update_prompt,
                bear_llm_ai_lib::commands::delete_prompt,
                bear_llm_ai_lib::commands::chat_completions,
                bear_llm_ai_lib::commands::chat_completions_stream        ])
        .run(context)
        .expect("error while running tauri application");
}