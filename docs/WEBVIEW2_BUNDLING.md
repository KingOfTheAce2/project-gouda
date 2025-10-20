# WebView2 Bundling Strategy

## Overview

BEAR LLM AI bundles the WebView2 runtime with the installer to ensure a seamless installation experience without requiring internet connectivity during installation.

## Configuration

### Current Setup (Industry Standard)

```json
"webviewInstallMode": {
  "type": "embedBootstrapper",
  "silent": true
}
```

This configuration embeds the WebView2 Evergreen Bootstrapper (~2MB) directly into the installer.

## WebView2 Installation Modes

### 1. `embedBootstrapper` (RECOMMENDED - Current)
- **What it does**: Embeds the WebView2 bootstrapper (~2MB) in the installer
- **Installation behavior**:
  - Downloads and installs WebView2 runtime during first launch if not present
  - Requires internet connection on first launch if WebView2 not installed
  - Silent installation with no user interaction
- **Pros**:
  - Industry standard approach
  - Small installer size increase (~2MB)
  - Automatic installation with fallback
  - Works offline if WebView2 already installed
- **Cons**:
  - First launch requires internet if WebView2 missing
  - ~100MB download on first launch (one-time)

### 2. `fixedRuntime` (Enterprise Option)
- **What it does**: Bundles the full WebView2 Fixed Version Runtime (~150MB)
- **Installation behavior**:
  - Completely offline installation
  - App uses its own isolated WebView2 version
  - No shared runtime with other apps
- **Pros**:
  - 100% offline installation
  - Version control and isolation
  - No dependency on system WebView2
- **Cons**:
  - Installer size increases by ~150MB
  - Each app has its own WebView2 copy (disk space)
  - Must manually update WebView2 with app updates

### 3. `downloadBootstrapper` (DEPRECATED - Previous)
- **What it does**: Downloads bootstrapper at runtime
- **Installation behavior**:
  - Requires internet during installation
  - Downloads WebView2 if not present
- **Cons**:
  - Not industry standard
  - Installation fails without internet
  - Poor user experience
  - Not recommended for production apps

## Build Requirements

### For `embedBootstrapper` (Current Setup)

No additional build steps required. Tauri automatically:
1. Downloads the WebView2 bootstrapper during build
2. Embeds it into the NSIS/MSI installer
3. Configures silent installation on first launch

### For `fixedRuntime` (Enterprise Option)

If you need complete offline installation:

1. Download the Fixed Version Runtime:
```powershell
# Download WebView2 Fixed Version Runtime
$version = "130.0.2849.56"  # Use latest stable version
$url = "https://msedge.sf.dl.delivery.mp.microsoft.com/filestreamingservice/files/$version/MicrosoftEdgeWebView2RuntimeInstaller$version.exe"
Invoke-WebRequest -Uri $url -OutFile "src-tauri/WebView2Runtime.exe"
```

2. Update `tauri.conf.json`:
```json
"webviewInstallMode": {
  "type": "fixedRuntime",
  "path": "WebView2Runtime.exe"
}
```

## CI/CD Integration

The current `embedBootstrapper` configuration works automatically in CI/CD:

```yaml
# .github/workflows/build.yml (example)
- name: Build Tauri App
  run: |
    npm run tauri build
  # WebView2 bootstrapper is automatically embedded
```

For `fixedRuntime`, add download step:
```yaml
- name: Download WebView2 Runtime
  if: runner.os == 'Windows'
  run: |
    $version = "130.0.2849.56"
    $url = "https://msedge.sf.dl.delivery.mp.microsoft.com/filestreamingservice/files/$version/MicrosoftEdgeWebView2RuntimeInstaller$version.exe"
    Invoke-WebRequest -Uri $url -OutFile "src-tauri/WebView2Runtime.exe"
```

## Testing

### Test Embedded Bootstrapper Installation

1. **Clean Environment Test**:
   ```powershell
   # Remove existing WebView2 (for testing only!)
   Get-AppxPackage *WebView* | Remove-AppxPackage

   # Install your app
   .\bear-llm-ai_0.0.6_x64-setup.exe

   # Launch app - should automatically install WebView2
   ```

2. **Offline Test**:
   - Disable internet connection
   - Install app on machine with WebView2 already present
   - Should work without internet

3. **First Launch Test**:
   - Install on clean machine without WebView2
   - Enable internet
   - Launch app - WebView2 should download and install silently

## Recommendations

### For Most Users (Current Configuration)
- ✅ Use `embedBootstrapper`
- Small installer size
- Industry standard
- Automatic installation
- Works with Windows Update managed WebView2

### For Enterprise/Offline Deployments
- Consider `fixedRuntime`
- Complete offline installation
- Controlled version management
- No external dependencies
- Larger installer but guaranteed to work

## Size Comparison

| Mode | Installer Size Increase | Disk Space After Install | Internet Required |
|------|------------------------|-------------------------|-------------------|
| downloadBootstrapper (old) | 0 MB | ~100 MB (shared) | During install |
| embedBootstrapper (current) | ~2 MB | ~100 MB (shared) | First launch only |
| fixedRuntime | ~150 MB | ~150 MB (isolated) | Never |

## WebView2 Runtime Details

- **Evergreen Runtime**: Automatically updates via Windows Update
- **Fixed Version Runtime**: App-specific, must be updated manually
- **System Requirements**: Windows 7 SP1 or later
- **Architecture**: x64, x86, ARM64 supported

## Troubleshooting

### "WebView2 not found" Error
- Ensure `embedBootstrapper` is configured in `tauri.conf.json`
- Check internet connection on first launch
- Manually install WebView2: https://go.microsoft.com/fwlink/p/?LinkId=2124703

### Installation Fails Silently
- Check Windows Event Viewer for WebView2 installation errors
- Ensure user has permissions to install software
- Try running installer as administrator

### Version Conflicts
- Evergreen runtime is shared system-wide
- If conflicts occur, consider switching to `fixedRuntime`
- Check installed version: `reg query "HKLM\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" /v pv`

## References

- [Tauri WebView2 Documentation](https://v2.tauri.app/distribute/windows/#webview2)
- [Microsoft WebView2 Documentation](https://docs.microsoft.com/en-us/microsoft-edge/webview2/)
- [WebView2 Runtime Download](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)
