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

**Fix:** You need to install **BOTH** x64 and x86 versions:

1. **Download x64 version:** https://aka.ms/vs/17/release/vc_redist.x64.exe
   - Run the installer
   - Click "Install"
   - Wait for completion

2. **Download x86 version:** https://aka.ms/vs/17/release/vc_redist.x86.exe
   - Run the installer
   - Click "Install"
   - Wait for completion

3. **Restart your computer** (important!)

4. Try running BEAR LLM AI again

**Why both versions?**
- x64 version: Required for 64-bit components
- x86 version: Required for some 32-bit dependencies
- Both are needed for full compatibility

**Still not working?**
- Check `preinit.log` to verify detection
- Make sure both installers completed without errors
- Check Windows Update for additional updates
- If issue persists, see "Sending Crash Reports" below

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
