# WebView2 Crash Fix - BEAR LLM AI

This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).

## Problem: Application Crashes on Windows

**Root Cause**: WebView2 Runtime is not bundled with the installer.

### Previous Configuration (BROKEN):
```json
"webviewInstallMode": {
  "type": "downloadBootstrapper",
  "silent": true
}
```

**Why this failed:**
- Only downloads a tiny bootstrapper (~5KB)
- Requires internet connection at install time to download WebView2 (~100MB)
- If download fails or times out → **application crashes**
- No offline support
- No WebView2 DLLs included in installer

## Solution: Embed WebView2 Bootstrapper

### New Configuration (FIXED):
```json
"webviewInstallMode": {
  "type": "embedBootstrapper",
  "silent": true
}
```

**Benefits:**
- ✅ Embeds the WebView2 bootstrapper directly in your installer
- ✅ Works offline (downloads runtime only if not present)
- ✅ Reliable installation on all Windows systems
- ✅ Automatic WebView2 runtime installation
- ✅ Smaller than embedding full runtime (~150KB bootstrapper vs ~100MB full runtime)

## Alternative Options

### Option 1: Embed Full Runtime (Maximum Reliability)
```json
"webviewInstallMode": {
  "type": "fixedRuntime",
  "path": "path/to/Microsoft.WebView2.FixedVersionRuntime.cab"
}
```

**Pros:**
- Complete offline installation
- No external downloads needed
- Guaranteed version consistency

**Cons:**
- Installer becomes ~100MB larger
- Need to manually download and maintain WebView2 runtime
- Requires regular updates

### Option 2: Skip Installation (Development Only)
```json
"webviewInstallMode": {
  "type": "skip"
}
```

**⚠️ WARNING**: Only use for testing on machines with WebView2 already installed!

## How to Build Fixed Installer

### 1. Clean Previous Build
```bash
cd src-tauri
rm -rf target/release/bundle
```

### 2. Rebuild Application
```bash
# From project root
npm run build
```

Or with Tauri CLI:
```bash
cd src-tauri
cargo tauri build
```

### 3. Verify Installer Contents
Your new installer will now include:
- `BEAR LLM AI.exe` - Main application
- `Uninstall BEAR LLM AI.exe` - Uninstaller
- `resources/` - Application resources
- **WebView2 bootstrapper embedded** (automatically installed)

### 4. Test Installation
1. Uninstall previous version completely
2. Delete `%LOCALAPPDATA%\Programs\BEAR LLM AI` if it exists
3. Install new version
4. Run application - should work without crashes

## Troubleshooting

### Issue: Still Crashing After Reinstall
**Solution**: Manually install WebView2 Runtime first
```powershell
# Option 1: Windows Update
winget install Microsoft.EdgeWebView2Runtime

# Option 2: Download directly from Microsoft
# Visit: https://developer.microsoft.com/en-us/microsoft-edge/webview2/
```

### Issue: Installer Size Too Large
**Solution**: Use `embedBootstrapper` (recommended, already set)
- Adds only ~150KB to installer
- Downloads runtime only when needed

### Issue: Corporate Environment Blocks Downloads
**Solution**: Switch to `fixedRuntime` mode
1. Download WebView2 Fixed Version Runtime
2. Place in project directory
3. Update `tauri.conf.json` path
4. Rebuild

## WebView2 Runtime Locations

After successful installation, WebView2 will be in:

**User Installation:**
```
%LOCALAPPDATA%\Microsoft\EdgeWebView2\Application\
```

**System Installation:**
```
C:\Program Files (x86)\Microsoft\EdgeWebView2\Application\
```

**Application Data (Your App):**
```
%APPDATA%\BEAR LLM AI\WebView2\
```

## Build Output Verification

After rebuilding, check:
```bash
ls -lh src-tauri/target/release/bundle/nsis/*.exe
```

Your installer should be:
- **Old (broken)**: ~5-10 MB (no WebView2)
- **New (fixed)**: ~5-10.2 MB (with embedded bootstrapper)

The size increase is minimal but critical for functionality.

## Testing Checklist

Before release:
- [ ] Clean uninstall of old version
- [ ] Install new version on clean Windows VM/system
- [ ] Verify application launches without errors
- [ ] Check WebView2 runtime is installed (check registry or file system)
- [ ] Test on Windows 10 and Windows 11
- [ ] Test on system WITHOUT existing WebView2 runtime

## Additional Notes

### Why Your App Was Crashing
1. Installer used `downloadBootstrapper` (minimal approach)
2. No WebView2 DLLs in application folder
3. Application tried to create WebView2 instance
4. WebView2 runtime not found → **CRASH**

### What Changed
1. Changed to `embedBootstrapper` in `tauri.conf.json`
2. Installer now includes WebView2 bootstrapper
3. Bootstrapper installs runtime automatically during app installation
4. Application can now find and use WebView2

### Windows Compatibility
- Windows 7+: ✅ Supported (with WebView2 runtime)
- Windows 8.1+: ✅ Supported
- Windows 10+: ✅ Fully supported (may have runtime pre-installed)
- Windows 11: ✅ Fully supported (runtime pre-installed)

---

**Summary**: Your app was crashing because WebView2 Runtime wasn't included. The fix embeds the bootstrapper, ensuring WebView2 is always available.

**Next Steps**:
1. Rebuild your installer
2. Test on a clean Windows system
3. Distribute the new installer to users
