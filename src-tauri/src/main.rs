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

    // Setup WebView2 user data folder BEFORE Tauri initialization
    // This ensures the environment variable is set before WebView2 is initialized
    #[cfg(target_os = "windows")]
    {
        use std::io::Write;
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            let log_dir = std::path::Path::new(&local_app_data).join("BEAR LLM AI");
            let _ = std::fs::create_dir_all(&log_dir);

            // Setup WebView2 user data folder
            let webview2_dir = log_dir.join("WebView2");
            if let Err(e) = std::fs::create_dir_all(&webview2_dir) {
                eprintln!("[BEAR LLM AI] Failed to create WebView2 folder: {:?}", e);
            } else {
                std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &webview2_dir);
                println!("[BEAR LLM AI] WebView2 user data folder set to: {:?}", webview2_dir);

                // Log to preinit.log
                if let Ok(mut file) = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(log_dir.join("preinit.log"))
                {
                    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                    let _ = writeln!(file, "[{}] ✓ WebView2 user data folder configured: {:?}", timestamp, webview2_dir);
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
    let run_result = app.run(|_app_handle, event| {
        // Handle application events
        if let tauri::RunEvent::ExitRequested { api, .. } = event {
            api.prevent_exit();
        }
    });

    // If run() fails, write error to file before exiting
    if let Err(e) = run_result {
        eprintln!("[BEAR LLM AI] FATAL ERROR during event loop: {:?}", e);

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
                    let _ = writeln!(file, "\n[{}] === FATAL ERROR DURING EVENT LOOP ===", timestamp);
                    let _ = writeln!(file, "[{}] Error: {:?}", timestamp, e);
                    let _ = writeln!(file, "[{}] ", timestamp);
                    let _ = writeln!(file, "[{}] TROUBLESHOOTING STEPS:", timestamp);
                    let _ = writeln!(file, "[{}] 1. Check if WebView2 Runtime is installed (see preinit.log)", timestamp);
                    let _ = writeln!(file, "[{}] 2. Check if Visual C++ Runtime is installed (see preinit.log)", timestamp);
                    let _ = writeln!(file, "[{}] 3. Verify app data directory is accessible: {:?}", timestamp, log_dir);
                    let _ = writeln!(file, "[{}] 4. Check disk space availability", timestamp);
                    let _ = writeln!(file, "[{}] 5. Run as administrator if permission issues persist", timestamp);
                    let _ = writeln!(file, "[{}] 6. Try deleting the WebView2 folder and restarting", timestamp);
                    let _ = writeln!(file, "[{}] ", timestamp);
                    let _ = writeln!(file, "[{}] Error log: {:?}\n", timestamp, error_log);

                    eprintln!("[BEAR LLM AI] Error details written to: {:?}", error_log);
                }
            }
        }

        panic!("Error while running Tauri application: {:?}", e);
    }

    println!("[BEAR LLM AI] Application exited successfully");
}