// This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
// BEAR LLM AI changes - Added Db import
// MIT License Copyright (c) 2024-present Frank Zhang
use crate::core::handle::BearLlmAiHandle;
use crate::services::db::Db;
use crate::crash_handler;
use tauri::{
    App,
    Manager,
    Wry,
};
use std::path::PathBuf;


pub fn init(app: &mut App<Wry>) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Starting Tauri application initialization...");
    let handle = app.handle();

    let db = tauri::async_runtime::block_on(async {
        log::info!("Starting async initialization block...");

        // Get app data directory with better error handling
        let app_data_dir = handle
            .path()
            .app_data_dir()
            .map_err(|e| {
                log::error!("Failed to resolve app data directory: {:?}", e);
                format!("Cannot access app data directory: {:?}", e)
            })?;

        log::info!("App data directory: {:?}", app_data_dir);

        // Ensure the directory exists
        if !app_data_dir.exists() {
            log::info!("App data directory does not exist, creating it...");
            std::fs::create_dir_all(&app_data_dir)
                .map_err(|e| {
                    log::error!("Failed to create app data directory: {:?}", e);
                    format!("Cannot create app data directory: {:?}", e)
                })?;
            log::info!("Created app data directory");
        } else {
            log::info!("App data directory already exists");
        }

        // Initialize crash handler ASAP
        log::info!("Initializing crash handler...");
        crash_handler::init_crash_handler(&app_data_dir);
        log::info!("Crash handler initialized");

        // Run dependency diagnostics and log results
        log::info!("Running dependency diagnostics...");
        crash_handler::run_dependency_diagnostics(&app_data_dir);
        log::info!("Dependency diagnostics complete");

        // WebView2 user data folder is now set in main.rs BEFORE Tauri initialization
        // This section verifies the setup and logs additional diagnostics
        #[cfg(target_os = "windows")]
        {
            if let Ok(webview2_path) = std::env::var("WEBVIEW2_USER_DATA_FOLDER") {
                log::info!("WebView2 user data folder already configured: {}", webview2_path);

                // Verify the folder is writable
                let webview2_dir = std::path::PathBuf::from(&webview2_path);
                let test_file = webview2_dir.join(".write_test");
                match std::fs::write(&test_file, b"test") {
                    Ok(_) => {
                        let _ = std::fs::remove_file(&test_file);
                        log::info!("✓ WebView2 folder is writable");
                    }
                    Err(e) => {
                        log::error!("✗ WebView2 folder is not writable: {:?}", e);
                        log::error!("This may cause the application to fail. Please check folder permissions.");
                    }
                }
            } else {
                log::warn!("WEBVIEW2_USER_DATA_FOLDER environment variable not set");
                log::warn!("WebView2 will use system default location");
            }
        }

        // Initialize database
        log::info!("Initializing database...");
        let db_wrapper = Db::new(&app_data_dir)
            .await
            .map_err(|e| {
                log::error!("Database initialization failed: {:?}", e);
                format!("Failed to initialize database: {:?}", e)
            })?;

        log::info!("Database initialization complete");
        Ok::<_, String>(db_wrapper.0)
    })?;

    log::info!("Managing application state...");
    handle.manage(BearLlmAiHandle { db });
    log::info!("Tauri application initialization complete");
    Ok(())
}