# Troubleshooting Quick Start - WebView2 Issues

## If BEAR LLM AI Crashes or Won't Start

The application now automatically creates crash logs to help diagnose issues.

### 🎯 All Logs Are In ONE Place

**Windows:**
1. Press `Win + R`
2. Type: `%LOCALAPPDATA%\BEAR LLM AI`
3. Press Enter

**Full Path Example:** `C:\Users\YourUsername\AppData\Local\BEAR LLM AI\`

---

### Step 1: Check Pre-Initialization Log

**File:** `preinit.log`

This file is created **before** the application window appears.

Shows:
- ✓ or ✗ WebView2 Runtime status
- ✓ or ✗ Visual C++ Runtime status

---

### Step 2: Check Fatal Error Log

**File:** `fatal_error.log`

If the app crashes immediately during startup.

Shows:
- The exact error that prevented startup
- Tauri initialization errors

---

### Step 3: Check Crash Log

**File:** `crash.log`

If the app crashes during use.

Shows:
- Where the crash happened (file and line number)
- Error message
- Full stack trace

---

### Step 4: Check Diagnostics Log

**File:** `diagnostics.log`

Detailed dependency information.

Shows:
- WebView2 runtime version
- VC++ runtime version
- WebView2Loader.dll location
- Windows version

## Quick Fix for Common Issues

### Missing WebView2 Runtime

**Error:** `✗ WebView2 Runtime NOT found`

**Fix:**
1. Download WebView2 Runtime: https://developer.microsoft.com/en-us/microsoft-edge/webview2/
2. Install it
3. Reinstall BEAR LLM AI

### Missing Visual C++ Runtime

**Error:** `✗ Visual C++ Runtime NOT found`

**Fix:**
1. Download VC++ Redistributable: https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist
2. Install "Visual C++ Redistributable for Visual Studio 2015-2022" (x64)
3. Restart BEAR LLM AI

## Sending Crash Reports

If you need help, collect these files and create an issue:

```powershell
# Run this in PowerShell to collect all logs:
$timestamp = Get-Date -Format 'yyyyMMdd_HHmmss'
$logsDir = "$env:USERPROFILE\Desktop\bear_llm_logs_$timestamp"
$sourceDir = "$env:LOCALAPPDATA\BEAR LLM AI"

Copy-Item -Path $sourceDir -Destination $logsDir -Recurse -Force
explorer $logsDir

Write-Host "Logs collected in: $logsDir" -ForegroundColor Green
```

Then attach the folder to your issue report.

## Full Documentation

- **Single Location Guide:** `docs/LOG_LOCATIONS.md`
- **Comprehensive Guide:** `docs/CRASH_LOGGING_GUIDE.md` (note: paths need updating)
- **Technical Details:** `docs/WEBVIEW2_CRASH_LOGGING_IMPLEMENTATION.md`
