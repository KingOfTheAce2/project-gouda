# Window Crash Fix Analysis: BEAR-LLM vs project-gouda

## Executive Summary

The BEAR-LLM repository contains several critical fixes that resolved window crashes and blank screen issues. These fixes address three main categories:

1. **Content Security Policy (CSP) Issues** - JavaScript/CSS failing to load
2. **Console Window Flashing** - Unwanted console windows appearing during startup
3. **Initialization Timeout Handling** - App hanging during setup check
4. **WebView2 Configuration** - Proper user data folder setup

This document details each fix and provides specific recommendations for project-gouda.

---

## 1. Critical CSP Fix (Version 1.0.16)

### Problem
The app showed a **blank white screen** after installation. Root cause: Content Security Policy was blocking Tauri's asset protocols, preventing JavaScript and CSS from loading.

### Root Cause
- CSP in `tauri.conf.json` only allowed `'self'` protocol
- Tauri converts Vite bundle paths to `tauri://localhost/assets/*` or `http://tauri.localhost/assets/*`
- Original CSP blocked these protocols, preventing asset loading

### BEAR-LLM Solution (Commit: 8606afc)
Modified `tauri.conf.json` CSP to explicitly allow Tauri protocols:

```json
{
  "app": {
    "security": {
      "csp": "default-src 'self' tauri: http://tauri.localhost https://tauri.localhost; 
              img-src 'self' asset: https://asset.localhost http://asset.localhost tauri: http://tauri.localhost blob: data:; 
              style-src 'self' 'unsafe-inline' tauri: http://tauri.localhost; 
              script-src 'self' tauri: http://tauri.localhost https://tauri.localhost; 
              font-src 'self' asset: https://asset.localhost http://asset.localhost tauri: http://tauri.localhost; 
              connect-src 'self' tauri: http://tauri.localhost; 
              object-src 'none'; 
              base-uri 'self'; 
              form-action 'self'"
    }
  }
}
```

### project-gouda Current Status
**CRITICAL ISSUE FOUND**: tauri.conf.json has `"csp": null`

```json
"app": {
  "security": {
    "csp": null
  }
}
```

Setting CSP to `null` disables the Content Security Policy entirely, which:
- Creates a security vulnerability
- May not properly load Tauri assets on some Windows installations
- Leaves the app exposed to XSS attacks

### Recommended Fix for project-gouda
Replace the CSP configuration with BEAR-LLM's working version:

```json
"app": {
  "security": {
    "csp": "default-src 'self' tauri: http://tauri.localhost https://tauri.localhost; img-src 'self' asset: https://asset.localhost http://asset.localhost tauri: http://tauri.localhost blob: data:; style-src 'self' 'unsafe-inline' tauri: http://tauri.localhost; script-src 'self' tauri: http://tauri.localhost https://tauri.localhost; font-src 'self' asset: https://asset.localhost http://asset.localhost tauri: http://tauri.localhost data:; connect-src 'self' tauri: http://tauri.localhost; object-src 'none'; base-uri 'self'; form-action 'self'"
  }
}
```

Key CSP improvements:
- ✓ Allows `tauri:` protocol for local asset loading
- ✓ Allows `http://tauri.localhost` for WebView communication
- ✓ Allows `https://tauri.localhost` for secure connections
- ✓ Allows `blob:` and `data:` for images (used by Vite)
- ✓ Allows `'unsafe-inline'` for styles (necessary for Vite dev)
- ✓ Restricts dangerous features (`object-src 'none'`)

---

## 2. Console Window Flashing Fix (Version 1.0.8)

### Problem
**Black console window flashing** during startup and operation on Windows. This happens when:
- Build process exits with console output
- System commands (wmic, powercfg) spawn without suppression
- Debug println! statements create windows

### BEAR-LLM Solution (Commit: 530d534)

#### A. Fix `build.rs`
**Before**: Used `println!()` which creates a console window
```rust
fn main() {
    std::process::exit(1);  // WRONG: exits with console output
}
```

**After**: Use logging without console
```rust
fn main() {
    #[cfg(target_os = "windows")]
    {
        use std::path::Path;
        let dll_path = Path::new("WebView2Loader.dll");
        if !dll_path.exists() {
            eprintln!("cargo:warning=WebView2Loader.dll not found");
        } else {
            println!("cargo:rerun-if-changed=WebView2Loader.dll");
        }
    }
    tauri_build::build()
}
```

#### B. Add `CREATE_NO_WINDOW` to System Commands
**Example from BEAR-LLM's system_monitor.rs**:

```rust
#[cfg(target_os = "windows")]
fn execute_wmic_command(command: &str) -> Result<String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    
    let output = std::process::Command::new("wmic")
        .args(&["/node:.", "/format:wmixml", command])
        .creation_flags(CREATE_NO_WINDOW)  // Suppress console window
        .output()?;
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
```

#### C. Replace `println!` with Logging
**Before**:
```rust
println!("PDF parsing failed, using fallback: {}", e);
```

**After**:
```rust
tracing::warn!("PDF parsing failed, using fallback: {}", e);
// or for non-output scenarios:
log::info!("Processing file: {}", filename);
```

### project-gouda Current Status
✓ **PARTIALLY FIXED**: Has `process_helper.rs` with `CREATE_NO_WINDOW` trait
✓ **PARTIALLY FIXED**: `build.rs` uses `eprintln!` correctly
✗ **NOT FIXED**: Still using `eprintln!` in some places (which is acceptable)
✗ **POTENTIAL ISSUE**: Still using `println!` in `file_processor.rs` for debug output

### Recommended Fixes for project-gouda

**1. Verify process_helper.rs is used everywhere:**
Ensure all system command spawning uses the `.no_window()` trait:

```rust
use crate::process_helper::ProcessCommandExt;

let output = tokio::process::Command::new("python")
    .arg("script.py")
    .no_window()  // Add this!
    .output()
    .await?;
```

**2. Replace debug println! with logging:**
Find all `println!` calls in Rust code:
```bash
grep -r "println!" src-tauri/src/ --include="*.rs"
```

Replace with:
```rust
log::info!("message");      // for info level
tracing::debug!("message");  // for debug level
```

**3. Keep eprintln! for cargo:warning (build.rs only)**

---

## 3. Initialization Timeout Handling (Version 1.0.9)

### Problem
After MSI installer launch, the app shows a **blank white screen** and hangs indefinitely. The app is waiting for setup status checks without a timeout, and if the backend hangs, the UI never appears.

### BEAR-LLM Solution (Commit: 7b61ab6)

#### A. Add Timeout to Setup Check (App.tsx)
```typescript
useEffect(() => {
  const checkFirstRun = async () => {
    try {
      // Add 5-second timeout to prevent hanging
      const timeout = new Promise((_, reject) =>
        setTimeout(() => reject(new Error('Setup check timeout')), 5000)
      );

      const checkPromise = Promise.all([
        invoke<boolean>('check_first_run'),
        invoke<any>('get_setup_status')
      ]);

      // Use Promise.race to enforce timeout
      const [isFirstRun, setupStatus] = await Promise.race([
        checkPromise, 
        timeout
      ]) as [boolean, any];

      if (isFirstRun || !setupStatus.setup_complete) {
        setShowSetup(true);
      } else {
        setSetupComplete(true);
      }
    } catch (err) {
      logger.error('Error checking setup status', err);
      // Graceful fallback: assume setup is complete
      setSetupComplete(true);
    } finally {
      // Always hide loading screen
      setIsInitializing(false);
    }
  };

  checkFirstRun();
}, []);
```

#### B. Add Loading Screen During Initialization
```typescript
const [isInitializing, setIsInitializing] = useState(true);

// In render:
if (isInitializing) {
  return (
    <div className="flex items-center justify-center h-screen">
      <div className="flex flex-col items-center gap-4">
        <BearLogo />
        <p>Initializing...</p>
        <div className="animate-spin rounded-full h-8 w-8 border-t-2 border-blue-500" />
      </div>
    </div>
  );
}
```

### project-gouda Current Status
✗ **NOT IMPLEMENTED**: No timeout on initialization checks
✗ **NO LOADING UI**: Returns `null` during InitializationProvider loading (blank screen)

Current code:
```typescript
// Returns null while loading - shows blank screen
if (initialized) return children;
if (isModelsError) throw error;
if (isSettingsError) throw error;
return null;  // BLANK SCREEN!
```

### Recommended Fix for project-gouda

**Update `src/lib/providers.tsx`**:

```typescript
export function InitializationProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const [initialized, setInitialized] = useState(false);
  const [initError, setInitError] = useState<string | null>(null);
  const { i18n } = useTranslation();
  const { setTheme } = useTheme();
  const { setModels, setSettings } = useAppStateStore();
  
  const {
    data: modelList,
    isSuccess: isModelsSuccess,
    isError: isModelsError,
    error: modelsError,
  } = useListModelsQuery();
  
  const {
    data: settingList,
    isSuccess: isSettingsSuccess,
    isError: isSettingsError,
    error: settingsError,
  } = useListSettingsQuery();

  useEffect(() => {
    if (isModelsSuccess) {
      setModels(modelList);
    }
  }, [isModelsSuccess, modelList, setModels]);

  useEffect(() => {
    if (isSettingsSuccess) {
      setSettings(settingList);
      const language = settingList.find(
        (s) => s.key === SETTING_DISPLAY_LANGUAGE
      )?.value;
      if (language !== i18n.language) {
        i18n.changeLanguage(language);
      }
      const themeSetting = settingList.find(
        (s) => s.key === SETTING_DISPLAY_THEME
      )?.value as string;
      setTheme(themeSetting);
    }
  }, [i18n, isSettingsSuccess, setModels, setSettings, setTheme, settingList]);

  // Add timeout protection
  useEffect(() => {
    const timer = setTimeout(() => {
      if (!initialized) {
        console.warn('Initialization timeout - proceeding with defaults');
        setInitError('Initialization took too long, using defaults');
        setInitialized(true);
      }
    }, 5000); // 5 second timeout

    return () => clearTimeout(timer);
  }, [initialized]);

  useEffect(() => {
    if (isModelsSuccess && isSettingsSuccess) {
      setInitialized(true);
    }
  }, [isModelsSuccess, isSettingsSuccess]);

  // Show error if initialization failed
  if (isModelsError) {
    throw new AppError(
      ERROR_TYPE_APP_STATE,
      modelsError.message,
      'Failed to initiate models!'
    );
  }
  if (isSettingsError) {
    throw new AppError(
      ERROR_TYPE_APP_STATE,
      settingsError.message,
      'Failed to initiate settings!'
    );
  }

  if (initialized) {
    return children;
  }

  // Show loading screen instead of blank screen
  return (
    <div className="flex items-center justify-center h-screen w-screen bg-background">
      <div className="flex flex-col items-center gap-4">
        <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-primary" />
        <p className="text-sm text-muted-foreground">Initializing...</p>
      </div>
    </div>
  );
}
```

---

## 4. WebView2 Configuration (Initialization)

### Problem
WebView2 runtime requires proper user data folder setup on Windows, otherwise the window may fail to initialize or crash.

### BEAR-LLM Solution (from `src-tauri/src/init.rs`)

```rust
#[cfg(target_os = "windows")]
fn setup_webview2_user_data_folder(app_data_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    use std::env;

    let webview2_dir = app_data_dir.join("WebView2");
    
    log::info!("Setting up WebView2 user data folder at: {:?}", webview2_dir);

    // Verify WebView2Loader.dll exists
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let webview2_loader = exe_dir.join("WebView2Loader.dll");
            if webview2_loader.exists() {
                log::info!("WebView2Loader.dll found");
            } else {
                log::warn!("WebView2Loader.dll not found");
            }
        }
    }

    // Create directory if needed
    if !webview2_dir.exists() {
        std::fs::create_dir_all(&webview2_dir)?;
        log::info!("Created WebView2 user data folder");
    } else {
        // Verify write permissions with test file
        let test_file = webview2_dir.join(".write_test");
        match std::fs::write(&test_file, b"test") {
            Ok(_) => {
                let _ = std::fs::remove_file(&test_file);
                log::info!("WebView2 folder permissions verified");
            }
            Err(e) => {
                log::warn!("WebView2 folder exists but is not writable");
                // Attempt recovery
                std::fs::remove_dir_all(&webview2_dir)?;
                std::fs::create_dir_all(&webview2_dir)?;
                log::info!("Successfully recreated WebView2 folder");
            }
        }
    }

    // Set environment variable
    std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &webview2_dir);
    log::info!("Set WEBVIEW2_USER_DATA_FOLDER environment variable");

    Ok(())
}
```

### project-gouda Current Status
✓ **PARTIALLY IMPLEMENTED**: Has WebView2 setup in `init.rs`
✗ **MISSING**: Some error handling details and logging

### Recommended Enhancement
Ensure the init.rs has proper non-fatal error handling:

```rust
if let Err(e) = setup_webview2_user_data_folder(&app_data_dir) {
    log::warn!("WebView2 setup failed, continuing with defaults: {:?}", e);
    // Continue - WebView2 will use system defaults
}
```

---

## 5. Error Handling in main.tsx (React Entry Point)

### Problem
If React fails to initialize, users see a blank screen with no error information.

### BEAR-LLM Solution (main.tsx)

```typescript
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import ErrorBoundary from './components/ErrorBoundary';
import './index.css';

// Global error handlers
window.addEventListener('error', (event) => {
  console.error('🔴 Initialization error:', event.error);
});

window.addEventListener('unhandledrejection', (event) => {
  console.error('🔴 Unhandled promise rejection:', event.reason);
});

console.log('🚀 BEAR AI initializing...');

const rootElement = document.getElementById('root');
if (!rootElement) {
  console.error('🔴 Fatal: Root element not found!');
  document.body.innerHTML = '<div style="padding: 20px; font-family: system-ui;">Fatal Error: Root element not found. Please reinstall the application.</div>';
} else {
  try {
    ReactDOM.createRoot(rootElement).render(
      <React.StrictMode>
        <ErrorBoundary>
          <App />
        </ErrorBoundary>
      </React.StrictMode>,
    );
    console.log('✅ BEAR AI React app mounted successfully');
  } catch (error) {
    console.error('🔴 Fatal error rendering app:', error);
    rootElement.innerHTML = '<div style="padding: 20px; font-family: system-ui;">Fatal Error: Failed to render application. Please check console for details.</div>';
  }
}
```

### project-gouda Current Status
✗ **NOT IMPLEMENTED**: Current main.tsx lacks global error handling

### Recommended Fix for project-gouda
Update `src/main.tsx`:

```typescript
import './styles.css';
import '@/i18n';

import { AnimatePresence } from 'framer-motion';
import { ThemeProvider } from 'next-themes';
import React, { Suspense } from 'react';
import ReactDOM from 'react-dom/client';
import { ErrorBoundary } from 'react-error-boundary';
import { RouterProvider } from 'react-router-dom';

import { GlobalFallback } from './components/GlobalFallback';
import { router } from './router'; // Your router config

// Global error handlers for debugging
window.addEventListener('error', (event) => {
  console.error('🔴 Window error:', event.error);
  console.error('Stack:', event.error?.stack);
});

window.addEventListener('unhandledrejection', (event) => {
  console.error('🔴 Unhandled promise rejection:', event.reason);
});

console.log('🚀 Application initializing...');

const rootElement = document.getElementById('root');

if (!rootElement) {
  console.error('🔴 Fatal: Root element not found!');
  document.body.innerHTML = `
    <div style="
      padding: 20px;
      font-family: system-ui;
      text-align: center;
      color: #d32f2f;
    ">
      <h1>Fatal Error</h1>
      <p>Root element not found. Please reinstall the application.</p>
    </div>
  `;
} else {
  try {
    ReactDOM.createRoot(rootElement).render(
      <React.StrictMode>
        <ErrorBoundary FallbackComponent={GlobalFallback}>
          <ThemeProvider defaultTheme="system" attribute="class">
            <Suspense fallback={<div>Loading...</div>}>
              <RouterProvider router={router} />
            </Suspense>
          </ThemeProvider>
        </ErrorBoundary>
      </React.StrictMode>,
    );
    console.log('✅ React app mounted successfully');
  } catch (error) {
    console.error('🔴 Fatal error rendering app:', error);
    if (rootElement) {
      rootElement.innerHTML = `
        <div style="
          padding: 20px;
          font-family: system-ui;
          text-align: center;
          color: #d32f2f;
        ">
          <h1>Fatal Error</h1>
          <p>Failed to render application.</p>
          <p style="font-size: 12px; color: #666;">
            Check developer console (F12) for details
          </p>
          <pre style="text-align: left; background: #f5f5f5; padding: 10px; overflow: auto;">
${String(error)}
          </pre>
        </div>
      `;
    }
  }
}
```

---

## Summary of Required Fixes for project-gouda

| Issue | Priority | Status | Action |
|-------|----------|--------|--------|
| CSP Configuration | CRITICAL | ✗ Missing | Replace `csp: null` with proper CSP configuration |
| Initialization Timeout | HIGH | ✗ Missing | Add timeout to setup checks in InitializationProvider |
| Loading UI | HIGH | ✗ Missing | Show loading screen instead of blank screen during init |
| Error Handling in main.tsx | HIGH | ✗ Missing | Add global error handlers and fallback UI |
| Console Window Suppression | MEDIUM | ✓ Partial | Verify `.no_window()` is used everywhere |
| Debug println! Removal | MEDIUM | ✗ Needed | Replace println! with logging framework |
| WebView2 Setup | LOW | ✓ Partial | Enhance error handling |

---

## Implementation Order

1. **First**: Fix CSP configuration (prevents blank screen)
2. **Second**: Add initialization timeout and loading UI (prevents hangs)
3. **Third**: Add error handling in main.tsx (improves debugging)
4. **Fourth**: Audit and fix console output (improves user experience)
5. **Fifth**: Review WebView2 setup (ensures reliability)

---

## Testing Checklist

After applying fixes:

- [ ] Build MSI installer
- [ ] Test fresh installation on clean Windows machine
- [ ] Verify window appears with UI (not blank)
- [ ] Check no console windows flash during startup
- [ ] Test with backend temporarily offline (should show timeout gracefully)
- [ ] Check browser dev console (F12) for any errors
- [ ] Verify logs in AppData folder are captured
- [ ] Test on Windows 10 and Windows 11
- [ ] Test on machines with different WebView2 versions

---

## References

- BEAR-LLM Repository: https://github.com/KingOfTheAce2/BEAR-LLM
- Critical Commits:
  - CSP Fix: 8606afc
  - Console Window Fix: 530d534
  - Initialization Timeout: 7b61ab6
  - White Screen Fix: 7b61ab6
- Tauri Security: https://tauri.app/v1/guides/distribution/windows/
- WebView2 Documentation: https://learn.microsoft.com/en-us/microsoft-edge/webview2/

