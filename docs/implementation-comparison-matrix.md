# WebView2 Implementation Comparison Matrix

**Analyst Agent - Hive Mind Collective**
**Generated:** 2025-10-24

---

## Quick Reference: Working vs. Broken Implementation

### 🔴 Broken Implementation Signature

```rust
// ❌ BROKEN PATTERN - DO NOT USE
fn main() {
    let result = tauri::Builder::default()
        .setup(bear_llm_ai_lib::init::init)  // WebView2 setup happens here (TOO LATE)
        .run(context);  // Error handling happens after run() fails
}

// src-tauri/src/init.rs
pub fn init(app: &mut App<Wry>) -> Result<(), Box<dyn std::error::Error>> {
    // Database setup
    let db = Db::new(&app_data_dir).await;  // ❌ No error handling

    // WebView2 setup happens HERE - after Tauri has already initialized WebView2
    setup_webview2_user_data_folder(&app_data_dir)?;  // ❌ TOO LATE
}
```

### 🟢 Working Implementation Signature

```rust
// ✅ CORRECT PATTERN - USE THIS
fn main() {
    // PHASE 1: Pre-initialization checks and setup (BEFORE Tauri)
    #[cfg(target_os = "windows")]
    {
        // Check dependencies
        bear_llm_ai_lib::crash_handler::check_webview2_runtime();
        bear_llm_ai_lib::crash_handler::check_vcredist_runtime();

        // Setup WebView2 BEFORE Tauri initialization
        let webview2_dir = log_dir.join("WebView2");
        std::fs::create_dir_all(&webview2_dir)?;
        std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &webview2_dir);
    }

    // PHASE 2: Build Tauri application
    let result = tauri::Builder::default()
        .setup(|app| {
            // Proper error handling
            match bear_llm_ai_lib::init::init(app) {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("Setup failed: {:?}", e);
                    Err(e)
                }
            }
        })
        .build(context);

    // PHASE 3: Handle build errors separately
    let app = match result {
        Ok(app) => app,
        Err(e) => {
            // Log and panic with details
            panic!("Error while building Tauri application: {:?}", e);
        }
    };

    // PHASE 4: Run event loop
    app.run(|_app_handle, event| {
        // Event handling
    });
}

// src-tauri/src/init.rs
pub fn init(app: &mut App<Wry>) -> Result<(), Box<dyn std::error::Error>> {
    // Database setup with proper error propagation
    let db = Db::new(&app_data_dir)
        .await
        .map_err(|e| {  // ✅ Proper error handling
            log::error!("Failed to initialize database: {:?}", e);
            format!("Database initialization failed: {:?}", e)
        })?;

    // Show window ONLY after successful initialization
    if let Some(window) = app.get_webview_window("main") {
        window.show()?;
        window.set_focus()?;
    }

    Ok(())
}
```

---

## Detailed Comparison Table

### 1. Initialization Sequence

| Component | Broken Implementation | Working Implementation | Critical? |
|-----------|----------------------|------------------------|-----------|
| **Pre-init checks** | None | WebView2 + VC++ detection | ⚠️ Medium |
| **WebView2 env var** | Set in `init()` after Tauri starts | Set in `main()` before Tauri | ✅ **CRITICAL** |
| **Folder creation** | In `init()` during setup | In `main()` before Tauri | ✅ **CRITICAL** |
| **Dependency order** | Tauri → Setup → WebView2 folder | WebView2 folder → Tauri → Setup | ✅ **CRITICAL** |

**Code Snippets:**

```rust
// ❌ BROKEN: WebView2 setup happens AFTER Tauri initialization
fn main() {
    let result = tauri::Builder::default()
        .setup(bear_llm_ai_lib::init::init)  // Setup runs after Builder starts
        .run(context);
}

// In init.rs:
pub fn init(app: &mut App<Wry>) -> Result<...> {
    setup_webview2_user_data_folder(&app_data_dir)?;  // TOO LATE!
}

// ✅ WORKING: WebView2 setup happens BEFORE Tauri initialization
fn main() {
    // Setup WebView2 environment FIRST
    std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &webview2_dir);

    // THEN build Tauri
    let result = tauri::Builder::default()
        .setup(|app| { /* ... */ })
        .build(context);
}
```

---

### 2. Error Handling

| Aspect | Broken Implementation | Working Implementation | Impact |
|--------|----------------------|------------------------|--------|
| **Database errors** | `.expect("failed")` → panic | `Result<>` with `.map_err()` | ✅ **HIGH** |
| **Build errors** | Not separated from run errors | Handled in `match result` | ✅ **HIGH** |
| **Setup errors** | Function reference (no error logging) | Closure with logging | ⚠️ Medium |
| **Fatal error logging** | Minimal | Comprehensive with troubleshooting steps | ⚠️ Medium |
| **Error propagation** | Panics immediately | Propagates up call stack | ✅ **HIGH** |

**Code Snippets:**

```rust
// ❌ BROKEN: Database initialization panics on error
impl Db {
    pub async fn new(app_data_dir: &Path) -> Self {
        let conn = Database::connect(&db_url)
            .await
            .expect("failed to connect to database");  // PANIC - cannot recover!

        Migrator::up(&conn, None)
            .await
            .expect("failed to run migrations");  // PANIC - cannot recover!

        Self(conn)
    }
}

// ✅ WORKING: Database initialization returns Result
impl Db {
    pub async fn new(app_data_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Database::connect(&db_url)
            .await
            .map_err(|e| {
                log::error!("Failed to connect to database: {:?}", e);
                format!("Database connection failed: {:?}", e)
            })?;  // Propagate error up

        Migrator::up(&conn, None)
            .await
            .map_err(|e| {
                log::error!("Failed to run database migrations: {:?}", e);
                format!("Database migration failed: {:?}", e)
            })?;  // Propagate error up

        Ok(Self(conn))
    }
}
```

---

### 3. Window Management

| Aspect | Broken Implementation | Working Implementation | Impact |
|--------|----------------------|------------------------|--------|
| **Initial visibility** | `"visible": true` (default) | `"visible": false` | ✅ **HIGH** |
| **Show timing** | Automatic (immediate) | After successful init | ✅ **HIGH** |
| **Focus handling** | Automatic | Explicit `set_focus()` | ⚠️ Low |
| **Error state** | Blank white window | No window shown | ✅ **HIGH** |

**Configuration & Code:**

```json
// ❌ BROKEN: tauri.conf.json
{
  "app": {
    "windows": [{
      // "visible" defaults to true - window shows immediately
    }]
  }
}

// ✅ WORKING: tauri.conf.json
{
  "app": {
    "windows": [{
      "visible": false  // Start hidden
    }]
  }
}
```

```rust
// ❌ BROKEN: No explicit window management
pub fn init(app: &mut App<Wry>) -> Result<...> {
    // Database setup...
    // Window automatically shows (even if init failed)
    Ok(())
}

// ✅ WORKING: Show window only after success
pub fn init(app: &mut App<Wry>) -> Result<...> {
    // Database setup...

    // Show window ONLY if we got here successfully
    if let Some(window) = app.get_webview_window("main") {
        log::info!("Showing main window...");
        window.show()?;
        window.set_focus()?;
    }

    Ok(())
}
```

---

### 4. Logging & Diagnostics

| Aspect | Broken Implementation | Working Implementation | Impact |
|--------|----------------------|------------------------|--------|
| **Pre-init logging** | None | File-based (`preinit.log`) | ⚠️ Medium |
| **Progress indicators** | Minimal | Comprehensive (`println!` at each phase) | ⚠️ Medium |
| **Error context** | Generic error messages | Detailed error messages with cause | ⚠️ Medium |
| **Fatal error log** | Basic error message | Troubleshooting steps + context | ⚠️ Medium |
| **Dependency diagnostics** | None | WebView2 + VC++ checks | ⚠️ Medium |

**Code Snippets:**

```rust
// ❌ BROKEN: Minimal logging
fn main() {
    let result = tauri::Builder::default()
        .setup(bear_llm_ai_lib::init::init)
        .run(context);

    if let Err(e) = result {
        eprintln!("Error: {:?}", e);  // Generic error
        panic!("Error while running Tauri application: {:?}", e);
    }
}

// ✅ WORKING: Comprehensive logging
fn main() {
    println!("[BEAR LLM AI] Starting Tauri initialization...");

    #[cfg(target_os = "windows")]
    {
        // Pre-init checks with logging to file
        match check_webview2_runtime() {
            Ok(msg) => writeln!(file, "[{}] ✓ {}", timestamp, msg),
            Err(msg) => writeln!(file, "[{}] ✗ WARNING: {}", timestamp, msg),
        }
    }

    println!("[BEAR LLM AI] Building Tauri application...");

    let result = tauri::Builder::default()
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
        .build(context);

    let app = match result {
        Ok(app) => {
            println!("[BEAR LLM AI] Application built successfully...");
            app
        }
        Err(e) => {
            eprintln!("[BEAR LLM AI] FATAL ERROR during application build: {:?}", e);

            // Detailed error logging to file
            writeln!(file, "[{}] Common causes:", timestamp);
            writeln!(file, "[{}] 1. WebView2 Runtime initialization failed", timestamp);
            writeln!(file, "[{}] 2. Window creation failed", timestamp);
            // ... more diagnostic info

            panic!("Error while building Tauri application: {:?}", e);
        }
    };

    println!("[BEAR LLM AI] Running application event loop...");
}
```

---

### 5. Setup Handler Implementation

| Aspect | Broken Implementation | Working Implementation | Impact |
|--------|----------------------|------------------------|--------|
| **Handler type** | Function reference | Closure with error handling | ⚠️ Medium |
| **Error logging** | None (direct pass-through) | Explicit logging in closure | ⚠️ Medium |
| **Success logging** | None | Explicit success message | ⚠️ Low |
| **Error context** | Lost in propagation | Preserved and logged | ⚠️ Medium |

**Code Snippets:**

```rust
// ❌ BROKEN: Function reference (no logging)
let result = tauri::Builder::default()
    .setup(bear_llm_ai_lib::init::init)  // Direct function reference
    .run(context);

// ✅ WORKING: Closure with error handling
let result = tauri::Builder::default()
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
    .build(context);
```

---

### 6. Build vs. Run Error Handling

| Aspect | Broken Implementation | Working Implementation | Impact |
|--------|----------------------|------------------------|--------|
| **Error separation** | Mixed (both in `.run()`) | Separated (`.build()` + `.run()`) | ✅ **HIGH** |
| **Build error handling** | N/A | Dedicated match arm | ✅ **HIGH** |
| **Run error handling** | Generic | Detailed with troubleshooting | ⚠️ Medium |
| **Error logging location** | `fatal_error.log` | `fatal_error.log` (build) + runtime logs | ⚠️ Medium |

**Code Snippets:**

```rust
// ❌ BROKEN: Mixed error handling
let result = tauri::Builder::default()
    .setup(bear_llm_ai_lib::init::init)
    .run(context);  // Build errors and run errors mixed

if let Err(e) = result {
    // Cannot distinguish build error from run error
    write_error_log(e);
    panic!("Error while running Tauri application: {:?}", e);
}

// ✅ WORKING: Separated error handling
let result = tauri::Builder::default()
    .setup(|app| { /* ... */ })
    .build(context);  // Separate build step

// Handle build errors
let app = match result {
    Ok(app) => app,
    Err(e) => {
        eprintln!("[BEAR LLM AI] FATAL ERROR during application build: {:?}", e);

        // Build-specific error logging
        writeln!(file, "[{}] Common causes:", timestamp);
        writeln!(file, "[{}] 1. WebView2 Runtime initialization failed", timestamp);
        writeln!(file, "[{}] 2. Window creation failed", timestamp);
        writeln!(file, "[{}] 3. Plugin initialization failed", timestamp);
        writeln!(file, "[{}] 4. Database initialization failed", timestamp);

        panic!("Error while building Tauri application: {:?}", e);
    }
};

// Handle run errors separately
println!("[BEAR LLM AI] Running application event loop...");
app.run(|_app_handle, event| {
    // Event handling
});
```

---

## File-by-File Comparison

### src-tauri/src/main.rs

| Line Range | Broken | Working | Change Type | Priority |
|------------|--------|---------|-------------|----------|
| 1-7 | Same | Same | N/A | N/A |
| 8-68 | ❌ None | ✅ Pre-init checks | **Addition** | ✅ **CRITICAL** |
| 69-154 | ❌ None | ✅ WebView2 setup | **Addition** | ✅ **CRITICAL** |
| 155-211 | ❌ Direct `.run()` | ✅ `.build()` + `.run()` separation | **Refactor** | ✅ **HIGH** |
| 212-256 | ❌ Minimal error handling | ✅ Comprehensive error logging | **Enhancement** | ⚠️ Medium |

**Key Differences:**

```rust
// BROKEN: Lines 8-75 (entire main function)
fn main() {
    let context = tauri::generate_context!();
    let log = tauri_plugin_log::Builder::new()
        .level(log::LevelFilter::Info)
        .with_colors(ColoredLevelConfig::default());

    let result = tauri::Builder::default()
        .plugin(log.build())
        .setup(bear_llm_ai_lib::init::init)
        .run(context);

    if let Err(e) = result {
        panic!("Error while running Tauri application: {:?}", e);
    }
}

// WORKING: Lines 8-266 (entire main function with phases)
fn main() {
    // PHASE 1: Pre-initialization (lines 8-68)
    #[cfg(target_os = "windows")]
    {
        // Dependency checks
        // ...
    }

    // PHASE 2: WebView2 setup (lines 69-154)
    #[cfg(target_os = "windows")]
    {
        let webview2_dir = log_dir.join("WebView2");
        std::fs::create_dir_all(&webview2_dir)?;
        std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &webview2_dir);
    }

    // PHASE 3: Tauri build (lines 155-211)
    let result = tauri::Builder::default()
        .plugin(log.build())
        .setup(|app| {
            match bear_llm_ai_lib::init::init(app) {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("Setup failed: {:?}", e);
                    Err(e)
                }
            }
        })
        .build(context);

    // PHASE 4: Error handling & run (lines 212-266)
    let app = match result {
        Ok(app) => app,
        Err(e) => {
            // Detailed error logging
            panic!("Error while building Tauri application: {:?}", e);
        }
    };

    app.run(|_app_handle, event| { /* ... */ });
}
```

---

### src-tauri/src/init.rs

| Function | Broken | Working | Change Type | Priority |
|----------|--------|---------|-------------|----------|
| `setup_webview2_user_data_folder()` | ✅ Exists (85 lines) | ❌ **DELETED** | **Removal** | ✅ **CRITICAL** |
| `init()` | ❌ Calls `setup_webview2_*` | ✅ Does NOT call (moved to main) | **Refactor** | ✅ **CRITICAL** |
| `init()` | ❌ `Db::new()` returns `Self` | ✅ `Db::new()` returns `Result<>` | **Enhancement** | ✅ **HIGH** |
| `init()` | ❌ No window management | ✅ Shows window after success | **Addition** | ✅ **HIGH** |
| `init()` | ❌ Minimal logging | ✅ Comprehensive logging | **Enhancement** | ⚠️ Medium |

**Key Differences:**

```rust
// BROKEN: init.rs includes WebView2 setup
pub fn init(app: &mut App<Wry>) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    let db = tauri::async_runtime::block_on(async {
        let app_data_dir = /* ... */;

        // ❌ WebView2 setup happens HERE (too late)
        if let Err(e) = setup_webview2_user_data_folder(&app_data_dir) {
            log::warn!("Failed to setup WebView2: {:?}", e);
        }

        let db_wrapper = Db::new(&app_data_dir).await;  // ❌ No error handling
        Ok::<_, String>(db_wrapper.0)
    })?;

    handle.manage(BearLlmAiHandle { db });
    Ok(())
}

// ✅ WORKING: init.rs does NOT include WebView2 setup
pub fn init(app: &mut App<Wry>) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Starting Tauri application initialization...");
    let handle = app.handle();

    let db = tauri::async_runtime::block_on(async {
        log::info!("Starting async initialization block...");

        let app_data_dir = /* ... */;

        // WebView2 setup is now in main.rs (before Tauri init)

        // ✅ Database initialization with proper error handling
        let db_wrapper = Db::new(&app_data_dir)
            .await
            .map_err(|e| {
                log::error!("Database initialization failed: {:?}", e);
                format!("Failed to initialize database: {:?}", e)
            })?;

        Ok::<_, String>(db_wrapper.0)
    })?;

    handle.manage(BearLlmAiHandle { db });

    // ✅ Show window ONLY after successful initialization
    if let Some(window) = app.get_webview_window("main") {
        log::info!("Showing main window...");
        window.show()?;
        window.set_focus()?;
    }

    Ok(())
}

// BROKEN: Separate function (deleted in working version)
#[cfg(target_os = "windows")]
fn setup_webview2_user_data_folder(app_data_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // 85 lines of WebView2 setup logic
    // ❌ THIS ENTIRE FUNCTION IS DELETED IN WORKING VERSION
}
```

---

### src-tauri/src/services/db.rs

| Function | Broken | Working | Change Type | Priority |
|----------|--------|---------|-------------|----------|
| `Db::new()` signature | `async fn new(...) -> Self` | `async fn new(...) -> Result<Self, Box<dyn Error>>` | **Signature change** | ✅ **CRITICAL** |
| Database connection error | `.expect("failed")` | `.map_err(\|e\| { log + format })` | **Error handling** | ✅ **CRITICAL** |
| Migration error | `.expect("failed")` | `.map_err(\|e\| { log + format })` | **Error handling** | ✅ **CRITICAL** |
| Success return | `Self(conn)` | `Ok(Self(conn))` | **Return type** | ✅ **CRITICAL** |

**Key Differences:**

```rust
// ❌ BROKEN: Db::new() panics on error
impl Db {
    pub async fn new(app_data_dir: &Path) -> Self {
        let db_path = app_data_dir.join(DB_NAME);
        let db_url = format!("sqlite:{}?mode=rwc", db_path.to_str().unwrap());

        let conn = Database::connect(&db_url)
            .await
            .expect("failed to connect to database");  // PANIC - no recovery!

        Migrator::up(&conn, None)
            .await
            .expect("failed to run migrations");  // PANIC - no recovery!

        Self(conn)  // Returns Self directly
    }
}

// ✅ WORKING: Db::new() returns Result
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
            }
        }

        let db_url = format!("sqlite:{}?mode=rwc", db_path.to_str().ok_or("Invalid database path")?);
        log::info!("Connecting to database at: {}", db_url);

        let conn = Database::connect(&db_url)
            .await
            .map_err(|e| {
                log::error!("Failed to connect to database: {:?}", e);
                format!("Database connection failed: {:?}", e)
            })?;  // Propagate error up

        log::info!("Database connected successfully, running migrations...");
        Migrator::up(&conn, None)
            .await
            .map_err(|e| {
                log::error!("Failed to run database migrations: {:?}", e);
                format!("Database migration failed: {:?}", e)
            })?;  // Propagate error up

        log::info!("Database initialized successfully");
        Ok(Self(conn))  // Returns Result<Self>
    }
}
```

---

### src-tauri/tauri.conf.json

| Property | Broken | Working | Change Type | Priority |
|----------|--------|---------|-------------|----------|
| `app.windows[0].visible` | `true` (default) | `false` | **Configuration** | ✅ **HIGH** |
| `app.withGlobalTauri` | `false` (default) | `true` | **Configuration** | ⚠️ Low |
| `version` | `0.0.14` | `0.0.16` | **Version** | N/A |

**Configuration Diff:**

```json
// BROKEN: Default window configuration
{
  "app": {
    "windows": [{
      "title": "BEAR LLM AI",
      "width": 1440,
      "height": 810
      // "visible" defaults to true
    }]
  }
}

// ✅ WORKING: Hidden window until initialized
{
  "app": {
    "windows": [{
      "title": "BEAR LLM AI",
      "width": 1440,
      "height": 810,
      "visible": false  // ✅ Start hidden
    }],
    "withGlobalTauri": true  // ✅ Enable debugging
  }
}
```

---

## Migration Path: Broken → Working

### Step-by-Step Refactoring Guide

#### Step 1: Update tauri.conf.json
```json
{
  "app": {
    "windows": [{
      "visible": false  // Add this line
    }]
  }
}
```

#### Step 2: Move WebView2 setup from init.rs to main.rs

**DELETE from init.rs:**
```rust
// Delete this entire function (85 lines)
#[cfg(target_os = "windows")]
fn setup_webview2_user_data_folder(app_data_dir: &PathBuf) -> Result<...> {
    // ...
}

// Delete this call from init()
setup_webview2_user_data_folder(&app_data_dir)?;
```

**ADD to main.rs (before Tauri::Builder):**
```rust
#[cfg(target_os = "windows")]
{
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        let log_dir = std::path::Path::new(&local_app_data).join("BEAR LLM AI");
        let webview2_dir = log_dir.join("WebView2");
        std::fs::create_dir_all(&webview2_dir)?;
        std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &webview2_dir);
    }
}
```

#### Step 3: Update Db::new() to return Result

**CHANGE in db.rs:**
```rust
// From:
pub async fn new(app_data_dir: &Path) -> Self {
    let conn = Database::connect(&db_url).await.expect("failed");
    Self(conn)
}

// To:
pub async fn new(app_data_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
    let conn = Database::connect(&db_url)
        .await
        .map_err(|e| {
            log::error!("Failed to connect to database: {:?}", e);
            format!("Database connection failed: {:?}", e)
        })?;
    Ok(Self(conn))
}
```

#### Step 4: Update init.rs to handle new Db::new() signature

**CHANGE in init.rs:**
```rust
// From:
let db_wrapper = Db::new(&app_data_dir).await;

// To:
let db_wrapper = Db::new(&app_data_dir)
    .await
    .map_err(|e| {
        log::error!("Database initialization failed: {:?}", e);
        format!("Failed to initialize database: {:?}", e)
    })?;
```

#### Step 5: Add window show logic to init.rs

**ADD to init.rs (at end of init function):**
```rust
// Show window ONLY after successful initialization
if let Some(window) = app.get_webview_window("main") {
    log::info!("Showing main window...");
    window.show()?;
    window.set_focus()?;
}
```

#### Step 6: Separate build and run in main.rs

**CHANGE in main.rs:**
```rust
// From:
let result = tauri::Builder::default()
    .setup(bear_llm_ai_lib::init::init)
    .run(context);

if let Err(e) = result {
    panic!("Error: {:?}", e);
}

// To:
let result = tauri::Builder::default()
    .setup(|app| {
        match bear_llm_ai_lib::init::init(app) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Setup failed: {:?}", e);
                Err(e)
            }
        }
    })
    .build(context);

let app = match result {
    Ok(app) => app,
    Err(e) => {
        eprintln!("FATAL ERROR during build: {:?}", e);
        panic!("Error while building Tauri application: {:?}", e);
    }
};

app.run(|_app_handle, event| {
    // Event handling
});
```

---

## Testing Checklist

After applying the migration, verify:

- [ ] WebView2 environment variable is set before Tauri initialization
- [ ] Database errors are propagated as `Result<>`, not panics
- [ ] Window starts hidden (`visible: false`)
- [ ] Window shows ONLY after successful initialization
- [ ] Build errors are logged to `fatal_error.log`
- [ ] Pre-initialization checks log to `preinit.log`
- [ ] Application shows window on success
- [ ] Application exits gracefully on error (no blank window)

---

## Summary

### Critical Differences (MUST IMPLEMENT)

1. **WebView2 setup timing**: Move from `init.rs` to `main.rs` (before Tauri)
2. **Database error handling**: Change from `expect()` to `Result<>` + `map_err()`
3. **Window visibility**: Set `"visible": false` + show after init
4. **Error separation**: Split `.build()` and `.run()` error handling

### Recommended Enhancements

1. **Pre-init checks**: Add dependency diagnostics (WebView2, VC++)
2. **Comprehensive logging**: Add progress indicators at each phase
3. **Setup handler**: Use closure instead of function reference
4. **Fatal error logging**: Add troubleshooting steps to error logs

### Optional Simplifications

1. **WebView2 integrity checking**: Can simplify folder recreation logic
2. **WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS**: May be redundant

---

**Comparison Matrix Complete**
**Analyst Agent - Hive Mind Collective**
