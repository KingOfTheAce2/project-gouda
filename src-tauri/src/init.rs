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
use std::path::PathBuf;

/// Initialize WebView2 user data folder with proper permissions for current user
#[cfg(target_os = "windows")]
fn setup_webview2_user_data_folder(app_data_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    use std::os::windows::fs::MetadataExt;

    // Create WebView2 user data folder path
    let webview2_dir = app_data_dir.join("EBWebView");

    log::info!("Setting up WebView2 user data folder at: {:?}", webview2_dir);

    // Ensure the directory exists with proper permissions
    if !webview2_dir.exists() {
        std::fs::create_dir_all(&webview2_dir)?;
        log::info!("Created WebView2 user data folder");
    } else {
        // Check if we can write to the existing directory
        match std::fs::metadata(&webview2_dir) {
            Ok(metadata) => {
                // Try to create a test file to verify write permissions
                let test_file = webview2_dir.join(".write_test");
                match std::fs::write(&test_file, b"test") {
                    Ok(_) => {
                        let _ = std::fs::remove_file(&test_file);
                        log::info!("WebView2 folder permissions verified");
                    }
                    Err(e) => {
                        log::warn!("WebView2 folder exists but is not writable: {:?}", e);
                        log::info!("Attempting to recreate WebView2 folder...");

                        // Try to remove and recreate the folder
                        if let Err(remove_err) = std::fs::remove_dir_all(&webview2_dir) {
                            log::error!("Failed to remove existing WebView2 folder: {:?}", remove_err);
                            return Err(format!(
                                "WebView2 folder exists but cannot be accessed. Please delete: {:?}",
                                webview2_dir
                            ).into());
                        }

                        std::fs::create_dir_all(&webview2_dir)?;
                        log::info!("Successfully recreated WebView2 folder");
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to read WebView2 folder metadata: {:?}", e);
                return Err(format!("Cannot access WebView2 folder: {:?}", e).into());
            }
        }
    }

    // Set environment variable for WebView2 to use this folder
    std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &webview2_dir);
    log::info!("Set WEBVIEW2_USER_DATA_FOLDER environment variable");

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn setup_webview2_user_data_folder(_app_data_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // No-op on non-Windows platforms
    Ok(())
}

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

        log::info!("App data directory: {:?}", app_data_dir);

        // Ensure the directory exists
        if !app_data_dir.exists() {
            std::fs::create_dir_all(&app_data_dir)
                .map_err(|e| {
                    eprintln!("Failed to create app data directory: {:?}", e);
                    format!("Cannot create app data directory: {:?}", e)
                })?;
            log::info!("Created app data directory");
        }

        // Setup WebView2 user data folder with proper permissions (Windows only)
        setup_webview2_user_data_folder(&app_data_dir)
            .map_err(|e| {
                eprintln!("Failed to setup WebView2 user data folder: {:?}", e);
                format!("WebView2 initialization failed: {:?}", e)
            })?;

        // Initialize database
        let db_wrapper = Db::new(&app_data_dir).await;
        Ok::<_, String>(db_wrapper.0)
    })?;

    handle.manage(BearLlmAiHandle { db });
    Ok(())
}