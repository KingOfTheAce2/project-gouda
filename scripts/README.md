# This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).

# Windows WebView2 Fix Scripts

This directory contains PowerShell scripts to fix WebView2 permission issues on Windows multi-user systems.

## Scripts Overview

### 1. windows-fix-webview2.ps1

**Purpose**: Diagnose and automatically fix WebView2 permission issues without data loss.

**Usage**:
```powershell
# Basic usage (with prompts)
.\windows-fix-webview2.ps1

# Force automatic fix without prompts
.\windows-fix-webview2.ps1 -Force

# Verbose output for debugging
.\windows-fix-webview2.ps1 -Force -Verbose
```

**What it does**:
- ✅ Checks current user and permissions
- ✅ Verifies WebView2 folder ownership
- ✅ Tests write permissions
- ✅ Automatically fixes permission issues (with -Force)
- ✅ Shows WebView2 runtime version
- ✅ Provides detailed diagnostic information

**When to use**:
- App crashes immediately after launch
- "Data folder cannot be created" error
- After switching Windows users
- After running app as Administrator accidentally

### 2. windows-cleanup-webview2.ps1

**Purpose**: Complete cleanup of all BEAR LLM AI data for fresh reinstall.

**Usage**:
```powershell
# With confirmation prompt
.\windows-cleanup-webview2.ps1

# Force cleanup without prompts
.\windows-cleanup-webview2.ps1 -Force
```

**⚠️ WARNING**: This script **DELETES ALL DATA** including:
- Chat histories
- Settings and preferences
- WebView2 cache
- Database files

**What it does**:
- 🗑️ Takes ownership of app data folder
- 🗑️ Removes all BEAR LLM AI data
- 🗑️ Cleans up related WebView2 folders
- 🗑️ Prepares system for clean reinstall

**When to use**:
- Permission errors cannot be fixed automatically
- Preparing for clean reinstall
- Multiple failed fix attempts
- Corrupt installation

## Common Scenarios

### Scenario 1: App flashes open and crashes

```powershell
# Run the fix script
.\windows-fix-webview2.ps1 -Force
```

If this doesn't work:
```powershell
# Complete cleanup and reinstall
.\windows-cleanup-webview2.ps1 -Force
# Then reinstall the app as regular user
```

### Scenario 2: "Run as Administrator" error

**Problem**: Someone ran the app as Administrator, now it won't work for regular users.

**Solution**:
```powershell
# As the regular user who needs to use the app
.\windows-cleanup-webview2.ps1 -Force
# Then reinstall as regular user (NOT as Administrator)
```

### Scenario 3: Multi-user system

**Problem**: User A installed the app, User B can't run it.

**Solution**: User B should:
```powershell
# User B runs this in their own PowerShell window
.\windows-cleanup-webview2.ps1 -Force
# Then User B installs their own copy
```

### Scenario 4: Permission denied errors

```powershell
# First try automatic fix
.\windows-fix-webview2.ps1 -Force -Verbose

# If that fails, use cleanup
.\windows-cleanup-webview2.ps1 -Force
```

## Requirements

- Windows 10 or later
- PowerShell 5.1 or later
- Regular user account (scripts work best as regular user)

## Troubleshooting

### Script won't run - "Execution Policy" error

```powershell
# Allow script execution for current session
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass

# Then run the script
.\windows-fix-webview2.ps1
```

### "Access denied" errors

1. Close all BEAR LLM AI windows
2. Run PowerShell as **regular user** (not Administrator)
3. Run the cleanup script with -Force:
   ```powershell
   .\windows-cleanup-webview2.ps1 -Force
   ```

### Scripts don't fix the issue

If automated fixes don't work:

1. **Manual cleanup**:
   - Close all BEAR LLM AI windows
   - Open File Explorer
   - Navigate to: `%LOCALAPPDATA%`
   - Delete `com.bearllm.ai` folder
   - Reinstall as regular user

2. **Check Windows Event Viewer**:
   ```
   Win + R → eventvwr.msc → Windows Logs → Application
   ```
   Look for BEAR LLM AI or WebView2 errors

3. **Report issue** with:
   - Script output
   - Event Viewer logs
   - Windows version
   - User account type

## Best Practices

1. **Always run as regular user**
   - Never use "Run as Administrator" for these scripts
   - Never use "Run as Administrator" for the app

2. **One user per installation**
   - Each Windows user should install their own copy
   - Use "Install for current user only" during installation

3. **After cleanup, always reinstall**
   - Don't just fix permissions and keep old installation
   - Clean reinstall ensures no lingering issues

4. **Backup data before cleanup**
   - Chat histories are stored in `%LOCALAPPDATA%\com.bearllm.ai\data.db`
   - Copy this file before running cleanup script if you want to preserve data

## Support

For more information, see:
- [Windows Multi-User Troubleshooting Guide](../docs/WINDOWS_MULTI_USER_GUIDE.md)
- [GitHub Issues](https://github.com/yourusername/project-gouda/issues)

## Version

These scripts are designed for BEAR LLM AI version **0.0.3+** with automatic WebView2 permission handling.

---

**This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).**
