# User-Level Installation Guide

## Overview

BEAR LLM AI now supports **two installation modes**:
1. **Current User Only** (Recommended) - No admin privileges required
2. **All Users** - Requires administrator privileges

## Installation Modes Comparison

| Feature | Current User Only | All Users |
|---------|------------------|-----------|
| **Admin Required** | ❌ No | ✅ Yes |
| **Install Location** | `%LOCALAPPDATA%\Programs\BEAR LLM AI` | `C:\Program Files\BEAR LLM AI` |
| **Available To** | Only you | All users on the computer |
| **Recommended For** | Personal use, standard users | Shared computers, IT deployments |
| **Updates** | User manages | Admin manages |

## Installation Process

### Option 1: Current User Installation (No Admin Required) ✅

**Step 1**: Download the installer (`BEAR-LLM-AI-v0.0.10.exe`)

**Step 2**: Double-click the installer (no need to "Run as administrator")

**Step 3**: On the installation type page, select:
```
⦿ Install for current user only (recommended, no admin required)
○ Install for all users (requires administrator privileges)
```

**Step 4**: Click "Next" and complete installation

**Result**:
- Installed to: `%LOCALAPPDATA%\Programs\BEAR LLM AI`
- Desktop shortcut created (optional)
- Start Menu entry in your user profile
- No UAC prompt required

### Option 2: All Users Installation (Requires Admin)

**Step 1**: Download the installer

**Step 2**: Right-click installer → "Run as administrator"

**Step 3**: On the installation type page, select:
```
○ Install for current user only (recommended, no admin required)
⦿ Install for all users (requires administrator privileges)
```

**Step 4**: Click "Next" and complete installation

**Result**:
- Installed to: `C:\Program Files\BEAR LLM AI`
- Desktop shortcuts for all users
- Start Menu entry in common programs
- Requires UAC approval

## Smart Installation Behavior

### If you select "All Users" without admin rights:

The installer will show a helpful dialog:
```
Installing for all users requires administrator privileges.

Do you want to install for current user only instead?

[Yes] [No]
```

- **Click "Yes"**: Switches to current user installation automatically
- **Click "No"**: Shows error and exits (you can restart as admin)

### Default Mode

By default, the installer selects **"Current user only"** to provide the smoothest installation experience without admin requirements.

## Installation Locations

### Current User Mode:
```
Installation: %LOCALAPPDATA%\Programs\BEAR LLM AI\
              (typically: C:\Users\YourName\AppData\Local\Programs\BEAR LLM AI\)

App Data:     %APPDATA%\com.bearllm.ai\
              (typically: C:\Users\YourName\AppData\Roaming\com.bearllm.ai\)

Logs:         %APPDATA%\com.bearllm.ai\logs\
```

### All Users Mode:
```
Installation: C:\Program Files\BEAR LLM AI\

App Data:     %APPDATA%\com.bearllm.ai\
              (per user, even in all-users mode)

Logs:         %APPDATA%\com.bearllm.ai\logs\
              (per user)
```

## WebView2 Installation

WebView2 Runtime installation behavior is **the same for both modes**:

- Downloaded automatically during installation
- Installs to system location (requires internet)
- Silent installation (no separate dialog)
- Shared by all applications on the system

## Uninstallation

### Current User Installation:
1. Settings → Apps → Apps & features
2. Find "BEAR LLM AI"
3. Click "Uninstall"
4. No admin required

### All Users Installation:
1. Settings → Apps → Apps & features
2. Find "BEAR LLM AI"
3. Click "Uninstall"
4. Requires admin approval (UAC prompt)

## Troubleshooting

### Issue: "I want to install without admin but it's asking for admin"

**Solution**: Make sure you select "Install for current user only" on the installation type page.

### Issue: "I accidentally chose 'All Users' and don't have admin rights"

**Solution**:
1. Click "Yes" when prompted to switch to current user installation
2. OR restart installer and choose "Current user only"

### Issue: "Where is the application installed?"

**Check**:
- Current user: `%LOCALAPPDATA%\Programs\BEAR LLM AI\`
- All users: `C:\Program Files\BEAR LLM AI\`

### Issue: "Can I upgrade from current user to all users?"

**Solution**:
1. Uninstall current user version
2. Run installer as administrator
3. Choose "All users" option

### Issue: "Can I have both installations?"

**Not recommended**: Only install one version to avoid conflicts. The installer will detect existing installations and prompt for upgrade/reinstall.

## Benefits of Current User Installation

### ✅ Advantages:
- **No admin required** - Install on work computers without IT
- **Faster installation** - No UAC prompts or delays
- **User control** - Manage your own installation
- **Portable-like** - Isolated to your user profile
- **Quick updates** - Update without admin approval

### ⚠️ Limitations:
- Only visible to your user account
- Other users can't run the application
- Not suitable for IT mass deployment

## Benefits of All Users Installation

### ✅ Advantages:
- Available to all users on the computer
- Standard Program Files location
- Easier for IT management
- System-wide availability

### ⚠️ Limitations:
- Requires administrator privileges
- Updates need admin approval
- UAC prompts during installation

## Recommendation

**For most users**: Choose **"Current user only"**
- Simple installation
- No admin headaches
- Full functionality
- Easy updates

**For IT/Enterprise**: Choose **"All users"**
- Centralized deployment
- Standard location
- Multi-user support

## Technical Details

### NSIS Configuration:
```json
{
  "nsis": {
    "installMode": "both",
    "languages": ["English"],
    "installerIcon": "icons/icon.ico",
    "template": "installer.nsi"
  }
}
```

### Installer Behavior:
```nsi
RequestExecutionLevel user  ; Start with user privileges

Function .onInit
    StrCpy $InstallMode "CurrentUser"  ; Default to current user
    SetShellVarContext current
FunctionEnd
```

### Installation Path Logic:
```
If CurrentUser:
    $INSTDIR = $LOCALAPPDATA\Programs\BEAR LLM AI

If AllUsers:
    $INSTDIR = $PROGRAMFILES\BEAR LLM AI
```

## Security Considerations

Both installation modes are equally secure:

✅ **Current User Installation**:
- Files protected by user account permissions
- Isolated from other users
- Can't modify system files

✅ **All Users Installation**:
- Files in protected Program Files
- Requires admin to modify
- Shared across users

## Frequently Asked Questions

**Q: Which mode should I choose?**
A: Current user only (no admin required) for personal use.

**Q: Will current user installation work the same?**
A: Yes, full functionality in both modes.

**Q: Can I switch modes later?**
A: Yes, uninstall and reinstall in the other mode.

**Q: Does WebView2 require admin?**
A: WebView2 bootstrapper handles this automatically.

**Q: Where are my settings stored?**
A: User profile (`%APPDATA%\com.bearllm.ai\`) regardless of installation mode.

**Q: Can I install without internet?**
A: You'll need internet for WebView2 download, or pre-install WebView2 Runtime.

**Q: What if I work on a locked-down computer?**
A: Current user installation should work if you can install to your user profile.

---

**Recommended Installation**: Current User Only (No Admin Required) ✅
