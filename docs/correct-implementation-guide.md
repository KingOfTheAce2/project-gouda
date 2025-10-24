# Correct WebView2 Implementation Guide

**Analyst Agent - Hive Mind Collective**
**Generated:** 2025-10-24
**Based On:** Working implementation (commit 6bf6326)

---

## Table of Contents

1. [Introduction](#introduction)
2. [Core Principles](#core-principles)
3. [Implementation Phases](#implementation-phases)
4. [Code Templates](#code-templates)
5. [Configuration Reference](#configuration-reference)
6. [Error Handling Patterns](#error-handling-patterns)
7. [Testing Strategy](#testing-strategy)
8. [Common Pitfalls](#common-pitfalls)

---

## Introduction

This guide documents the **CORRECT** approach to implementing WebView2 initialization in a Tauri application, based on the working solution that resolved persistent initialization failures.

### What Makes This Implementation Correct?

1. **Timing:** WebView2 setup occurs BEFORE Tauri initialization
2. **Error Propagation:** All errors return `Result<>` instead of panicking
3. **Window Management:** Window remains hidden until initialization succeeds
4. **Diagnostics:** Comprehensive pre-flight checks and logging
5. **Separation of Concerns:** Build errors vs. runtime errors handled separately

---

## Core Principles

### Principle 1: Setup Before Initialization

**Rule:** Environment variables must be set BEFORE the component that uses them initializes.

```rust
// ✅ CORRECT
fn main() {
    // 1. Set environment variable
    std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &folder);

    // 2. THEN initialize Tauri (which initializes WebView2)
    let app = tauri::Builder::default().build(context);
}

// ❌ WRONG
fn main() {
    let app = tauri::Builder::default()
        .setup(|app| {
            // 1. Tauri already initialized WebView2
            // 2. Setting env var NOW is too late!
            std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &folder);
            Ok(())
        })
        .build(context);
}
```

**Why:** WebView2 reads `WEBVIEW2_USER_DATA_FOLDER` during initialization. Setting it afterward has no effect.

---

### Principle 2: Propagate Errors, Don't Panic

**Rule:** Use `Result<>` return types and `.map_err()` for error context, not `.expect()` or `.unwrap()`.

```rust
// ✅ CORRECT
pub async fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
    let conn = Database::connect(&db_url)
        .await
        .map_err(|e| {
            log::error!("Failed to connect to database: {:?}", e);
            format!("Database connection failed: {:?}", e)
        })?;

    Ok(Self(conn))
}

// ❌ WRONG
pub async fn new(path: &Path) -> Self {
    let conn = Database::connect(&db_url)
        .await
        .expect("failed to connect to database");  // PANIC - no recovery!

    Self(conn)
}
```

**Why:** Panics cannot be caught or logged properly in setup handlers. `Result<>` allows graceful error handling and informative error messages.

---

### Principle 3: Hide Window Until Ready

**Rule:** Start with window hidden, show only after successful initialization.

```json
// tauri.conf.json
{
  "app": {
    "windows": [{
      "visible": false  // ✅ Start hidden
    }]
  }
}
```

```rust
// init.rs
pub fn init(app: &mut App<Wry>) -> Result<...> {
    // All initialization logic...

    // Show window ONLY after success
    if let Some(window) = app.get_webview_window("main") {
        window.show()?;
        window.set_focus()?;
    }

    Ok(())
}
```

**Why:** Prevents blank/frozen window from appearing if initialization fails.

---

### Principle 4: Separate Build and Run Errors

**Rule:** Handle `.build()` errors separately from `.run()` errors.

```rust
// ✅ CORRECT
let result = tauri::Builder::default()
    .setup(|app| { /* ... */ })
    .build(context);

let app = match result {
    Ok(app) => app,
    Err(e) => {
        eprintln!("BUILD ERROR: {:?}", e);
        // Log to fatal_error.log
        panic!("Error while building Tauri application: {:?}", e);
    }
};

app.run(|_app_handle, event| {
    // RUN-TIME event handling
});

// ❌ WRONG
let result = tauri::Builder::default()
    .setup(|app| { /* ... */ })
    .run(context);  // Build and run errors mixed

if let Err(e) = result {
    eprintln!("ERROR: {:?}", e);  // Cannot distinguish build from run error
}
```

**Why:** Clearer error messages, better debugging, distinct failure modes.

---

### Principle 5: Comprehensive Pre-Flight Checks

**Rule:** Check dependencies BEFORE attempting initialization.

```rust
fn main() {
    #[cfg(target_os = "windows")]
    {
        // Check WebView2 runtime
        match check_webview2_runtime() {
            Ok(msg) => log_preinit(&log_dir, &format!("✓ {}", msg)),
            Err(msg) => log_preinit(&log_dir, &format!("✗ WARNING: {}", msg)),
        }

        // Check VC++ Runtime
        match check_vcredist_runtime() {
            Ok(msg) => log_preinit(&log_dir, &format!("✓ {}", msg)),
            Err(msg) => log_preinit(&log_dir, &format!("✗ ERROR: {}", msg)),
        }
    }

    // Now safe to proceed with Tauri initialization
}
```

**Why:** Early detection of missing dependencies with helpful error messages before initialization fails.

---

## Implementation Phases

### Phase 1: Pre-Initialization (main.rs)

**Timing:** BEFORE `tauri::Builder::default()`

**Responsibilities:**
1. Check system dependencies (WebView2, VC++ Runtime)
2. Create log directories
3. Log pre-initialization status
4. Set up WebView2 user data folder
5. Set environment variables

**Code Template:**

```rust
fn main() {
    // PHASE 1: PRE-INITIALIZATION
    #[cfg(target_os = "windows")]
    {
        use std::io::Write;

        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            let log_dir = std::path::Path::new(&local_app_data).join("BEAR LLM AI");
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
                        let _ = writeln!(file, "[{}] Application may fail to start", timestamp);
                    }
                }

                // Check VC++ Runtime
                match bear_llm_ai_lib::crash_handler::check_vcredist_runtime() {
                    Ok(msg) => {
                        let _ = writeln!(file, "[{}] ✓ {}", timestamp, msg);
                    }
                    Err(msg) => {
                        let _ = writeln!(file, "[{}] ✗ ERROR: {}", timestamp, msg);
                        // Include installation instructions...
                    }
                }

                let _ = writeln!(file, "[{}] Pre-initialization check complete\n", timestamp);
            }
        }
    }

    // Setup WebView2 user data folder BEFORE Tauri initialization
    #[cfg(target_os = "windows")]
    {
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            let log_dir = std::path::Path::new(&local_app_data).join("BEAR LLM AI");
            let _ = std::fs::create_dir_all(&log_dir);

            let webview2_dir = log_dir.join("WebView2");

            // Create WebView2 folder
            if let Err(e) = std::fs::create_dir_all(&webview2_dir) {
                eprintln!("[APP] Failed to create WebView2 folder: {:?}", e);
            } else {
                // Set environment variable BEFORE Tauri initialization
                std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &webview2_dir);
                println!("[APP] WebView2 user data folder set to: {:?}", webview2_dir);
            }
        }
    }

    println!("[APP] Starting Tauri initialization...");

    // NOW proceed to Phase 2: Tauri Build
}
```

**Key Points:**
- ✅ Runs BEFORE Tauri initialization
- ✅ Logs to file (logging framework not available yet)
- ✅ Sets environment variables
- ✅ Non-fatal errors (warnings only)

---

### Phase 2: Tauri Build (main.rs)

**Timing:** After pre-initialization, before `.run()`

**Responsibilities:**
1. Create Tauri Builder
2. Add plugins
3. Configure setup handler
4. Build application (separate from running)

**Code Template:**

```rust
fn main() {
    // ... Phase 1 code above ...

    println!("[APP] Building Tauri application...");

    let context = tauri::generate_context!();
    let log = tauri_plugin_log::Builder::new()
        .level(log::LevelFilter::Info)
        .with_colors(ColoredLevelConfig::default());

    #[cfg(debug_assertions)]
    let log = log.level(log::LevelFilter::Debug);

    // PHASE 2: TAURI BUILD
    let result = tauri::Builder::default()
        .plugin(log.build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            println!("[APP] Running setup handler...");
            match bear_llm_ai_lib::init::init(app) {
                Ok(_) => {
                    println!("[APP] Setup completed successfully");
                    Ok(())
                }
                Err(e) => {
                    eprintln!("[APP] Setup failed: {:?}", e);
                    Err(e)
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            // ... command handlers ...
        ])
        .build(context);  // BUILD (not run yet)

    // Proceed to Phase 3: Error Handling
}
```

**Key Points:**
- ✅ Uses closure for setup (allows error logging)
- ✅ Calls `.build()` not `.run()`
- ✅ Logs progress with `println!`
- ✅ Propagates errors from setup handler

---

### Phase 3: Build Error Handling (main.rs)

**Timing:** After `.build()`, before `.run()`

**Responsibilities:**
1. Handle build errors separately
2. Log detailed error information
3. Provide troubleshooting guidance
4. Exit gracefully or panic with context

**Code Template:**

```rust
fn main() {
    // ... Phase 1 and 2 code above ...

    // PHASE 3: BUILD ERROR HANDLING
    let app = match result {
        Ok(app) => {
            println!("[APP] Application built successfully, starting event loop...");
            app
        }
        Err(e) => {
            eprintln!("[APP] FATAL ERROR during application build: {:?}", e);

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
                        let _ = writeln!(file, "[{}] Common causes:", timestamp);
                        let _ = writeln!(file, "[{}] 1. WebView2 Runtime initialization failed", timestamp);
                        let _ = writeln!(file, "[{}] 2. Window creation failed (check display settings)", timestamp);
                        let _ = writeln!(file, "[{}] 3. Plugin initialization failed", timestamp);
                        let _ = writeln!(file, "[{}] 4. Database initialization failed (check permissions)", timestamp);
                        let _ = writeln!(file, "[{}] ", timestamp);
                        let _ = writeln!(file, "[{}] Please check the logs for details.", timestamp);
                        let _ = writeln!(file, "[{}] Error log: {:?}\n", timestamp, error_log);

                        eprintln!("[APP] Error details written to: {:?}", error_log);
                    }
                }
            }

            panic!("Error while building Tauri application: {:?}", e);
        }
    };

    // Proceed to Phase 4: Event Loop
}
```

**Key Points:**
- ✅ Separate handling from run errors
- ✅ Detailed error logging to file
- ✅ Troubleshooting guidance
- ✅ Panic with context (developer-friendly)

---

### Phase 4: Event Loop (main.rs)

**Timing:** After successful build

**Responsibilities:**
1. Run application event loop
2. Handle runtime events
3. Manage application lifecycle

**Code Template:**

```rust
fn main() {
    // ... Phase 1, 2, and 3 code above ...

    // PHASE 4: EVENT LOOP
    println!("[APP] Running application event loop...");
    app.run(|_app_handle, event| {
        // Handle application events
        if let tauri::RunEvent::ExitRequested { api, .. } = event {
            api.prevent_exit();  // Prevent accidental closure
        }
    });
}
```

**Key Points:**
- ✅ Only runs if build succeeded
- ✅ Simple event handling
- ✅ Can prevent accidental exits

---

### Phase 5: Setup Handler (init.rs)

**Timing:** Called by Tauri during `.setup()` phase

**Responsibilities:**
1. Get app data directory
2. Initialize crash handler
3. Run dependency diagnostics
4. Initialize database
5. Show window on success

**Code Template:**

```rust
// src-tauri/src/init.rs
use tauri::{App, Wry};
use std::path::PathBuf;

pub fn init(app: &mut App<Wry>) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Starting Tauri application initialization...");
    let handle = app.handle();

    let db = tauri::async_runtime::block_on(async {
        log::info!("Starting async initialization block...");

        // Get app data directory
        let app_data_dir = handle
            .path()
            .app_data_dir()
            .map_err(|e| {
                log::error!("Failed to get app data directory: {:?}", e);
                format!("Cannot determine app data directory: {:?}", e)
            })?;

        log::info!("App data directory: {:?}", app_data_dir);

        // Ensure directory exists
        if !app_data_dir.exists() {
            log::info!("App data directory does not exist, creating...");
            std::fs::create_dir_all(&app_data_dir)
                .map_err(|e| {
                    log::error!("Failed to create app data directory: {:?}", e);
                    format!("Cannot create app data directory: {:?}", e)
                })?;
            log::info!("Created app data directory");
        }

        // Initialize crash handler
        log::info!("Initializing crash handler...");
        crash_handler::init_crash_handler(&app_data_dir);
        log::info!("Crash handler initialized");

        // Run dependency diagnostics
        log::info!("Running dependency diagnostics...");
        crash_handler::run_dependency_diagnostics(&app_data_dir);
        log::info!("Dependency diagnostics complete");

        // Initialize database with error handling
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

    // Show window ONLY after successful initialization
    if let Some(window) = app.get_webview_window("main") {
        log::info!("Showing main window...");
        if let Err(e) = window.show() {
            log::error!("Failed to show main window: {:?}", e);
        } else {
            log::info!("Main window is now visible");
        }

        // Set focus
        if let Err(e) = window.set_focus() {
            log::error!("Failed to focus window: {:?}", e);
        }
    } else {
        log::warn!("Main window not found during initialization");
    }

    Ok(())
}
```

**Key Points:**
- ✅ Comprehensive logging at each step
- ✅ Proper error propagation via `Result<>`
- ✅ Shows window only after success
- ✅ Uses `async_runtime::block_on` for database init

---

### Phase 6: Database Initialization (services/db.rs)

**Timing:** Called from setup handler

**Responsibilities:**
1. Create database directory
2. Connect to database
3. Run migrations
4. Return connection or error

**Code Template:**

```rust
// src-tauri/src/services/db.rs
use sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};
use std::path::Path;

const DB_NAME: &str = "bear-llm-ai.db";

pub struct Db(pub DatabaseConnection);

impl Db {
    pub async fn new(app_data_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let db_path = app_data_dir.join(DB_NAME);

        log::info!("Initializing database at: {:?}", db_path);

        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            if !parent.exists() {
                log::info!("Creating database directory: {:?}", parent);
                std::fs::create_dir_all(parent)
                    .map_err(|e| {
                        log::error!("Failed to create database directory {:?}: {:?}", parent, e);
                        format!("Failed to create database directory: {:?}", e)
                    })?;
                log::info!("Database directory created successfully");
            }
        }

        let db_url = format!(
            "sqlite:{}?mode=rwc",
            db_path.to_str().ok_or("Invalid database path")?
        );
        log::info!("Connecting to database at: {}", db_url);

        let conn = Database::connect(&db_url)
            .await
            .map_err(|e| {
                log::error!("Failed to connect to database: {:?}", e);
                format!("Database connection failed: {:?}", e)
            })?;

        log::info!("Database connected successfully, running migrations...");
        Migrator::up(&conn, None)
            .await
            .map_err(|e| {
                log::error!("Failed to run database migrations: {:?}", e);
                format!("Database migration failed: {:?}", e)
            })?;

        log::info!("Database initialized successfully");
        Ok(Self(conn))
    }

    // ... other methods ...
}
```

**Key Points:**
- ✅ Returns `Result<Self, Box<dyn std::error::Error>>`
- ✅ Uses `.map_err()` for error context
- ✅ Comprehensive logging
- ✅ No `.expect()` or `.unwrap()`

---

## Configuration Reference

### tauri.conf.json

**Required Configuration:**

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "build": {
    "beforeBuildCommand": "vite build",
    "beforeDevCommand": "vite dev",
    "frontendDist": "../dist",
    "devUrl": "http://localhost:3000"
  },
  "bundle": {
    "active": true,
    "windows": {
      "webviewInstallMode": {
        "type": "downloadBootstrapper",
        "silent": true
      }
    },
    "resources": [
      "WebView2Loader.dll",
      "resources/windows/vc_redist.x64.exe"
    ]
  },
  "app": {
    "windows": [{
      "title": "Your App Name",
      "width": 1440,
      "height": 810,
      "visible": false,  // ✅ CRITICAL: Start hidden
      "center": true
    }],
    "withGlobalTauri": true  // ✅ Helpful for debugging
  }
}
```

**Critical Settings:**
- `"visible": false` - Prevents blank window during initialization
- `"webviewInstallMode.type": "downloadBootstrapper"` - Auto-installs WebView2
- `"webviewInstallMode.silent": true` - Silent installation
- `"resources"` - Bundle WebView2Loader.dll and VC++ installer

---

### Cargo.toml

**Required Dependencies:**

```toml
[dependencies]
tauri = { version = "2.0", features = [] }
tauri-plugin-log = { version = "2.0", features = ["colored"] }
tauri-plugin-clipboard-manager = "2.0"
tauri-plugin-dialog = "2.0"
tauri-plugin-fs = "2.0"
sea-orm = { version = "0.12", features = ["sqlx-sqlite", "runtime-tokio-native-tls", "macros"] }
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio", "tls-native-tls"] }
chrono = { version = "0.4.34", features = ["clock"] }
log = "0.4.20"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[build-dependencies]
tauri-build = { version = "2.0", features = [] }
```

---

## Error Handling Patterns

### Pattern 1: Early Checks (Non-Fatal)

**Use Case:** Dependency checks before initialization

```rust
match check_webview2_runtime() {
    Ok(msg) => log::info!("✓ {}", msg),
    Err(msg) => {
        log::warn!("✗ WARNING: {}", msg);
        // Continue anyway - might still work
    }
}
```

**Characteristics:**
- ✅ Logs warnings
- ✅ Continues execution
- ✅ Provides helpful error messages

---

### Pattern 2: Critical Errors (Fatal)

**Use Case:** Database connection, migrations, essential setup

```rust
let conn = Database::connect(&db_url)
    .await
    .map_err(|e| {
        log::error!("Failed to connect to database: {:?}", e);
        format!("Database connection failed: {:?}", e)
    })?;  // Propagate error up
```

**Characteristics:**
- ✅ Uses `Result<>` return type
- ✅ Logs error before propagating
- ✅ Provides context via `.map_err()`
- ✅ Stops execution (`?` operator)

---

### Pattern 3: Build Errors

**Use Case:** Tauri build failures

```rust
let app = match result {
    Ok(app) => app,
    Err(e) => {
        eprintln!("FATAL ERROR during build: {:?}", e);

        // Log to file with troubleshooting steps
        write_fatal_error_log(e);

        panic!("Error while building Tauri application: {:?}", e);
    }
};
```

**Characteristics:**
- ✅ Separate handling from run errors
- ✅ Logs to file before panic
- ✅ Provides troubleshooting guidance
- ✅ Panics with context

---

### Pattern 4: Optional Operations

**Use Case:** Window management, focus setting

```rust
if let Some(window) = app.get_webview_window("main") {
    if let Err(e) = window.show() {
        log::error!("Failed to show window: {:?}", e);
        // Continue anyway - not fatal
    }
}
```

**Characteristics:**
- ✅ Uses `if let` for optional values
- ✅ Logs errors but continues
- ✅ Non-fatal failures

---

## Testing Strategy

### Test 1: Clean Installation

**Scenario:** Fresh Windows installation, no dependencies

**Steps:**
1. Install application
2. Verify WebView2 auto-installs
3. Verify VC++ auto-installs (if bundled)
4. Verify application starts

**Expected Logs:**
```
[preinit.log]
[2025-10-24 12:00:00] === PRE-INITIALIZATION CHECK ===
[2025-10-24 12:00:00] ✗ WARNING: WebView2 Runtime NOT found
[2025-10-24 12:00:01] ✓ Visual C++ Runtime installed: x64, x86
[2025-10-24 12:00:01] ✓ WebView2 user data folder configured
```

**Pass Criteria:**
- ✅ Application starts
- ✅ Window shown after initialization
- ✅ No errors in logs

---

### Test 2: Corrupted WebView2 Cache

**Scenario:** Existing WebView2 folder with permission issues

**Steps:**
1. Create `%LOCALAPPDATA%\BEAR LLM AI\WebView2` folder
2. Set folder to read-only
3. Start application
4. Verify folder is recreated

**Expected Behavior:**
- ✅ Folder removed and recreated
- ✅ Application starts successfully
- ✅ Logs show recreation

---

### Test 3: Missing Dependencies

**Scenario:** WebView2 and/or VC++ Runtime missing

**Steps:**
1. Uninstall WebView2 Runtime
2. Uninstall VC++ Redistributable
3. Start application
4. Verify error messages

**Expected Logs:**
```
[preinit.log]
[2025-10-24 12:00:00] ✗ WARNING: WebView2 Runtime NOT found
[2025-10-24 12:00:00] ✗ ERROR: Visual C++ Runtime NOT found
```

**Pass Criteria:**
- ✅ Clear error messages
- ✅ Installation instructions logged
- ✅ Application exits gracefully (or installs dependencies)

---

### Test 4: Database Migration

**Scenario:** Existing database with old schema

**Steps:**
1. Copy old database file
2. Start application
3. Verify migrations run
4. Verify data preserved

**Expected Logs:**
```
[Application Log]
[INFO] Database connected successfully, running migrations...
[INFO] Database initialized successfully
```

**Pass Criteria:**
- ✅ Migrations run successfully
- ✅ Data integrity preserved
- ✅ No errors in logs

---

### Test 5: Permission Errors

**Scenario:** No write access to %LOCALAPPDATA%

**Steps:**
1. Remove write permissions from %LOCALAPPDATA%
2. Start application
3. Verify error handling

**Expected Behavior:**
- ✅ Clear error message
- ✅ Fatal error logged
- ✅ Application exits gracefully

---

## Common Pitfalls

### Pitfall 1: Setting Environment Variables Too Late

**Problem:**
```rust
// ❌ WRONG
let app = tauri::Builder::default()
    .setup(|app| {
        std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &folder);  // TOO LATE
        Ok(())
    })
    .build(context);
```

**Solution:**
```rust
// ✅ CORRECT
std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &folder);  // BEFORE Builder

let app = tauri::Builder::default()
    .setup(|app| { /* ... */ })
    .build(context);
```

---

### Pitfall 2: Using .expect() Instead of Result<>

**Problem:**
```rust
// ❌ WRONG
pub async fn new(path: &Path) -> Self {
    let conn = Database::connect(&db_url).await.expect("failed");  // PANIC
    Self(conn)
}
```

**Solution:**
```rust
// ✅ CORRECT
pub async fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
    let conn = Database::connect(&db_url)
        .await
        .map_err(|e| format!("Database connection failed: {:?}", e))?;
    Ok(Self(conn))
}
```

---

### Pitfall 3: Showing Window Immediately

**Problem:**
```json
// ❌ WRONG: tauri.conf.json
{
  "app": {
    "windows": [{
      "visible": true  // Shows immediately, even if init fails
    }]
  }
}
```

**Solution:**
```json
// ✅ CORRECT
{
  "app": {
    "windows": [{
      "visible": false  // Start hidden
    }]
  }
}
```

```rust
// Show explicitly after init
if let Some(window) = app.get_webview_window("main") {
    window.show()?;
}
```

---

### Pitfall 4: Mixing Build and Run Errors

**Problem:**
```rust
// ❌ WRONG
let result = tauri::Builder::default()
    .setup(|app| { /* ... */ })
    .run(context);  // Build and run errors mixed

if let Err(e) = result {
    eprintln!("ERROR: {:?}", e);  // Cannot distinguish type of error
}
```

**Solution:**
```rust
// ✅ CORRECT
let result = tauri::Builder::default()
    .setup(|app| { /* ... */ })
    .build(context);  // Separate build step

let app = match result {
    Ok(app) => app,
    Err(e) => {
        eprintln!("BUILD ERROR: {:?}", e);  // Clear error type
        panic!("Error while building Tauri application: {:?}", e);
    }
};

app.run(|_app_handle, event| { /* ... */ });  // Separate run step
```

---

### Pitfall 5: No Pre-Flight Checks

**Problem:**
```rust
// ❌ WRONG
fn main() {
    // Directly build Tauri without checking dependencies
    let app = tauri::Builder::default().build(context);
}
```

**Solution:**
```rust
// ✅ CORRECT
fn main() {
    // Check dependencies first
    #[cfg(target_os = "windows")]
    {
        match check_webview2_runtime() {
            Ok(msg) => log::info!("✓ {}", msg),
            Err(msg) => log::warn!("✗ WARNING: {}", msg),
        }
    }

    // Now build Tauri
    let app = tauri::Builder::default().build(context);
}
```

---

## Checklist for New Implementations

### Pre-Implementation
- [ ] Read this guide completely
- [ ] Understand the 5 core principles
- [ ] Review working code examples
- [ ] Set up test environment

### Implementation
- [ ] Configure `tauri.conf.json` with `"visible": false`
- [ ] Add pre-initialization checks in `main.rs`
- [ ] Set WebView2 env var BEFORE Tauri Builder
- [ ] Use closure for `.setup()` handler
- [ ] Separate `.build()` and `.run()`
- [ ] Change database `new()` to return `Result<>`
- [ ] Show window only after successful init
- [ ] Add comprehensive logging

### Testing
- [ ] Test clean installation (no dependencies)
- [ ] Test with corrupted WebView2 cache
- [ ] Test with missing dependencies
- [ ] Test database migrations
- [ ] Test permission errors
- [ ] Verify all error messages are helpful
- [ ] Check logs for completeness

### Deployment
- [ ] Bundle WebView2Loader.dll
- [ ] Bundle VC++ Redistributable (optional)
- [ ] Configure auto-installation
- [ ] Test installer on clean machine
- [ ] Document troubleshooting steps
- [ ] Provide user-friendly error messages

---

## Summary

### Critical Success Factors

1. **Timing is everything** - Set up WebView2 BEFORE Tauri initialization
2. **Propagate errors** - Use `Result<>`, not panics
3. **Hide until ready** - Keep window hidden during initialization
4. **Separate concerns** - Handle build errors separately from runtime errors
5. **Check early** - Pre-flight checks before initialization

### Implementation Phases

1. **Pre-init** (main.rs) - Dependency checks, env vars
2. **Build** (main.rs) - Create Tauri application
3. **Error Handling** (main.rs) - Handle build failures
4. **Run** (main.rs) - Event loop
5. **Setup** (init.rs) - Database, crash handler, show window
6. **Database** (db.rs) - Connection, migrations

### Code Quality Metrics

- ✅ No `.expect()` or `.unwrap()` in critical paths
- ✅ All errors return `Result<>` with context
- ✅ Comprehensive logging at each phase
- ✅ Window shown only after success
- ✅ Pre-flight checks before initialization
- ✅ Separate build and run error handling

---

**Implementation Guide Complete**
**Analyst Agent - Hive Mind Collective**
**Use This As Reference For All Future Tauri/WebView2 Projects**
