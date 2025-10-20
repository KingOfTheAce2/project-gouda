# Windows Installation Fixes for v0.0.6

## Issues Fixed

### 1. Installation Mode Changed: `currentUser` → `perMachine`

**Problem:**
- `currentUser` mode can cause permissions issues on Windows 10/11
- App may fail to start due to insufficient privileges
- WebView2 installation may fail silently

**Solution:**
```json
"nsis": {
  "installMode": "perMachine"
}
```

**Impact:**
- Requires administrator privileges during installation
- More reliable installation across Windows versions
- Proper WebView2 runtime installation
- Better compatibility with enterprise environments

### 2. Added `allowDowngrades` Flag

**Problem:**
- Users couldn't downgrade versions if needed
- Testing earlier versions was difficult
- Update rollbacks would fail

**Solution:**
```json
"allowDowngrades": true
```

**Impact:**
- Allows version rollback if needed
- Easier testing of different versions
- Better development workflow

### 3. NSIS Installer Optimization

**Added:**
```json
"nsis": {
  "installMode": "perMachine",
  "languages": ["English"],
  "displayLanguageSelector": false,
  "compressionLevel": "high"
}
```

**Benefits:**
- Faster installation with high compression
- Simplified installer (English only)
- Smaller installer size
- More reliable installation process

## Common Windows Installation Errors

### Error: "Application failed to start"

**Causes:**
1. WebView2 not installed
2. Permissions issues
3. Corrupted installation

**Solutions:**
1. Install WebView2 manually: https://go.microsoft.com/fwlink/p/?LinkId=2124703
2. Run installer as Administrator
3. Uninstall completely and reinstall

### Error: "NSIS Error: Error launching installer"

**Causes:**
1. Corrupted download
2. Antivirus blocking
3. Insufficient permissions

**Solutions:**
1. Re-download installer
2. Temporarily disable antivirus
3. Right-click → "Run as Administrator"

### Error: "The application was unable to start correctly (0xc000007b)"

**Causes:**
1. Missing Visual C++ Redistributables
2. 32-bit/64-bit mismatch
3. Corrupted system files

**Solutions:**
1. Install VC++ Redistributables: https://aka.ms/vs/17/release/vc_redist.x64.exe
2. Ensure you downloaded the correct architecture
3. Run `sfc /scannow` in Command Prompt (Admin)

## Testing Windows Installers

### Clean Installation Test

```powershell
# 1. Uninstall existing version
Get-WmiObject -Class Win32_Product | Where-Object { $_.Name -like "*BEAR LLM*" } | ForEach-Object { $_.Uninstall() }

# 2. Clear app data (optional, for clean test)
Remove-Item -Path "$env:APPDATA\com.bearllm.ai" -Recurse -Force

# 3. Install new version
.\bear-llm-ai_0.0.6_x64-setup.exe

# 4. Verify installation
Get-WmiObject -Class Win32_Product | Where-Object { $_.Name -like "*BEAR LLM*" }
```

### WebView2 Installation Verification

```powershell
# Check if WebView2 is installed
$webview2 = Get-ItemProperty -Path "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -ErrorAction SilentlyContinue

if ($webview2) {
    Write-Host "WebView2 Version: $($webview2.pv)" -ForegroundColor Green
} else {
    Write-Host "WebView2 not found" -ForegroundColor Red
}
```

## Build Configuration Changes

### tauri.conf.json Changes

```json
{
  "bundle": {
    "windows": {
      "allowDowngrades": true,  // NEW: Allow version downgrade
      "nsis": {
        "installMode": "perMachine",  // CHANGED: Was "currentUser"
        "languages": ["English"],  // NEW: Simplified language selection
        "displayLanguageSelector": false,  // NEW: Hide language selector
        "compressionLevel": "high"  // NEW: Better compression
      }
    }
  }
}
```

### GitHub Actions Workflow

**New:** `windows-release.yml` - Dedicated Windows builds
- Separate workflow for Windows testing
- Faster iteration on Windows-specific issues
- Better error reporting
- Artifact upload for testing

**Updated:** `main.yml` - Removed Windows builds
- Focus on macOS and Linux
- Windows builds use dedicated workflow
- Prevents cross-platform build conflicts

## Installation Requirements

### Minimum Requirements
- Windows 10 (Build 1809 or later)
- 64-bit processor
- ~200MB disk space
- Administrator privileges (for installation only)

### Recommended
- Windows 10 20H2 or Windows 11
- 4GB RAM
- SSD storage
- Internet connection (first launch for WebView2)

## Troubleshooting Commands

### Check Installation Status
```powershell
# Get installed app info
Get-WmiObject -Class Win32_Product | Where-Object { $_.Name -like "*BEAR LLM*" } | Format-List

# Check installation directory
Get-ChildItem "C:\Program Files\BEAR LLM AI"
```

### View Installation Logs
```powershell
# NSIS installation logs
Get-Content "$env:TEMP\bear-llm-ai-install.log"

# Windows Event Viewer
Get-EventLog -LogName Application -Source "BEAR LLM AI" -Newest 10
```

### Repair Installation
```powershell
# Reinstall with force
.\bear-llm-ai_0.0.6_x64-setup.exe /S /REINSTALL

# Or uninstall and clean install
.\bear-llm-ai_0.0.6_x64-setup.exe /S /UNINSTALL
Remove-Item -Path "C:\Program Files\BEAR LLM AI" -Recurse -Force
.\bear-llm-ai_0.0.6_x64-setup.exe
```

## Development Notes

### Building Locally

```bash
# Build Windows installer
npm run tauri build -- --target x86_64-pc-windows-msvc

# Output location
# src-tauri/target/release/bundle/nsis/bear-llm-ai_0.0.6_x64-setup.exe
```

### Testing Changes

1. Build installer locally
2. Test on clean Windows VM or machine
3. Verify WebView2 installation
4. Check app starts correctly
5. Test updater functionality

### CI/CD Workflow

```bash
# Trigger Windows release manually
# Go to: Actions → windows-release → Run workflow

# Or push to release branch
git push origin main:release
```

## References

- [Tauri Windows Installer Docs](https://v2.tauri.app/distribute/windows-installer/)
- [NSIS Documentation](https://nsis.sourceforge.io/Docs/)
- [WebView2 Runtime Download](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)
- [VC++ Redistributables](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist)

---

**Last Updated:** v0.0.6
**Applies To:** Windows 10 (1809+), Windows 11
