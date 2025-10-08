// MIT License Copyright (c) 2024-present Frank Zhang
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_log::fern::colors::ColoredLevelConfig;

fn main() {
    let mut context = tauri::generate_context!();
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
        .setup(kaas_lib::init::init)
        .invoke_handler(tauri::generate_handler![
            kaas_lib::commands::get_settings,
            kaas_lib::commands::update_settings,
            kaas_lib::commands::get_setting,
            kaas_lib::commands::get_models,
            kaas_lib::commands::create_model,
            kaas_lib::commands::update_model,
            kaas_lib::commands::delete_model,
            kaas_lib::commands::get_providers,
            kaas_lib::commands::list_remote_models,
            kaas_lib::commands::get_conversations,
            kaas_lib::commands::create_conversation,
            kaas_lib::commands::update_conversation,
            kaas_lib::commands::delete_conversation,
            kaas_lib::commands::get_conversation_messages,
            kaas_lib::commands::create_messages,
            kaas_lib::commands::get_prompts,
            kaas_lib::commands::create_prompt,
            kaas_lib::commands::update_prompt,
            kaas_lib::commands::delete_prompt,
            kaas_lib::commands::chat_completions,
            kaas_lib::commands::chat_completions_stream
        ])
        .run(context)
        .expect("error while running tauri application");
}