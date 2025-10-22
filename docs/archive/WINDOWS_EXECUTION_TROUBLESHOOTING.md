# Windows Execution Troubleshooting - Version 0.0.11

## Issue: Application Cannot Execute on Windows

### Symptom
When trying to run BEAR LLM AI v0.0.11 on Windows, the application fails to start with an error message like:
- "This app can't run on your PC"
- "The application was unable to start correctly"
- Silent failure (nothing happens when clicking the executable)

### Root Causes Analysis

#### 1. Missing Visual C++ Runtime
**Most Common Cause**: Tauri applications require Microsoft Visual C++ Redistributable

**Solution**:
1. Download and install [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe)
2. Install both x64 and x86 versions if needed
3. Restart your computer
4. Try running BEAR LLM AI again

#### 2. WebView2 Runtime Missing
**Cause**: Windows needs WebView2 Runtime for Tauri apps

**Check**:
- Windows 11: WebView2 is pre-installed
- Windows 10: May need manual installation

**Solution**:
1. Download [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)
2. Install the Evergreen Standalone Installer
3. Restart your computer
4. Try running BEAR LLM AI again

#### 3. Architecture Mismatch
**Cause**: Application built for different architecture (x64 vs ARM64)

**Check**:
- Right-click "This PC" → Properties
- Check "System type" (64-bit or ARM-based)

**Solution**:
- Download the correct installer for your system:
  - `BEAR-LLM-AI_0.0.11_x64-setup.exe` for 64-bit Intel/AMD
  - `BEAR-LLM-AI_0.0.11_arm64-setup.exe` for ARM64

#### 4. Corrupted Download
**Cause**: Incomplete or corrupted installer download

**Solution**:
1. Delete the downloaded installer
2. Clear your browser cache
3. Re-download from official source
4. Verify file size matches expected size
5. Try installing again

#### 5. Antivirus/SmartScreen Blocking
**Cause**: Windows Defender or antivirus blocking unsigned executable

**Solution**:
1. Check Windows Security → Virus & threat protection → Protection history
2. If blocked, click "Allow on device"
3. Or: Right-click installer → Properties → Unblock → Apply → OK
4. Try running again

#### 6. Insufficient Permissions
**Cause**: Installation location requires elevated permissions

**Solution for Current User Install**:
1. Simply double-click the installer (no admin needed)
2. It will install to: `%LOCALAPPDATA%\Programs\BEAR LLM AI`

**Solution for All Users Install**:
1. Right-click installer → "Run as administrator"
2. Choose "Install for all users"
3. It will install to: `C:\Program Files\BEAR LLM AI`

## Step-by-Step Diagnostic Process

### Step 1: Verify System Requirements
```
Windows Version: Windows 10 version 1809 or later, Windows 11
Architecture: x64 or ARM64
RAM: 4GB minimum
Disk Space: 500MB free
```

### Step 2: Check Event Viewer
1. Press Win + X → Event Viewer
2. Navigate to: Windows Logs → Application
3. Look for errors related to "BEAR LLM AI" or "BEAR-LLM-AI"
4. Note the error code and message

Common Error Codes:
- **0xc000007b**: Missing Visual C++ Runtime
- **0xc0000135**: Missing DLL dependency
- **0x80070005**: Access denied

### Step 3: Check Dependencies
Run this in PowerShell as Administrator:
```powershell
# Check Visual C++ Redistributable
Get-WmiObject -Class Win32_Product | Where-Object {$_.Name -like "*Visual C++*"}

# Check WebView2 Runtime
Get-ItemProperty HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5} -ErrorAction SilentlyContinue
```

### Step 4: Test Minimal Installation
1. Create a new local user account (for testing)
2. Log in as the new user
3. Try installing BEAR LLM AI
4. If it works: Issue is with your main user profile
5. If it fails: System-wide issue

## Quick Fix Commands

### Fix 1: Install All Dependencies (PowerShell Admin)
```powershell
# Download and install Visual C++ Redistributable
Invoke-WebRequest -Uri "https://aka.ms/vs/17/release/vc_redist.x64.exe" -OutFile "$env:TEMP\vc_redist.x64.exe"
Start-Process -FilePath "$env:TEMP\vc_redist.x64.exe" -Args "/install /quiet /norestart" -Wait

# Download and install WebView2 Runtime
Invoke-WebRequest -Uri "https://go.microsoft.com/fwlink/p/?LinkId=2124703" -OutFile "$env:TEMP\MicrosoftEdgeWebview2Setup.exe"
Start-Process -FilePath "$env:TEMP\MicrosoftEdgeWebview2Setup.exe" -Args "/silent /install" -Wait

Write-Host "Dependencies installed. Please restart your computer."
```

### Fix 2: Repair Installation
```powershell
# Uninstall current version
$app = Get-WmiObject -Class Win32_Product | Where-Object {$_.Name -like "*BEAR LLM AI*"}
if ($app) { $app.Uninstall() }

# Clean up residual files
Remove-Item "$env:LOCALAPPDATA\BEAR LLM AI" -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item "$env:APPDATA\com.bearllm.ai" -Recurse -Force -ErrorAction SilentlyContinue

# Reinstall
# Download and run installer again
```

## Installation Checklist

Before installing BEAR LLM AI v0.0.11:

- [ ] Windows 10 (1809+) or Windows 11
- [ ] Visual C++ Redistributable 2015-2022 installed
- [ ] WebView2 Runtime installed
- [ ] 500MB free disk space
- [ ] User account has file creation permissions
- [ ] Antivirus temporarily disabled (for testing)
- [ ] Previous version uninstalled (if upgrading)

After installation:

- [ ] Shortcut appears in Start Menu
- [ ] Desktop shortcut created (if selected)
- [ ] Files exist in installation directory
- [ ] Can launch from Start Menu
- [ ] Application window appears
- [ ] No crash or error messages

## Common Solutions by Error Message

### "This app can't run on your PC"
→ Install Visual C++ Redistributable (most common)

### "VCRUNTIME140.dll was not found"
→ Install Visual C++ Redistributable

### "Application was unable to start correctly (0xc000007b)"
→ Install both x86 and x64 Visual C++ Redistributable

### Silent failure (nothing happens)
→ Check Event Viewer for specific error
→ Try running as administrator
→ Check antivirus quarantine

### "WebView2 Runtime not found"
→ Install WebView2 Runtime Evergreen

## Developer Notes

### Build Configuration (v0.0.11)
```json
{
  "bundle": {
    "targets": ["nsis", "msi"],
    "windows": {
      "webviewInstallMode": {
        "type": "downloadBootstrapper",
        "silent": true
      }
    }
  }
}
```

### Known Issues
1. **NSIS Installer**: Custom template removed in v0.0.11 (now using Tauri built-in)
2. **Dependencies**: Requires VC++ Runtime (not bundled)
3. **WebView2**: Downloaded automatically by installer (requires internet)

### Testing Recommendations
1. Test on clean Windows 10 VM
2. Test on Windows 11 with WebView2 pre-installed
3. Test without VC++ Runtime to verify error handling
4. Test with antivirus enabled
5. Test both user-level and admin installation modes

## Support Resources

- **Documentation**: `/docs` folder in repository
- **GitHub Issues**: Report execution problems with Event Viewer logs
- **System Info**: Include Windows version, architecture, installed runtimes

## Version History

### v0.0.11 Changes
- Removed custom NSIS installer template
- Using Tauri built-in installer with `installMode: "both"`
- Fixed Rust compilation warnings
- Improved error handling in initialization code

### Known Execution Issues
- Requires VC++ Runtime (not auto-installed)
- WebView2 download requires internet connection
- First launch may be slow while WebView2 initializes

---

**Last Updated**: 2025-10-21
**Version**: 0.0.11
**Platform**: Windows 10/11 (x64)
