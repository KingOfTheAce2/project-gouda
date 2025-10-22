# BEAR LLM AI - Log File Locations

## Single Unified Location

**All logs are now in ONE place:**

```
%LOCALAPPDATA%\BEAR LLM AI\
```

**Full path example:**
```
C:\Users\YourUsername\AppData\Local\BEAR LLM AI\
```

## How to Access

### Method 1: Run Dialog (Fastest)
1. Press `Win + R`
2. Type: `%LOCALAPPDATA%\BEAR LLM AI`
3. Press Enter

### Method 2: File Explorer
1. Open File Explorer
2. Paste in address bar: `%LOCALAPPDATA%\BEAR LLM AI`
3. Press Enter

### Method 3: Command Line
```cmd
cd %LOCALAPPDATA%\"BEAR LLM AI"
dir
```

### Method 4: PowerShell
```powershell
explorer "$env:LOCALAPPDATA\BEAR LLM AI"
```

## Log Files

All logs are in the same directory:

| Log File | Purpose | When Created |
|----------|---------|--------------|
| `preinit.log` | Pre-initialization checks | Before Tauri starts |
| `fatal_error.log` | Tauri startup failures | If Tauri fails to initialize |
| `crash.log` | Runtime crashes | On panic/crash |
| `diagnostics.log` | Dependency checks | During initialization |
| `logs\bear-llm-ai.log` | Application logs | Normal operations |

## Viewing Logs

### View All Logs in One Command (PowerShell)

```powershell
# Open log directory
explorer "$env:LOCALAPPDATA\BEAR LLM AI"

# View recent entries from all logs
Get-ChildItem "$env:LOCALAPPDATA\BEAR LLM AI" -Filter *.log -Recurse |
    ForEach-Object {
        Write-Host "`n=== $($_.Name) ===" -ForegroundColor Cyan
        Get-Content $_.FullName -Tail 20
    }
```

### View Specific Log

```powershell
# Pre-initialization
Get-Content "$env:LOCALAPPDATA\BEAR LLM AI\preinit.log" -Tail 20

# Fatal errors
Get-Content "$env:LOCALAPPDATA\BEAR LLM AI\fatal_error.log" -Tail 20

# Crashes
Get-Content "$env:LOCALAPPDATA\BEAR LLM AI\crash.log" -Tail 20

# Diagnostics
Get-Content "$env:LOCALAPPDATA\BEAR LLM AI\diagnostics.log" -Tail 20

# Application
Get-Content "$env:LOCALAPPDATA\BEAR LLM AI\logs\bear-llm-ai.log" -Tail 50
```

## Collecting All Logs for Bug Reports

### PowerShell Script

```powershell
# Create archive on Desktop
$destDir = "$env:USERPROFILE\Desktop\bear_llm_logs_$(Get-Date -Format 'yyyyMMdd_HHmmss')"
$sourceDir = "$env:LOCALAPPDATA\BEAR LLM AI"

# Copy all logs
Copy-Item -Path $sourceDir -Destination $destDir -Recurse -Force

# Open folder
explorer $destDir

Write-Host "Logs collected in: $destDir" -ForegroundColor Green
```

## Log Rotation

To clear old logs:

```powershell
# Clear all logs (keeps folder structure)
Remove-Item "$env:LOCALAPPDATA\BEAR LLM AI\*.log" -Force
Remove-Item "$env:LOCALAPPDATA\BEAR LLM AI\logs\*.log" -Force

# Or delete entire log directory
Remove-Item "$env:LOCALAPPDATA\BEAR LLM AI" -Recurse -Force
```

Logs will be recreated on next application start.

## Troubleshooting

### Can't Find Log Directory?

The application may not have started yet. The directory is created on first run.

### Logs Are Empty?

Check timestamps. If the application crashed before writing logs, check Windows Event Viewer:
1. Press `Win + X` → Event Viewer
2. Navigate to: Windows Logs → Application
3. Look for "BEAR LLM AI" errors

### Need More Verbose Logs?

In development mode, set environment variable:
```powershell
$env:RUST_LOG="debug"
# Then run application
```

## Quick Reference Card

```
📁 All Logs Location:
   %LOCALAPPDATA%\BEAR LLM AI\

📄 Log Files:
   ├── preinit.log          (Pre-startup checks)
   ├── fatal_error.log      (Startup failures)
   ├── crash.log            (Runtime crashes)
   ├── diagnostics.log      (Dependency info)
   └── logs\
       └── bear-llm-ai.log  (Normal operations)

⌨️ Quick Access:
   Win + R → %LOCALAPPDATA%\BEAR LLM AI

👁️ Quick View:
   dir %LOCALAPPDATA%\"BEAR LLM AI" /s
```

## Related Documentation

- **Comprehensive Guide:** `docs/CRASH_LOGGING_GUIDE.md`
- **Quick Troubleshooting:** `docs/TROUBLESHOOTING_QUICK_START.md`
- **Technical Details:** `docs/WEBVIEW2_CRASH_LOGGING_IMPLEMENTATION.md`
