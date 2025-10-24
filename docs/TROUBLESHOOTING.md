# BEAR LLM AI - Troubleshooting Guide

## WebView2 Initialization Issues

If the application fails to start or shows a blank window, follow these steps:

### Quick Fix (Windows)

1. **Run the fix script**:
   ```batch
   scripts\fix-webview2-windows.bat
   ```
   This script will automatically:
   - Clear corrupted WebView2 cache
   - Check for required dependencies
   - Verify folder permissions
   - Show log file locations

2. **Restart your computer** after running the script

3. **Try launching the application again**

### Manual Fix Steps

#### Step 1: Clear WebView2 Cache

The WebView2 cache can become corrupted. To clear it:

1. Close all instances of BEAR LLM AI
2. Open File Explorer
3. Navigate to: `%LOCALAPPDATA%\BEAR LLM AI\WebView2`
4. Delete the entire `WebView2` folder
5. Restart the application

**Location**: `C:\Users\[YourUsername]\AppData\Local\BEAR LLM AI\WebView2`

#### Step 2: Verify WebView2 Runtime

Check if WebView2 Runtime is installed:

1. Press `Win + R`
2. Type: `regedit`
3. Navigate to: `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}`
4. Check for `pv` value (version)

**If not installed**, download from:
- **Evergreen Bootstrapper**: https://go.microsoft.com/fwlink/p/?LinkId=2124703
- **Evergreen Standalone**: https://go.microsoft.com/fwlink/p/?LinkId=2124704

#### Step 3: Install Visual C++ Runtime

BEAR LLM AI requires both x64 and x86 versions:

1. **Download and install x64**: https://aka.ms/vs/17/release/vc_redist.x64.exe
2. **Download and install x86**: https://aka.ms/vs/17/release/vc_redist.x86.exe
3. **Restart your computer**

#### Step 4: Check Folder Permissions

Ensure the application can write to its data folder:

1. Navigate to: `%LOCALAPPDATA%\BEAR LLM AI`
2. Right-click → Properties → Security tab
3. Ensure your user account has "Full Control"
4. If not, click "Edit" and grant permissions

#### Step 5: Check Log Files

Review the log files for detailed error information:

**Log file locations**:
- `%LOCALAPPDATA%\BEAR LLM AI\preinit.log` - Pre-initialization checks
- `%LOCALAPPDATA%\BEAR LLM AI\fatal_error.log` - Fatal errors during startup
- `%LOCALAPPDATA%\BEAR LLM AI\crash.log` - Application crashes
- `%LOCALAPPDATA%\BEAR LLM AI\diagnostics.log` - Dependency diagnostics

**To view logs**:
1. Press `Win + R`
2. Type: `%LOCALAPPDATA%\BEAR LLM AI`
3. Open the log files with Notepad

### Common Error Messages

#### "WebView2 Runtime NOT found"

**Solution**: Install WebView2 Runtime (see Step 2 above)

#### "Visual C++ Runtime NOT found"

**Solution**: Install VC++ Runtime (see Step 3 above)

#### "Cannot create WebView2 folder"

**Solutions**:
1. Check folder permissions (see Step 4 above)
2. Run the application as administrator (right-click → Run as administrator)
3. Check if antivirus is blocking folder creation

#### Application starts but shows blank window

**Solutions**:
1. Clear WebView2 cache (see Step 1 above)
2. Disable hardware acceleration:
   - Set environment variable: `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--disable-gpu`
3. Update graphics drivers
4. Check Windows Update for pending updates

#### Application crashes immediately

**Solutions**:
1. Check crash.log for details
2. Ensure all dependencies are installed
3. Try running as administrator
4. Temporarily disable antivirus/firewall
5. Reinstall the application

### System Requirements

- **Windows**: 10 (1809+) or 11
- **WebView2 Runtime**: Latest version
- **Visual C++ Runtime**: 2015-2022 (x64 and x86)
- **Minimum RAM**: 4 GB
- **Minimum Disk Space**: 500 MB

### Advanced Troubleshooting

#### Enable Verbose Logging

To get more detailed logs:

1. Set environment variable before running:
   ```batch
   set RUST_LOG=debug
   "BEAR LLM AI.exe"
   ```

#### Check Windows Event Viewer

1. Press `Win + X` → Event Viewer
2. Navigate to: Windows Logs → Application
3. Look for errors from "BEAR LLM AI"

#### Reinstall WebView2 Runtime

If WebView2 is corrupted:

1. Download WebView2 Standalone installer
2. Run installer with `/silent /install` flags
3. Restart your computer
4. Launch BEAR LLM AI

#### Run System File Checker

To repair corrupted system files:

1. Open Command Prompt as Administrator
2. Run: `sfc /scannow`
3. Wait for completion (may take 30+ minutes)
4. Restart your computer

### Getting Help

If none of these solutions work:

1. **Collect logs**: Copy all files from `%LOCALAPPDATA%\BEAR LLM AI`
2. **System info**: Run `msinfo32` and save as .nfo file
3. **Create issue**: Open GitHub issue with logs and system info
4. **Contact support**: Include detailed description of the problem

### Known Issues

#### Issue: Application shows briefly then disappears

**Workaround**: Check fatal_error.log for initialization failures

#### Issue: High memory usage in WebView2

**Workaround**: Clear WebView2 cache regularly

#### Issue: Application won't start after Windows update

**Solution**: Reinstall Visual C++ Runtime and WebView2

### Prevention Tips

1. **Keep dependencies updated**:
   - Update Windows regularly
   - Keep WebView2 Runtime current

2. **Regular maintenance**:
   - Clear WebView2 cache monthly
   - Check log files for warnings

3. **Avoid**:
   - Running multiple instances
   - Forcing termination (use proper exit)
   - Modifying application data manually

---

## macOS Issues (Future Support)

WebView2 is Windows-only. macOS version will use WKWebView.

## Linux Issues (Future Support)

Linux version will use WebKitGTK.

---

## Uninstallation

### What Gets Removed

When you uninstall BEAR LLM AI, you'll be prompted with two options:

**Option 1: Remove All Data (Yes)**
- ✓ Application files
- ✓ Settings and configuration
- ✓ Conversation history and database
- ✓ All log files
- ✓ WebView2 cache
- ✓ Complete removal from `%LOCALAPPDATA%\BEAR LLM AI`

**Option 2: Keep User Data (No)**
- ✓ Application files removed
- ✓ Temporary log files removed (preinit.log, crash.log, etc.)
- ✓ WebView2 cache removed
- ✗ Settings preserved
- ✗ Conversation history preserved
- ✗ Database preserved for reinstallation

### What Is Preserved

The following are **never** removed during uninstall (shared system components):
- Visual C++ Runtime (may be used by other applications)
- WebView2 Runtime (system-wide component)

### Manual Cleanup

If you chose to keep user data and later want to remove it manually:

1. Press `Win + R`
2. Type: `%LOCALAPPDATA%\BEAR LLM AI`
3. Press Enter
4. Delete the entire folder

### Clean Reinstallation

For a completely fresh installation:

1. Uninstall BEAR LLM AI (choose "Yes" to remove all data)
2. Verify removal: Check `%LOCALAPPDATA%` - "BEAR LLM AI" folder should be gone
3. Reinstall the application

---

**Last Updated**: 2025-10-23
**Version**: 0.0.17
