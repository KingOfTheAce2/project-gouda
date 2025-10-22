# BEAR LLM AI Documentation

## 📁 Documentation Structure

### 🔧 Troubleshooting (Current Issues & Solutions)

**Location:** `docs/troubleshooting/`

Active documentation for diagnosing and fixing WebView2 and dependency issues:

| File | Description |
|------|-------------|
| **`CRASH_LOGGING_SUMMARY.md`** | ⭐ Quick overview of crash logging system |
| **`LOG_LOCATIONS.md`** | ⭐ Where to find all log files (single location guide) |
| **`TROUBLESHOOTING_QUICK_START.md`** | Quick troubleshooting steps for crashes |
| `CRASH_LOGGING_GUIDE.md` | Comprehensive crash logging documentation |
| `WEBVIEW2_CRASH_LOGGING_IMPLEMENTATION.md` | Technical implementation details |
| `WEBVIEW2_STATUS_REPORT.md` | Status of all WebView2 fixes |
| `WINDOW_CRASH_FIXES.md` | Window crash analysis and solutions |

**Start Here for Crashes:**
1. `CRASH_LOGGING_SUMMARY.md` - What was added
2. `LOG_LOCATIONS.md` - Where to find logs
3. `TROUBLESHOOTING_QUICK_START.md` - Quick fixes

---

### 📦 Archive (Historical Documentation)

**Location:** `docs/archive/`

Previous installation guides, fix summaries, and version history:

- Installation guides (pre-v0.0.13)
- WebView2 bundling documentation
- Version change logs (v0.0.11, etc.)
- Historical fix summaries
- Multi-user installation guides

**Note:** These docs are kept for reference but may be outdated. Use `troubleshooting/` for current information.

---

### 🌍 Localization

**Location:** `docs/`

User-facing options documentation in multiple languages:

- `options_en.md` - English
- `options_de.md` - German (Deutsch)
- `options_fr.md` - French (Français)
- `options_nl.md` - Dutch (Nederlands)
- `options_zh-Hans.md` - Chinese Simplified (简体中文)

---

### 📊 Comparisons

**Location:** `docs/`

- `BEAR_LLM_COMPARISON.md` - Comparison with other LLM tools

---

## 🚀 Quick Access Guide

### If the Application Crashes

```
1. Open: docs/troubleshooting/LOG_LOCATIONS.md
2. Access: %LOCALAPPDATA%\BEAR LLM AI
3. Check: preinit.log, crash.log, diagnostics.log
```

### For Implementation Details

```
docs/troubleshooting/WEBVIEW2_CRASH_LOGGING_IMPLEMENTATION.md
```

### For Historical Context

```
docs/archive/ - Previous fixes and installation guides
```

---

## 📝 File Organization Summary

```
docs/
├── README.md                          (This file)
├── BEAR_LLM_COMPARISON.md            (Tool comparison)
│
├── troubleshooting/                   (Active troubleshooting)
│   ├── CRASH_LOGGING_SUMMARY.md      ⭐ Start here for crashes
│   ├── LOG_LOCATIONS.md              ⭐ Log file locations
│   ├── TROUBLESHOOTING_QUICK_START.md
│   ├── CRASH_LOGGING_GUIDE.md
│   ├── WEBVIEW2_CRASH_LOGGING_IMPLEMENTATION.md
│   ├── WEBVIEW2_STATUS_REPORT.md
│   └── WINDOW_CRASH_FIXES.md
│
├── archive/                           (Historical documentation)
│   ├── COMMIT-SUMMARY.md
│   ├── FIXES-SUMMARY.md
│   ├── Installation-Fixes.md
│   ├── OUT_OF_BOX_WINDOWS_INSTALLATION.md
│   ├── User-Level-Installation.md
│   ├── V0.0.11_CHANGES.md
│   ├── WEBVIEW2_BUNDLING.md
│   ├── WINDOWS_EXECUTION_TROUBLESHOOTING.md
│   ├── WINDOWS_INSTALLATION_FIXES.md
│   ├── WINDOWS_INSTALL_GUIDE.md
│   ├── WINDOWS_MULTI_USER_GUIDE.md
│   ├── WebView2-Fixes-Applied.md
│   └── WebView2-Implementation-Audit.md
│
└── options_*.md                       (Localized user docs)
    ├── options_en.md
    ├── options_de.md
    ├── options_fr.md
    ├── options_nl.md
    └── options_zh-Hans.md
```

---

## 🎯 Common Tasks

### Diagnose a Crash
1. Read: `troubleshooting/LOG_LOCATIONS.md`
2. Access logs: `Win + R` → `%LOCALAPPDATA%\BEAR LLM AI`
3. Follow: `troubleshooting/TROUBLESHOOTING_QUICK_START.md`

### Understand WebView2 Fixes
1. Overview: `troubleshooting/WEBVIEW2_STATUS_REPORT.md`
2. Details: `troubleshooting/WINDOW_CRASH_FIXES.md`
3. Implementation: `troubleshooting/WEBVIEW2_CRASH_LOGGING_IMPLEMENTATION.md`

### Collect Logs for Bug Report
```powershell
$logsDir = "$env:USERPROFILE\Desktop\bear_llm_logs_$(Get-Date -Format 'yyyyMMdd_HHmmss')"
Copy-Item -Path "$env:LOCALAPPDATA\BEAR LLM AI" -Destination $logsDir -Recurse -Force
explorer $logsDir
```

---

## 📅 Version History

- **v0.0.13** - Added comprehensive crash logging system
- **v0.0.12** - WebView2 and VC++ runtime fixes
- **v0.0.11** - Out-of-box Windows installation
- Earlier versions - See `archive/` folder

---

## 🔗 External Resources

- **WebView2 Runtime:** https://developer.microsoft.com/microsoft-edge/webview2/
- **VC++ Redistributable:** https://learn.microsoft.com/cpp/windows/latest-supported-vc-redist
- **Tauri Documentation:** https://tauri.app/

---

**Last Updated:** October 22, 2025
