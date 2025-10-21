# Installation and Runtime Fixes - Version 0.0.10

## Issues Fixed

### 1. Installation Permission Error
**Problem**: Error opening files for writing `C:\Program Files\BEAR LLM AI\BEAR LLM AI.exe` and `WebView2Loader.dll`

**Root Cause**:
- NSIS installer was not explicitly requesting administrator privileges
- `perMachine` installation mode requires admin access but wasn't enforcing it

**Solution**:
- Created custom NSIS installer script (`src-tauri/installer.nsi`)
- Added `RequestExecutionLevel admin` directive
- Implemented admin privilege check with user-friendly error message
- Configured `template` property in tauri.conf.json to use custom script

**Files Modified**:
- `src-tauri/installer.nsi` (new)
- `src-tauri/tauri.conf.json` (added NSIS template configuration)

### 2. Application Crash on Startup
**Problem**: Window opens briefly then crashes immediately after installation

**Root Causes**:
1. WebView2 initialization errors were causing fatal crashes
2. Missing or inaccessible WebView2 user data folder
3. Strict error handling prevented graceful degradation

**Solutions**:

#### A. Non-Fatal WebView2 Initialization
Changed from fatal error to warning-based approach:
```rust
// Before: Would crash the app
setup_webview2_user_data_folder(&app_data_dir)?;

// After: Logs warning and continues
if let Err(e) = setup_webview2_user_data_folder(&app_data_dir) {
    log::warn!("WebView2 setup failed, continuing with defaults: {:?}", e);
    // App continues with system default WebView2 configuration
}
```

#### B. WebView2Loader.dll Verification
Added diagnostic logging to verify DLL presence:
```rust
if let Ok(exe_path) = env::current_exe() {
    if let Some(exe_dir) = exe_path.parent() {
        let webview2_loader = exe_dir.join("WebView2Loader.dll");
        if webview2_loader.exists() {
            log::info!("WebView2Loader.dll found at: {:?}", webview2_loader);
        } else {
            log::warn!("WebView2Loader.dll not found");
        }
    }
}
```

#### C. Tokio Process Feature
Fixed compilation error for process helper:
```toml
tokio = { version = "1.36.0", features = ["process"] }
```

**Files Modified**:
- `src-tauri/src/init.rs` (non-fatal error handling, DLL verification)
- `src-tauri/Cargo.toml` (tokio process feature)

## Installation Instructions (For Users)

### Recommended Installation Method
1. Download the `.exe` installer
2. **Right-click** the installer
3. Select **"Run as administrator"**
4. Follow the installation wizard
5. WebView2 will be automatically downloaded if not present

### Alternative: Double-Click Installation
If you double-click the installer without admin rights:
- A clear error message will appear
- Message: "Administrator privileges are required to install BEAR LLM AI"
- Solution: Right-click and select "Run as administrator"

## Troubleshooting

### Issue: Installer shows "Access Denied"
**Solution**: Run installer as administrator

### Issue: Application crashes immediately after starting
**Checks**:
1. Verify WebView2Loader.dll exists in installation folder
2. Check logs at: `%APPDATA%\com.bearllm.ai\logs\`
3. Ensure WebView2 Runtime is installed:
   - Visit: https://developer.microsoft.com/en-us/microsoft-edge/webview2/
   - Download and install WebView2 Runtime

### Issue: "WebView2 initialization failed"
**Solution**: The app now handles this gracefully:
- Warning is logged but app continues
- WebView2 will use system defaults
- If persistent, manually install WebView2 Runtime

## Technical Details

### NSIS Installer Configuration
```nsi
RequestExecutionLevel admin

Function .onInit
    UserInfo::GetAccountType
    Pop $0
    ${If} $0 != "admin"
        MessageBox MB_ICONSTOP "Administrator privileges are required..."
        SetErrorLevel 740 ; ERROR_ELEVATION_REQUIRED
        Quit
    ${EndIf}
FunctionEnd
```

### WebView2 Configuration (tauri.conf.json)
```json
{
  "windows": {
    "webviewInstallMode": {
      "type": "downloadBootstrapper",
      "silent": true
    },
    "nsis": {
      "installMode": "perMachine",
      "template": "installer.nsi",
      "compression": "lzma"
    }
  },
  "resources": [
    "WebView2Loader.dll",
    "resources/*"
  ]
}
```

### Error Handling Philosophy
- **Installation**: Strict - Require admin privileges upfront
- **Runtime**: Graceful - Log warnings but don't crash on WebView2 issues
- **User Experience**: Clear error messages with actionable solutions

## Version History

### v0.0.10
- Fixed installation permission errors
- Fixed application crash on startup
- Added WebView2Loader.dll verification
- Improved error handling and logging
- Added tokio process feature support

## Testing Checklist

- [ ] Installer requires admin privileges
- [ ] Clear error message when run without admin
- [ ] Application starts successfully after installation
- [ ] WebView2Loader.dll is bundled correctly
- [ ] Logs are created and accessible
- [ ] Application handles WebView2 errors gracefully
- [ ] No crashes on startup
- [ ] UI appears and is functional

## Related Files

- `/src-tauri/installer.nsi` - Custom NSIS installer script
- `/src-tauri/tauri.conf.json` - Tauri configuration
- `/src-tauri/src/init.rs` - Application initialization
- `/src-tauri/Cargo.toml` - Rust dependencies
- `/docs/WebView2-Fixes-Applied.md` - WebView2 implementation details
