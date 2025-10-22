# Troubleshooting Documentation

## 🎯 Start Here

If BEAR LLM AI crashes or won't start:

1. **`LOG_LOCATIONS.md`** - Find where all crash logs are stored
2. **`CRASH_LOGGING_SUMMARY.md`** - Quick overview of what was added
3. **`TROUBLESHOOTING_QUICK_START.md`** - Quick fixes for common issues

## 📚 Complete Documentation

### For Users

| File | Description |
|------|-------------|
| **`LOG_LOCATIONS.md`** | ⭐ All logs in one place - %LOCALAPPDATA%\BEAR LLM AI |
| **`TROUBLESHOOTING_QUICK_START.md`** | Quick fixes for WebView2 and VC++ issues |
| `CRASH_LOGGING_GUIDE.md` | Comprehensive guide (400+ lines) |

### For Developers

| File | Description |
|------|-------------|
| `CRASH_LOGGING_SUMMARY.md` | Implementation summary and file changes |
| `WEBVIEW2_CRASH_LOGGING_IMPLEMENTATION.md` | Technical implementation details |
| `WEBVIEW2_STATUS_REPORT.md` | Status of all WebView2 fixes |
| `WINDOW_CRASH_FIXES.md` | Analysis from BEAR-LLM repository |

## 🚀 Quick Actions

### Access All Logs
```
Press Win + R
Type: %LOCALAPPDATA%\BEAR LLM AI
Press Enter
```

### Collect Logs for Bug Report
```powershell
$logsDir = "$env:USERPROFILE\Desktop\bear_llm_logs_$(Get-Date -Format 'yyyyMMdd_HHmmss')"
Copy-Item -Path "$env:LOCALAPPDATA\BEAR LLM AI" -Destination $logsDir -Recurse -Force
explorer $logsDir
```

### View Recent Logs
```powershell
Get-Content "$env:LOCALAPPDATA\BEAR LLM AI\preinit.log" -Tail 20
Get-Content "$env:LOCALAPPDATA\BEAR LLM AI\diagnostics.log" -Tail 20
Get-Content "$env:LOCALAPPDATA\BEAR LLM AI\crash.log" -Tail 20
```

## 📂 Log Files

All logs are in: `%LOCALAPPDATA%\BEAR LLM AI\`

| Log File | Purpose |
|----------|---------|
| `preinit.log` | Pre-startup dependency checks |
| `fatal_error.log` | Tauri initialization failures |
| `crash.log` | Runtime crashes with backtrace |
| `diagnostics.log` | Dependency info (WebView2, VC++) |
| `logs\bear-llm-ai.log` | Normal application logs |

## 🔗 Common Issues

### WebView2 Runtime Missing
Download: https://developer.microsoft.com/microsoft-edge/webview2/

### Visual C++ Runtime Missing
Download: https://learn.microsoft.com/cpp/windows/latest-supported-vc-redist

Install "Visual C++ Redistributable for Visual Studio 2015-2022" (x64)

## ↩️ Navigation

- [Main Documentation Index](../README.md)
- [Archive (Historical Docs)](../archive/)

---

**Last Updated:** October 22, 2025
