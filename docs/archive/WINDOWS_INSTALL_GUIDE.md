# Windows Installation Guide

> This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).

## ✅ Fixed Issues (v0.0.1+)

- **Identifier typo:** No more confusing "bearllmail" folder path
- **WebView2:** Automatically installs Microsoft Edge WebView2 runtime
- **Permissions:** Works properly without Administrator rights

## Simple Installation

### Step 1: Download

Download the installer from releases:
- **NSIS Installer (Recommended):** `BEAR LLM AI_0.0.1_x64-setup.exe`
- **MSI Installer:** `BEAR LLM AI_0.0.1_x64_en-US.msi`

### Step 2: Install

1. **Run the installer** (double-click)
2. **Don't run as Administrator** - run as normal user
3. **Wait for installation** - WebView2 will auto-install if needed
4. **Launch the application**

That's it! 🎉

## What Gets Installed

### Application Files

```
C:\Users\<YourName>\AppData\Local\com.bearllm.ai\
├── BEAR LLM AI.exe      ← Main application
├── bear-llm-ai.db       ← Your data (conversations, models, settings)
└── EBWebView\           ← Browser engine data
```

### WebView2 Runtime (Auto-installed)

```
C:\Program Files (x86)\Microsoft\EdgeWebView\Application\
└── <version>\
    └── msedgewebview2.exe
```

**Size:** ~150MB
**Internet Required:** Only for first-time installation

## Common Questions

### What is WebView2?

It's the browser engine that displays the application's interface. Think of it like having a mini Microsoft Edge inside the app.

### Why "com.bearllm.ai" folder?

This is the standard app identifier format. It ensures:
- ✅ Unique folder name
- ✅ No conflicts with other apps
- ✅ Proper organization

### Do I need Microsoft Edge?

No! WebView2 is separate from Microsoft Edge browser. The app includes its own runtime.

### Can I install without internet?

For the first installation, internet is required to download WebView2 (~150MB). After that, the app works fully offline.

## Troubleshooting

### Installation Fails

**Solution:** Make sure you have:
- Windows 10 version 1803 or later
- At least 500MB free disk space
- Internet connection (for first install)

### "Cannot create data folder" Error

**Solution:**
1. **Uninstall** the application
2. **Delete** old folders:
   ```
   C:\Users\<YourName>\AppData\Local\com.bearllmai\
   C:\Users\<YourName>\AppData\Local\com.bearllmail\
   ```
3. **Reinstall** from the new version

### Application Won't Launch

**Manual WebView2 Installation:**

1. Download: [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/#download-section)
2. Install: Run `MicrosoftEdgeWebview2Setup.exe`
3. Restart: Launch BEAR LLM AI again

### Permission Denied

**Don't run as Administrator!**

The application is designed to run as a normal user. Running as Administrator can cause permission conflicts.

## Uninstallation

### Complete Removal

**1. Uninstall Application:**
- Windows Settings → Apps → BEAR LLM AI → Uninstall

**2. Remove Data (Optional):**
```
C:\Users\<YourName>\AppData\Local\com.bearllm.ai\
```

**3. Keep WebView2:**
WebView2 is used by many applications. We recommend keeping it installed.

## System Requirements

| Requirement | Specification |
|------------|---------------|
| **OS** | Windows 10 (1803+) or Windows 11 |
| **Processor** | x64, x86, or ARM64 |
| **RAM** | 4GB minimum, 8GB recommended |
| **Disk Space** | 500MB (including WebView2) |
| **Internet** | Required for first install |

## Updates

The application includes an auto-updater:

1. **Check for updates** when you launch
2. **Download** update in background
3. **Install** on next restart

No need to manually download new versions!

## Support

If you encounter issues:

1. Check this guide first
2. Review `docs/WEBVIEW2_FIX.md` for technical details
3. Report issues with:
   - Windows version
   - Error messages
   - Screenshots

## Dutch (Nederlands)

### Veel Voorkomende Fout

**Foutmelding:**
> "De gegevensmap kan niet worden gemaakt"

**Oplossing:**
1. Oude versie deïnstalleren
2. Verwijder oude mappen:
   - `C:\Users\<UwNaam>\AppData\Local\com.bearllmai\`
   - `C:\Users\<UwNaam>\AppData\Local\com.bearllmail\`
3. Nieuwe versie installeren

**Let op:** Gebruik geen Administrator rechten!

---

**Version:** 0.0.1+
**Last Updated:** 2025-10-11
