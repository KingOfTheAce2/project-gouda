# WebView2 Configuration Guide

This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).

## Understanding WebView Technologies

### ✅ What You're Using: **WebView2** (Modern Chromium)

Your application uses:
- **Wry 0.46.1**: Cross-platform webview abstraction
- **WebView2-COM 0.33.0**: Microsoft's Chromium-based WebView2
- **Tauri 2.x**: Modern desktop framework

### ❌ What You're NOT Using: **EBWebView/EdgeHTML** (Deprecated)

EBWebView (EdgeHTML) was discontinued in 2018 and is NOT used by Tauri.

## Common Confusion: The "EBWebView" Folder

**Why you might see "EBWebView" references:**

Microsoft's WebView2 uses the folder name "EBWebView" for **backward compatibility**, even though the actual technology is modern Chromium-based WebView2. This is purely a naming convention, not an indicator of the underlying engine.

**To avoid confusion, this application now uses:**
```rust
// src-tauri/src/init.rs:20
let webview2_dir = app_data_dir.join("WebView2");
```

## WebView2 Runtime Requirements

### Windows Requirements
- **OS**: Windows 7, 8, 10, 11 (10+ recommended)
- **Runtime**: WebView2 Runtime (automatically installed via downloadBootstrapper)

### Automatic Installation
Your `tauri.conf.json` is configured to automatically download WebView2:

```json
"webviewInstallMode": {
  "type": "downloadBootstrapper",
  "silent": true
}
```

### Manual Installation Options

#### Option 1: Embedded Runtime (Larger installer)
```json
"webviewInstallMode": {
  "type": "embedBootstrapper"
}
```

#### Option 2: Fixed Version (Most control)
```json
"webviewInstallMode": {
  "type": "fixedRuntime",
  "path": "path/to/webview2/runtime"
}
```

## Verifying WebView2 Usage

### Check Dependencies
```bash
./scripts/verify-webview2.sh
```

### Check Runtime Version (Windows)
```powershell
# In PowerShell
Get-ItemProperty -Path "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
```

### Check User Agent String
Add this to your frontend code:
```javascript
console.log('User Agent:', navigator.userAgent);
// Should contain "Chrome" or "Edg" (Chromium-based Edge)
```

## Troubleshooting Windows Issues

### Issue 1: "WebView2 not found"
**Solution**: Ensure `webviewInstallMode` is configured in `tauri.conf.json`

### Issue 2: "Permission denied" on WebView2 folder
**Solution**: Check `init.rs:15-68` - permission handling is already implemented

### Issue 3: Outdated WebView2 Runtime
**Solution**:
```bash
# Update WebView2 Runtime
winget install Microsoft.EdgeWebView2Runtime
```

### Issue 4: Corporate/Restricted Environments
**Solution**: Use `embedBootstrapper` or `fixedRuntime` modes

## Performance Optimization

### Enable Hardware Acceleration
```json
// tauri.conf.json
"app": {
  "windows": [{
    "transparent": false,  // Better performance
    "decorations": true
  }]
}
```

### Disable DevTools in Production
```rust
// Cargo.toml
[features]
custom-protocol = ["tauri/custom-protocol"]
# DevTools are automatically disabled in production builds
```

## Platform-Specific Notes

### Windows 7/8
- Requires manual WebView2 Runtime installation
- May have limited feature support

### Windows 10/11
- WebView2 Runtime often pre-installed
- Full feature support

### Corporate Networks
- May need proxy configuration
- Consider `embedBootstrapper` mode

## Additional Resources

- [WebView2 Documentation](https://docs.microsoft.com/en-us/microsoft-edge/webview2/)
- [Tauri WebView Configuration](https://tauri.app/v1/api/config#bundleconfig.windows)
- [Wry Repository](https://github.com/tauri-apps/wry)

## Support

If you encounter issues:

1. Run `./scripts/verify-webview2.sh`
2. Check Windows Event Viewer for WebView2 errors
3. Enable debug logging in `tauri.conf.json`
4. Review logs in `%APPDATA%\[app-name]\logs\`

---

**Summary**: You ARE using modern WebView2 (Chromium). The "EBWebView" folder name is just historical naming, not an indicator of legacy technology.
