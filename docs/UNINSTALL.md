# BEAR LLM AI - Uninstallation Guide

## Standard Uninstallation

### Windows 10/11

**Method 1: Settings App**
1. Press `Win + I` to open Settings
2. Go to **Apps** → **Installed apps**
3. Search for "BEAR LLM AI"
4. Click the three dots (**⋯**) → **Uninstall**
5. Confirm when prompted

**Method 2: Control Panel**
1. Press `Win + R`
2. Type: `appwiz.cpl`
3. Press Enter
4. Find "BEAR LLM AI" in the list
5. Click **Uninstall**
6. Follow the prompts

**Method 3: Start Menu**
1. Open Start Menu
2. Find "BEAR LLM AI"
3. Right-click → **Uninstall**
4. Follow the prompts

## Data Removal Options

During uninstallation, you'll see this prompt:

```
Do you want to remove all application data including:

  • Settings and configuration
  • Conversation history
  • Log files
  • Cache files

Location: C:\Users\[YourName]\AppData\Local\BEAR LLM AI

Select 'No' to keep your data for future installations.
```

### Choose "Yes" (Remove All Data) if:
- ✓ You're permanently removing the application
- ✓ You want a completely clean system
- ✓ You're troubleshooting and need a fresh start
- ✓ You don't need your conversation history

**Result**: Everything is removed, including:
- All settings and preferences
- Conversation history and database
- All log files
- WebView2 cache
- Complete `%LOCALAPPDATA%\BEAR LLM AI` folder

### Choose "No" (Keep User Data) if:
- ✓ You plan to reinstall later
- ✓ You want to preserve conversation history
- ✓ You want to keep your settings
- ✓ You're upgrading to a newer version

**Result**: Only application files removed, preserved:
- Settings and configuration
- Conversation history and database
- Custom prompts and models

**Removed** (temporary files):
- Log files (preinit.log, crash.log, etc.)
- WebView2 cache (regenerated on next install)

## What Is Never Removed

These system-wide components remain installed:

### Visual C++ Runtime
- **Why preserved**: Other applications may depend on it
- **Location**: System-wide installation
- **To remove manually**: Uninstall from Windows Settings → Apps

### WebView2 Runtime
- **Why preserved**: System-wide component used by many apps
- **Location**: System-wide installation
- **To remove manually**: Not recommended (used by Microsoft Edge and other apps)

## Complete Removal (Advanced)

For absolute complete removal of all components:

### Step 1: Uninstall BEAR LLM AI
1. Run standard uninstall (see above)
2. Choose **"Yes"** to remove all data

### Step 2: Verify Application Data Removal
1. Press `Win + R`
2. Type: `%LOCALAPPDATA%`
3. Press Enter
4. Check that "BEAR LLM AI" folder is gone
5. If it exists, delete it manually

### Step 3: Remove Visual C++ Runtime (Optional)
⚠️ **Warning**: Only do this if no other applications need it!

1. Press `Win + I` → Apps → Installed apps
2. Search: "Microsoft Visual C++ 2015-2022 Redistributable"
3. Uninstall both x64 and x86 versions
4. Restart computer

### Step 4: Remove WebView2 Runtime (Not Recommended)
⚠️ **Warning**: Many applications depend on WebView2!

WebView2 is a system component. Removing it may break:
- Microsoft Edge
- Microsoft Teams
- Other modern Windows applications

**If you must remove it**:
1. Download the WebView2 removal tool from Microsoft
2. Follow Microsoft's official instructions
3. Restart computer

## Troubleshooting Uninstallation

### Issue: Uninstaller says "File in use"

**Solution**:
1. Close all instances of BEAR LLM AI
2. Open Task Manager (`Ctrl + Shift + Esc`)
3. End any "BEAR LLM AI" processes
4. End any "WebView2" processes
5. Try uninstalling again

### Issue: Application data not removed

**Solution**:
1. Manually delete: `%LOCALAPPDATA%\BEAR LLM AI`
2. If access denied, take ownership:
   - Right-click folder → Properties
   - Security tab → Advanced
   - Change owner to your account
   - Apply and retry deletion

### Issue: "Cannot find uninstaller"

**Solution**:
1. Reinstall BEAR LLM AI
2. Run uninstaller immediately after
3. Or manually delete:
   - Program Files location
   - `%LOCALAPPDATA%\BEAR LLM AI`
   - Start Menu shortcuts

### Issue: Uninstaller freezes

**Solution**:
1. Open Task Manager
2. End "uninstall.exe" or "nsis" process
3. Manually delete:
   ```
   C:\Users\[YourName]\AppData\Local\Programs\BEAR LLM AI
   C:\Users\[YourName]\AppData\Local\BEAR LLM AI
   ```
4. Clean registry (advanced):
   - Press `Win + R` → `regedit`
   - Navigate to: `HKEY_CURRENT_USER\Software\BEAR LLM AI`
   - Delete the key
   - Navigate to: `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall`
   - Find and delete BEAR LLM AI entry

## Reinstallation After Uninstall

### Fresh Installation
1. Uninstall (choose "Yes" to remove all data)
2. Download latest installer
3. Run installer
4. First launch will create fresh settings

### Upgrade Installation
1. Uninstall (choose "No" to keep data)
2. Download newer version
3. Run installer
4. Your settings and data are preserved

## Data Backup Before Uninstall

To backup your data before uninstalling:

### Backup Conversation History
1. Press `Win + R`
2. Type: `%LOCALAPPDATA%\BEAR LLM AI`
3. Press Enter
4. Copy the entire folder to a safe location
5. After reinstall, copy back if needed

### Export Settings (Manual)
1. Open BEAR LLM AI
2. Export your settings (if available in UI)
3. Save to a safe location
4. Import after reinstall

## Silent Uninstall (Advanced)

For IT administrators or automated uninstallation:

```batch
REM Silent uninstall with data removal
"C:\Users\[User]\AppData\Local\Programs\BEAR LLM AI\uninstall.exe" /S

REM Silent uninstall keeping data (not supported - use standard method)
```

⚠️ **Note**: Silent uninstall automatically removes all data. To preserve data, use the standard uninstall method.

## Post-Uninstall Verification

After uninstalling, verify complete removal:

### Check 1: Application Files
```batch
dir "C:\Users\%USERNAME%\AppData\Local\Programs\BEAR LLM AI"
```
Should return: "File Not Found"

### Check 2: Application Data
```batch
dir "C:\Users\%USERNAME%\AppData\Local\BEAR LLM AI"
```
Should return: "File Not Found" (if you chose to remove all data)

### Check 3: Start Menu
- Search for "BEAR LLM AI"
- Should not appear

### Check 4: Installed Apps
- Open Settings → Apps
- Search "BEAR LLM AI"
- Should not appear

## Getting Help

If you experience issues during uninstallation:

1. **Check logs**: `%LOCALAPPDATA%\BEAR LLM AI\*.log`
2. **Try safe mode**: Boot Windows in Safe Mode and uninstall
3. **Contact support**: Provide uninstaller logs and error messages
4. **Manual removal**: Use the complete removal guide above

---

**Last Updated**: 2025-10-23
**Version**: 0.0.17
