# Crash Logging and Diagnostics Guide

**Version:** 0.0.13
**Last Updated:** October 22, 2025

## Overview

BEAR LLM AI now includes comprehensive crash logging and dependency diagnostics to help troubleshoot WebView2 and runtime dependency issues on Windows.

## Log File Locations

### Windows

All log files are automatically created when the application starts or crashes. You can find them in the following locations:

#### 1. Pre-Initialization Log (Before Tauri starts)

**Location:** `%TEMP%\bear_llm_ai_preinit.log`

**Full Path Example:** `C:\Users\YourUsername\AppData\Local\Temp\bear_llm_ai_preinit.log`

**Contents:**
- WebView2 runtime detection results
- Visual C++ runtime detection results
- Timestamp of each startup attempt

**When to Check:** If the application fails to start or shows a blank window immediately.

---

#### 2. Fatal Error Log (Tauri initialization failures)

**Location:** `%TEMP%\bear_llm_ai_fatal_error.log`

**Full Path Example:** `C:\Users\YourUsername\AppData\Local\Temp\bear_llm_ai_fatal_error.log`

**Contents:**
- Fatal errors that prevent Tauri from running
- Error details and stack traces
- Timestamp of crash

**When to Check:** If the application crashes immediately on startup before any UI appears.

---

#### 3. Crash Log (Panic/Runtime crashes)

**Location:** `%APPDATA%\com.bearllm.ai\crash.log`

**Full Path Example:** `C:\Users\YourUsername\AppData\Roaming\com.bearllm.ai\crash.log`

**Contents:**
- Panic location (file, line, column)
- Panic message
- Full backtrace
- Timestamp of crash

**When to Check:** If the application crashes during use or after initialization.

---

#### 4. Diagnostics Log (Dependency checks)

**Location:** `%APPDATA%\com.bearllm.ai\diagnostics.log`

**Full Path Example:** `C:\Users\YourUsername\AppData\Roaming\com.bearllm.ai\diagnostics.log`

**Contents:**
- WebView2 runtime status
- Visual C++ runtime status
- WebView2Loader.dll detection
- Operating system information
- Timestamp of each check

**When to Check:** To verify all dependencies are properly installed.

---

#### 5. Application Log (Normal operations)

**Location:** `%APPDATA%\com.bearllm.ai\logs\bear-llm-ai.log`

**Full Path Example:** `C:\Users\YourUsername\AppData\Roaming\com.bearllm.ai\logs\bear-llm-ai.log`

**Contents:**
- Application startup information
- Database initialization
- WebView2 setup details
- General application operations

**When to Check:** For general troubleshooting of application behavior.

---

## How to Access Logs

### Method 1: Using Windows Explorer

1. Press `Win + R` to open Run dialog
2. Type one of the following and press Enter:
   - `%TEMP%` - For pre-initialization and fatal error logs
   - `%APPDATA%\com.bearllm.ai` - For crash, diagnostics, and application logs
3. Look for the relevant log file

### Method 2: Using Command Prompt

```cmd
# View pre-initialization log
type %TEMP%\bear_llm_ai_preinit.log

# View fatal error log
type %TEMP%\bear_llm_ai_fatal_error.log

# View crash log
type %APPDATA%\com.bearllm.ai\crash.log

# View diagnostics log
type %APPDATA%\com.bearllm.ai\diagnostics.log

# View application log
type %APPDATA%\com.bearllm.ai\logs\bear-llm-ai.log
```

### Method 3: Using PowerShell

```powershell
# Open log directory in Explorer
explorer "$env:APPDATA\com.bearllm.ai"
explorer "$env:TEMP"

# View logs in console
Get-Content "$env:TEMP\bear_llm_ai_preinit.log" -Tail 50
Get-Content "$env:APPDATA\com.bearllm.ai\crash.log" -Tail 50
Get-Content "$env:APPDATA\com.bearllm.ai\diagnostics.log" -Tail 50
```

---

## Understanding Log Entries

### Pre-Initialization Log Format

```
[2025-10-22 14:30:15] === PRE-INITIALIZATION CHECK ===
[2025-10-22 14:30:15] ✓ WebView2 Runtime found (Registry): version 120.0.2210.144
[2025-10-22 14:30:15] ✓ Visual C++ Runtime installed: version v14.38.33135.0
[2025-10-22 14:30:15] Pre-initialization check complete. Log: "C:\\Users\\...\\bear_llm_ai_preinit.log"
[2025-10-22 14:30:15] Proceeding to Tauri initialization...
```

### Diagnostics Log Format

```
[2025-10-22 14:30:16] === DEPENDENCY DIAGNOSTICS ===
[2025-10-22 14:30:16] ✓ WebView2 Runtime found (Registry): version 120.0.2210.144
[2025-10-22 14:30:16] ✓ Visual C++ Runtime installed: version v14.38.33135.0
[2025-10-22 14:30:16] ✓ WebView2Loader.dll found at: "C:\\Program Files\\BEAR LLM AI\\WebView2Loader.dll"
[2025-10-22 14:30:16] OS: Windows 11
[2025-10-22 14:30:16] === END DIAGNOSTICS ===
```

### Crash Log Format

```
================================================================================
CRASH REPORT - 2025-10-22 14:35:22
================================================================================

Location: src/services/db.rs:45:12
Panic message: Database connection failed
Backtrace:
   0: std::backtrace::Backtrace::create
   1: bear_llm_ai_lib::services::db::Db::new
   ...

================================================================================
```

---

## Common Issues and Solutions

### Issue 1: WebView2 Runtime Not Found

**Log Entry:**
```
[2025-10-22 14:30:15] ✗ WARNING: WebView2 Runtime NOT found - installation may fail
```

**Solution:**
1. The NSIS installer should automatically download and install WebView2
2. If manual installation is needed, download from:
   https://developer.microsoft.com/en-us/microsoft-edge/webview2/
3. Reinstall BEAR LLM AI after installing WebView2

---

### Issue 2: Visual C++ Runtime Not Found

**Log Entry:**
```
[2025-10-22 14:30:15] ✗ WARNING: Visual C++ Runtime NOT found - may cause runtime errors
```

**Solution:**
1. The NSIS installer should automatically install VC++ Runtime
2. If manual installation is needed, download from:
   https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist
3. Download and install "Visual C++ Redistributable for Visual Studio 2015-2022" (x64)

---

### Issue 3: WebView2Loader.dll Not Found

**Log Entry:**
```
[2025-10-22 14:30:16] ✗ WebView2Loader.dll NOT found at: "C:\\Program Files\\BEAR LLM AI\\WebView2Loader.dll"
```

**Solution:**
1. This indicates a packaging issue during build
2. Rebuild the application with `npm run tauri build`
3. Verify `WebView2Loader.dll` is in the `resources` section of `tauri.conf.json`
4. Reinstall from a fresh installer

---

### Issue 4: Application Crashes on Startup

**Steps to Diagnose:**

1. Check pre-initialization log first:
   ```cmd
   type %TEMP%\bear_llm_ai_preinit.log
   ```

2. Check fatal error log:
   ```cmd
   type %TEMP%\bear_llm_ai_fatal_error.log
   ```

3. Check crash log:
   ```cmd
   type %APPDATA%\com.bearllm.ai\crash.log
   ```

4. Check diagnostics log:
   ```cmd
   type %APPDATA%\com.bearllm.ai\diagnostics.log
   ```

5. Collect all logs and report the issue with:
   - Windows version (Settings → System → About)
   - All log files listed above
   - Steps to reproduce

---

## Collecting Logs for Bug Reports

When reporting issues, please include the following:

### Step 1: Reproduce the Issue

1. Delete existing logs (optional, for clean slate):
   ```powershell
   Remove-Item "$env:TEMP\bear_llm_ai_*.log"
   Remove-Item "$env:APPDATA\com.bearllm.ai\*.log"
   Remove-Item "$env:APPDATA\com.bearllm.ai\logs\*.log"
   ```

2. Launch BEAR LLM AI
3. Reproduce the crash or issue

### Step 2: Collect All Logs

```powershell
# Create a logs archive folder
$logsDir = "$env:USERPROFILE\Desktop\bear_llm_ai_logs"
New-Item -ItemType Directory -Path $logsDir -Force

# Copy all logs
Copy-Item "$env:TEMP\bear_llm_ai_*.log" $logsDir -ErrorAction SilentlyContinue
Copy-Item "$env:APPDATA\com.bearllm.ai\*.log" $logsDir -ErrorAction SilentlyContinue
Copy-Item "$env:APPDATA\com.bearllm.ai\logs\*.log" $logsDir -ErrorAction SilentlyContinue

# Open the folder
explorer $logsDir
```

### Step 3: Include System Information

```powershell
# Get Windows version
Get-ComputerInfo | Select-Object WindowsProductName, WindowsVersion, OsHardwareAbstractionLayer

# Get installed runtimes
Get-ItemProperty HKLM:\SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64 -ErrorAction SilentlyContinue
Get-ItemProperty HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5} -ErrorAction SilentlyContinue
```

---

## Developer Information

### Crash Handler Implementation

**File:** `src-tauri/src/crash_handler.rs`

**Functions:**
- `init_crash_handler()` - Sets up panic hook
- `write_diagnostic_info()` - Writes timestamped diagnostic entries
- `check_webview2_runtime()` - Detects WebView2 installation
- `check_vcredist_runtime()` - Detects VC++ runtime
- `run_dependency_diagnostics()` - Runs all checks

### Integration Points

1. **main.rs** - Pre-initialization checks before Tauri starts
2. **init.rs** - Crash handler initialization and dependency diagnostics
3. **lib.rs** - Module export

### Enabling Backtraces

For more detailed crash information during development:

```powershell
$env:RUST_BACKTRACE=1
cargo run
```

Or in production builds, backtraces are automatically captured via `std::backtrace::Backtrace::force_capture()`.

---

## Support

If you're still experiencing issues after checking the logs:

1. Collect all logs using the script above
2. Include your Windows version and system information
3. Describe the exact steps to reproduce the issue
4. Report via GitHub issues: https://github.com/KingOfTheAce2/project-gouda/issues

---

## Version History

- **v0.0.13** - Added comprehensive crash logging and dependency diagnostics
- **v0.0.12** - Initial WebView2 fixes
- **v0.0.11** - NSIS installer hooks

