// MIT License Copyright (c) 2024-present Frank Zhang
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_log::fern::colors::ColoredLevelConfig;

fn main() {
    // Early dependency check before Tauri initialization
    // This helps diagnose issues before the window is created
    #[cfg(target_os = "windows")]
    {
        use std::io::Write;

        // Use LocalAppData\BEAR LLM AI for all logs (consistent location)
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            let log_dir = std::path::Path::new(&local_app_data).join("BEAR LLM AI");

            // Create directory if it doesn't exist
            let _ = std::fs::create_dir_all(&log_dir);

            let pre_init_log = log_dir.join("preinit.log");

            if let Ok(mut file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&pre_init_log)
            {
                let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                let _ = writeln!(file, "\n[{}] === PRE-INITIALIZATION CHECK ===", timestamp);

                // Check WebView2 runtime
                match bear_llm_ai_lib::crash_handler::check_webview2_runtime() {
                    Ok(msg) => {
                        let _ = writeln!(file, "[{}] ✓ {}", timestamp, msg);
                    }
                    Err(msg) => {
                        let _ = writeln!(file, "[{}] ✗ WARNING: {}", timestamp, msg);
                        let _ = writeln!(file, "[{}] Application may fail to start due to missing WebView2 runtime", timestamp);
                    }
                }

                // Check VC++ Runtime
                match bear_llm_ai_lib::crash_handler::check_vcredist_runtime() {
                    Ok(msg) => {
                        let _ = writeln!(file, "[{}] ✓ {}", timestamp, msg);
                    }
                    Err(msg) => {
                        let _ = writeln!(file, "[{}] ✗ ERROR: {}", timestamp, msg);
                        let _ = writeln!(file, "[{}] ", timestamp);
                        let _ = writeln!(file, "[{}] INSTALLATION REQUIRED:", timestamp);
                        let _ = writeln!(file, "[{}] 1. Download and install BOTH x64 and x86 versions:", timestamp);
                        let _ = writeln!(file, "[{}]    - x64: https://aka.ms/vs/17/release/vc_redist.x64.exe", timestamp);
                        let _ = writeln!(file, "[{}]    - x86: https://aka.ms/vs/17/release/vc_redist.x86.exe", timestamp);
                        let _ = writeln!(file, "[{}] 2. Restart your computer after installation", timestamp);
                        let _ = writeln!(file, "[{}] 3. Try running the application again", timestamp);
                        let _ = writeln!(file, "[{}] ", timestamp);
                        let _ = writeln!(file, "[{}] If the issue persists after installation, please:", timestamp);
                        let _ = writeln!(file, "[{}] - Verify both installers completed successfully", timestamp);
                        let _ = writeln!(file, "[{}] - Check Windows Update for additional updates", timestamp);
                        let _ = writeln!(file, "[{}] - Report the issue with this log file", timestamp);
                    }
                }

                let _ = writeln!(file, "[{}] Pre-initialization check complete. Log: {:?}", timestamp, pre_init_log);
                let _ = writeln!(file, "[{}] Proceeding to Tauri initialization...\n", timestamp);
            }
        }
    }

    // ========================================================================
    // WebView2 User Data Folder Configuration
    // ========================================================================
    // CRITICAL: This MUST run BEFORE tauri::Builder::default() is called.
    // WebView2 initialization happens during Builder creation, so environment
    // variables must be set beforehand.
    //
    // Why this is needed:
    // 1. Prevents WebView2 from using system-wide temp folders (can have permission issues)
    // 2. Isolates application data to %LOCALAPPDATA%\BEAR LLM AI\WebView2
    // 3. Detects and recovers from corrupted WebView2 cache folders
    // 4. Ensures proper write permissions before WebView2 initializes
    // ========================================================================
    #[cfg(target_os = "windows")]
    {
        use std::io::Write;
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            let log_dir = std::path::Path::new(&local_app_data).join("BEAR LLM AI");
            let _ = std::fs::create_dir_all(&log_dir);
            let preinit_log = log_dir.join("preinit.log");

            // Helper function to log messages with timestamps
            let log_msg = |msg: &str| {
                let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                if let Ok(mut file) = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&preinit_log)
                {
                    let _ = writeln!(file, "[{}] {}", timestamp, msg);
                }
            };

            let webview2_dir = log_dir.join("WebView2");

            // Check existing WebView2 folder for corruption or permission issues
            if webview2_dir.exists() {
                log_msg("Existing WebView2 folder detected, verifying integrity...");

                // Perform write test to ensure folder is accessible
                let test_file = webview2_dir.join(".write_test");
                match std::fs::write(&test_file, b"test") {
                    Ok(_) => {
                        let _ = std::fs::remove_file(&test_file);
                        log_msg("✓ WebView2 folder is writable");
                    }
                    Err(e) => {
                        // Permission error detected - recreate folder
                        log_msg(&format!("✗ WebView2 folder permission error: {:?}", e));
                        log_msg("Attempting to recreate WebView2 folder...");
                        let _ = std::fs::remove_dir_all(&webview2_dir);
                    }
                }
            }

            // Create or recreate WebView2 folder with proper permissions
            match std::fs::create_dir_all(&webview2_dir) {
                Ok(_) => {
                    // Set environment variables to direct WebView2 to our custom location
                    // Both variables ensure WebView2 respects our folder choice
                    std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &webview2_dir);
                    std::env::set_var(
                        "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
                        format!("--user-data-dir=\"{}\"", webview2_dir.to_str().unwrap_or(""))
                    );

                    println!("[BEAR LLM AI] WebView2 user data folder set to: {:?}", webview2_dir);
                    log_msg(&format!("✓ WebView2 user data folder configured: {:?}", webview2_dir));
                }
                Err(e) => {
                    // Critical failure - application may crash on startup
                    eprintln!("[BEAR LLM AI] CRITICAL: Failed to create WebView2 folder: {:?}", e);
                    log_msg(&format!("✗ CRITICAL: Cannot create WebView2 folder: {:?}", e));
                }
            }
        }
    }

    // Log start of Tauri initialization to console
    println!("[BEAR LLM AI] Starting Tauri initialization...");

    let context = tauri::generate_context!();
    let log = tauri_plugin_log::Builder::new()
        .level(log::LevelFilter::Info)
        .with_colors(ColoredLevelConfig::default());

    #[cfg(debug_assertions)]
    let log = log.level(log::LevelFilter::Debug);

    println!("[BEAR LLM AI] Building Tauri application...");

    // Build the Tauri application with all plugins and handlers
    let result = tauri::Builder::default()
        .plugin(log.build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            println!("[BEAR LLM AI] Running setup handler...");
            match bear_llm_ai_lib::init::init(app) {
                Ok(_) => {
                    println!("[BEAR LLM AI] Setup completed successfully");
                    Ok(())
                }
                Err(e) => {
                    eprintln!("[BEAR LLM AI] Setup failed: {:?}", e);
                    Err(e)
                }
            }
        })
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
            bear_llm_ai_lib::commands::chat_completions_stream
        ])
        .build(context);

    // Handle build errors
    let app = match result {
        Ok(app) => {
            println!("[BEAR LLM AI] Application built successfully, starting event loop...");
            app
        }
        Err(e) => {
            eprintln!("[BEAR LLM AI] FATAL ERROR during application build: {:?}", e);

            #[cfg(target_os = "windows")]
            {
                use std::io::Write;
                if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
                    let log_dir = std::path::Path::new(&local_app_data).join("BEAR LLM AI");
                    let _ = std::fs::create_dir_all(&log_dir);
                    let error_log = log_dir.join("fatal_error.log");

                    if let Ok(mut file) = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&error_log)
                    {
                        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                        let _ = writeln!(file, "\n[{}] === FATAL ERROR DURING BUILD ===", timestamp);
                        let _ = writeln!(file, "[{}] Error: {:?}", timestamp, e);
                        let _ = writeln!(file, "[{}] ", timestamp);
                        let _ = writeln!(file, "[{}] This error occurred while building the Tauri application.", timestamp);
                        let _ = writeln!(file, "[{}] Common causes:", timestamp);
                        let _ = writeln!(file, "[{}] 1. WebView2 Runtime initialization failed", timestamp);
                        let _ = writeln!(file, "[{}] 2. Window creation failed (check display settings)", timestamp);
                        let _ = writeln!(file, "[{}] 3. Plugin initialization failed", timestamp);
                        let _ = writeln!(file, "[{}] 4. Database initialization failed (check app data folder permissions)", timestamp);
                        let _ = writeln!(file, "[{}] ", timestamp);
                        let _ = writeln!(file, "[{}] Please check the logs above for more details.", timestamp);
                        let _ = writeln!(file, "[{}] Error log: {:?}\n", timestamp, error_log);

                        eprintln!("[BEAR LLM AI] Error details written to: {:?}", error_log);
                    }
                }
            }

            panic!("Error while building Tauri application: {:?}", e);
        }
    };

    // Run the application event loop
    println!("[BEAR LLM AI] Running application event loop...");
    app.run(|_app_handle, event| {
        // Handle application events
        if let tauri::RunEvent::ExitRequested { api, .. } = event {
            api.prevent_exit();
        }
    });
}