// This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
// BEAR LLM AI changes - Added Db import
// MIT License Copyright (c) 2024-present Frank Zhang
use crate::core::handle::BearLlmAiHandle;
use crate::services::db::Db;
use tauri::{
    App,
    Manager,
    Wry,
};

pub fn init(app: &mut App<Wry>) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    let db = tauri::async_runtime::block_on(async {
        // Get app data directory with better error handling
        let app_data_dir = handle
            .path()
            .app_data_dir()
            .map_err(|e| {
                eprintln!("Failed to resolve app data directory: {:?}", e);
                format!("Cannot access app data directory: {:?}", e)
            })?;

        // Ensure the directory exists
        if !app_data_dir.exists() {
            std::fs::create_dir_all(&app_data_dir)
                .map_err(|e| {
                    eprintln!("Failed to create app data directory: {:?}", e);
                    format!("Cannot create app data directory: {:?}", e)
                })?;
        }

        // Initialize database
        let db_wrapper = Db::new(&app_data_dir).await;
        Ok::<_, String>(db_wrapper.0)
    })?;

    handle.manage(BearLlmAiHandle { db });
    Ok(())
}