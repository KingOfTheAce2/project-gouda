# Release Notes - BEAR LLM AI v0.0.2

> This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).

**Release Date:** 2025-10-11
**Build Status:** ✅ Stable
**Platform Support:** Windows, macOS, Linux

---

## 🎉 What's New in v0.0.2

This is a critical bug-fix release that addresses installation and startup issues found in v0.0.1.

### 🔧 Critical Fixes

#### 1. **Fixed Application Identifier** 🏷️
- **Issue:** Application identifier was ambiguous (`com.bearllmai`)
- **Problem:** Windows interpreted it as `com.bearllmail`, creating confusing folder names
- **Fixed:** Changed to clear identifier `com.bearllm.ai`
- **Impact:** Clean, professional folder structure

**Before:**
```
C:\Users\<name>\AppData\Local\com.bearllmail\  ❌ Confusing!
```

**After:**
```
C:\Users\<name>\AppData\Local\com.bearllm.ai\  ✅ Clear!
```

#### 2. **Added WebView2 Auto-Installation** 🌐
- **Issue:** Application crashed if Microsoft Edge WebView2 wasn't installed
- **Problem:** Users saw error: "De gegevensmap kan niet worden gemaakt"
- **Fixed:** Added automatic WebView2 installation during setup
- **Impact:** Application works out-of-the-box on fresh Windows installations

#### 3. **Fixed Startup Crashes** 💥
- **Issue:** Application crashed immediately after opening window
- **Problem:** Poor error handling when creating app data directories
- **Fixed:** Added graceful error handling and automatic directory creation
- **Impact:** Application starts reliably on first launch

#### 4. **Improved Error Handling** 🛡️
- **Issue:** Silent failures with no error messages
- **Problem:** Users couldn't diagnose issues
- **Fixed:** Added debug logging and helpful error messages
- **Impact:** Easier troubleshooting and support

---

## 📋 Complete Change Log

### Configuration Changes
- ✅ Updated application identifier: `com.bearllmai` → `com.bearllm.ai`
- ✅ Added WebView2 auto-installation configuration
- ✅ Version bumped to 0.0.2

### Code Improvements
- ✅ Enhanced `init.rs` with proper error handling
- ✅ Added directory existence checks and creation
- ✅ Improved database initialization with logging
- ✅ Added SQLite connection mode flags (`?mode=rwc`)

### Documentation Added
- ✅ `docs/CRASH_FIX.md` - Technical deep-dive on startup crash fix
- ✅ `docs/WEBVIEW2_FIX.md` - WebView2 and identifier fix documentation
- ✅ `docs/WINDOWS_INSTALL_GUIDE.md` - User-friendly installation guide
- ✅ `docs/UPDATER_SETUP.md` - Tauri updater configuration guide
- ✅ `docs/GITHUB_SECRETS_SETUP.md` - CI/CD setup instructions

### Files Changed
```
src-tauri/tauri.conf.json      | Version + identifier + WebView2 config
src-tauri/Cargo.toml           | Version bump
package.json                   | Version bump
src-tauri/src/init.rs          | Better error handling (+19 lines)
src-tauri/src/services/db.rs   | Enhanced initialization (+13 lines)
```

---

## 🚀 Upgrade Instructions

### For New Users
Simply download and install v0.0.2. Everything will work out of the box!

### For Existing v0.0.1 Users

**Option 1: Clean Install (Recommended)**

1. **Uninstall v0.0.1:**
   - Windows Settings → Apps → BEAR LLM AI → Uninstall

2. **Delete old data folders:**
   ```powershell
   # PowerShell
   Remove-Item "$env:LOCALAPPDATA\com.bearllmai" -Recurse -Force
   Remove-Item "$env:LOCALAPPDATA\com.bearllmail" -Recurse -Force
   ```

3. **Install v0.0.2:**
   - Download and run the new installer
   - WebView2 will auto-install if needed

**Option 2: In-Place Upgrade**

1. Install v0.0.2 over v0.0.1
2. The updater will migrate your data automatically
3. Old folders will remain but won't be used

⚠️ **Note:** Your conversations, models, and settings will be preserved in both cases.

---

## 📦 Download

### Windows
- **NSIS Installer (Recommended):** `BEAR_LLM_AI_0.0.2_x64-setup.exe`
- **MSI Installer:** `BEAR_LLM_AI_0.0.2_x64_en-US.msi`
- **Portable:** `BEAR_LLM_AI_0.0.2_x64.zip`

### macOS
- **Apple Silicon (M1/M2/M3):** `BEAR_LLM_AI_0.0.2_aarch64.dmg`
- **Intel:** `BEAR_LLM_AI_0.0.2_x64.dmg`

### Linux
- **AppImage:** `BEAR_LLM_AI_0.0.2_amd64.AppImage`
- **Debian/Ubuntu:** `BEAR_LLM_AI_0.0.2_amd64.deb`

---

## 🐛 Known Issues

### Windows
- **First launch may be slow:** WebView2 installation can take 30-60 seconds
- **Antivirus warnings:** Some antivirus software may flag the installer (false positive)

### All Platforms
- **Ollama required:** Application requires Ollama to be installed separately
- **No provider fallback:** Only Ollama is supported (OpenAI/Claude removed)

---

## 🔜 What's Next (v0.0.3+)

Planned improvements:
- 🌍 Multi-language UI support (Dutch, German, French)
- 📊 Conversation export/import
- 🎨 Custom themes
- ⚙️ Advanced model configuration
- 🔌 Plugin system
- 📱 Mobile companion app

---

## 🆘 Support

### Having Issues?

1. **Check documentation:**
   - `docs/WINDOWS_INSTALL_GUIDE.md` - Installation help
   - `docs/WEBVIEW2_FIX.md` - Technical details
   - `docs/CRASH_FIX.md` - Troubleshooting

2. **Manual WebView2 install:**
   - Download: [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/#download-section)
   - Install and restart the application

3. **Report issues:**
   - Include: Windows version, error messages, screenshots
   - GitHub Issues: [project-gouda/issues](https://github.com/yourorg/project-gouda/issues)

---

## 📊 Comparison: v0.0.1 vs v0.0.2

| Feature | v0.0.1 | v0.0.2 |
|---------|--------|--------|
| **Identifier** | ❌ `com.bearllmai` (ambiguous) | ✅ `com.bearllm.ai` (clear) |
| **WebView2** | ❌ Manual installation required | ✅ Auto-installs |
| **Startup** | ❌ Crashes on first launch | ✅ Reliable startup |
| **Error Handling** | ❌ Silent failures | ✅ Helpful error messages |
| **Windows Path** | ❌ `.../com.bearllmail/` | ✅ `.../com.bearllm.ai/` |
| **Documentation** | ⚠️ Basic README | ✅ Comprehensive guides |

---

## 🙏 Credits

- **Original Author:** Frank Zhang
- **License:** MIT License (original code) + BEAR AI SOFTWARE LICENSE AGREEMENT (proprietary changes)
- **Built with:** Tauri v2, React 18, TypeScript, Rust
- **Special Thanks:** To early testers who reported the v0.0.1 issues

---

## 🔐 Security

This release includes properly signed installers with:
- ✅ Code signing for Windows executables
- ✅ Signed update packages
- ✅ Verified builds via GitHub Actions

**Signing Key Fingerprint:**
```
Public Key: dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IDE3N0U1NTAyMkI5MTY4RUQKUldUdGFKRXJBbFYrRitSaUcvUGFsYkpseGhqSDFUMjFlLzMycnlaY1VKaDJJTkpUNU9iU0srSmQK
```

---

## 📝 License

This software includes:
- **Open Source Components:** Licensed under MIT License
- **Proprietary Enhancements:** Licensed under BEAR AI SOFTWARE LICENSE AGREEMENT

See `LICENSE` and `THIRD_PARTY_LICENSES.txt` for details.

---

**Download v0.0.2 now and enjoy a stable, reliable experience!** 🎉
