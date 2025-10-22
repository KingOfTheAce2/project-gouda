# Windows Multi-User Troubleshooting Guide

[English](#english) | [Nederlands](#nederlands)

---

## English

### Problem: WebView2 Data Folder Access Error

**Error Message:**
```
The data folder cannot be created.
Microsoft Edge cannot read from and write to its data folder:
C:\Users\[username]\AppData\Local\com.bearllm.ai\EBWebView
```

### Why This Happens

This error occurs when:
1. The application was installed by **User A** (e.g., `jvbbe`)
2. You're trying to run it as **User B** (e.g., `jbvbe`) or as Administrator
3. Windows creates user-specific data folders in `C:\Users\[username]\AppData\Local\`
4. Each Windows user has their own isolated AppData folder with restricted permissions

**Important:** Running as Administrator does NOT fix this - it makes it worse! The app needs to run as a regular user.

### What's Fixed in Version 0.0.3+

Starting from version **0.0.3**, BEAR LLM AI includes automatic WebView2 permission handling:

✅ **Automatic permission verification** on startup
✅ **Automatic folder recreation** if permissions are incorrect
✅ **Detailed logging** to help diagnose issues
✅ **Per-user WebView2 data isolation**

**If you're upgrading from 0.0.2 or earlier**, use the cleanup scripts below before reinstalling.

---

## Quick Fix: Automated Scripts (Recommended for v0.0.3+)

For version **0.0.3 and later**, we provide PowerShell scripts to automatically fix permission issues.

### Option A: Permission Fix Script

Run this if the app is crashing or showing WebView2 errors:

```powershell
# Download and run the fix script
irm https://raw.githubusercontent.com/yourusername/project-gouda/main/scripts/windows-fix-webview2.ps1 | iex

# Or if you have the repository cloned:
cd project-gouda
.\scripts\windows-fix-webview2.ps1 -Force -Verbose
```

This script will:
- ✅ Check WebView2 folder permissions
- ✅ Automatically fix ownership issues
- ✅ Verify write permissions
- ✅ Display WebView2 runtime status

### Option B: Complete Cleanup Script

Use this for a fresh start (deletes all data):

```powershell
# WARNING: This deletes all your chat history and settings!
cd project-gouda
.\scripts\windows-cleanup-webview2.ps1 -Force
```

After running the cleanup, reinstall the application following "Solution 1" below.

---

## Solution 1: Clean Reinstall (Traditional Method)

Use this if automated scripts don't work or you prefer manual steps.

### Step 1: Uninstall Existing Installation

1. **As the user who wants to use the app** (not as Administrator):
   - Press `Win + R`
   - Type: `appwiz.cpl`
   - Press Enter
   - Find "BEAR LLM AI" in the list
   - Click "Uninstall"

2. **Clean up leftover folders** (PowerShell as regular user):
   ```powershell
   # Remove all user-specific app data
   Remove-Item -Recurse -Force "$env:LOCALAPPDATA\com.bearllm.ai" -ErrorAction SilentlyContinue

   # Remove WebView2 data folders for this app
   Get-ChildItem "$env:LOCALAPPDATA" -Directory | Where-Object { $_.Name -match "bearllm" } | Remove-Item -Recurse -Force
   ```

### Step 2: Reinstall for Current User

1. **Download the installer** for version 0.0.2
2. **Run the installer as REGULAR USER** (not as Administrator)
3. **DO NOT right-click → "Run as Administrator"**
4. During installation, choose **"Install for current user only"**
5. Launch the application normally (again, not as Administrator)

### Step 3: Verify Installation

```powershell
# Check that the app data folder exists and has correct permissions
Test-Path "$env:LOCALAPPDATA\com.bearllm.ai"
# Should return: True

# Check folder owner
Get-Acl "$env:LOCALAPPDATA\com.bearllm.ai" | Select-Object Owner
# Should show: YourComputerName\YourUsername
```

---

## Solution 2: Fix Permissions (Advanced)

If you can't reinstall, try fixing permissions for existing installation.

### Check Current Ownership

```powershell
# Check which user owns the WebView2 folder
Get-Acl "$env:LOCALAPPDATA\com.bearllm.ai" | Format-List
```

### Grant Current User Full Control

```powershell
# Grant yourself full control over the folder
$path = "$env:LOCALAPPDATA\com.bearllm.ai"
$acl = Get-Acl $path
$permission = "$env:USERNAME","FullControl","ContainerInherit,ObjectInherit","None","Allow"
$accessRule = New-Object System.Security.AccessControl.FileSystemAccessRule $permission
$acl.SetAccessRule($accessRule)
Set-Acl $path $acl

# Verify permissions
Get-Acl $path | Format-List
```

### Take Ownership (If needed)

```powershell
# Take ownership of the folder
takeown /F "$env:LOCALAPPDATA\com.bearllm.ai" /R /D Y

# Grant yourself full control
icacls "$env:LOCALAPPDATA\com.bearllm.ai" /grant "%USERNAME%:(OI)(CI)F" /T
```

---

## Prevention Tips

1. **Always install as regular user** - Never use "Run as Administrator" for installation
2. **One user per installation** - Each Windows user should install their own copy
3. **Shared computers** - Consider portable installation instead of per-user installation
4. **Administrator accounts** - If you need admin privileges, install while logged in as that admin user (not via right-click "Run as Administrator")

---

## Common Questions

**Q: Can multiple users share one installation?**
A: No, not with the current configuration. Each user needs their own installation or you need a system-wide installation.

**Q: Why can't I run it as Administrator?**
A: When you "Run as Administrator", Windows uses the Administrator's profile (`C:\Windows\System32\config\systemprofile\AppData\Local\`), not your user profile. This causes permission conflicts.

**Q: I installed it for User A but User B needs to use it. What do I do?**
A: User B should install their own copy following "Solution 1: Clean Reinstall" above.

**Q: Where are my chat histories stored?**
A: In `C:\Users\[YourUsername]\AppData\Local\com.bearllm.ai\data.db` - this is user-specific.

---

## Advanced: Technical Details (v0.0.3+)

### How the Fix Works

Starting in version **0.0.3**, BEAR LLM AI implements automatic WebView2 permission management:

1. **On Startup**: The app checks if `%LOCALAPPDATA%\com.bearllm.ai\EBWebView` exists
2. **Permission Test**: Attempts to write a test file to verify permissions
3. **Auto-Fix**: If permissions fail:
   - Removes the corrupted folder
   - Recreates it with proper permissions for current user
   - Sets `WEBVIEW2_USER_DATA_FOLDER` environment variable
4. **Logging**: All operations are logged for diagnostics

### Check Logs

Application logs are stored in:
```
%LOCALAPPDATA%\com.bearllm.ai\logs\
```

Look for entries like:
```
INFO: Setting up WebView2 user data folder at: C:\Users\...\com.bearllm.ai\EBWebView
INFO: WebView2 folder permissions verified
INFO: Set WEBVIEW2_USER_DATA_FOLDER environment variable
```

Or error messages:
```
ERROR: Failed to read WebView2 folder metadata
WARN: WebView2 folder exists but is not writable
```

### Manual Permission Fix

If automatic fixes don't work, use PowerShell as regular user:

```powershell
# Get the folder path
$folder = "$env:LOCALAPPDATA\com.bearllm.ai\EBWebView"

# Take ownership
takeown /F "$folder" /R /D Y

# Grant yourself full control
icacls "$folder" /grant "${env:USERNAME}:(OI)(CI)F" /T

# Verify
Get-Acl "$folder" | Format-List
```

## Still Having Issues?

If you're still experiencing problems after trying the automatic fixes:

1. **Check Windows Event Viewer:**
   ```
   Win + R → eventvwr.msc → Windows Logs → Application
   ```
   Look for errors related to "BEAR LLM AI" or "WebView2"

2. **Verify WebView2 installation:**
   ```powershell
   Get-ItemProperty -Path "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
   ```
   Should show WebView2 runtime version

3. **Check application logs:**
   ```powershell
   Get-Content "$env:LOCALAPPDATA\com.bearllm.ai\logs\*.log" -Tail 50
   ```

4. **Create a detailed bug report** with:
   - Windows username (current user)
   - BEAR LLM AI version (check in app or Cargo.toml)
   - Installation method (installer type, as admin or not)
   - Exact error message
   - Application logs from `%LOCALAPPDATA%\com.bearllm.ai\logs\`
   - Event Viewer logs

---

# Nederlands

### Probleem: WebView2 Gegevensmapfout

**Foutmelding:**
```
De gegevensmap kan niet worden gemaakt.
Microsoft Edge kan niet lezen van en schrijven naar de bijbehorende gegevensmap:
C:\Users\[gebruikersnaam]\AppData\Local\com.bearllm.ai\EBWebView
```

### Waarom Dit Gebeurt

Deze fout treedt op wanneer:
1. De applicatie is geïnstalleerd door **Gebruiker A** (bijv. `jvbbe`)
2. U probeert het uit te voeren als **Gebruiker B** (bijv. `jbvbe`) of als Administrator
3. Windows maakt gebruikersspecifieke gegevensmappen in `C:\Users\[gebruikersnaam]\AppData\Local\`
4. Elke Windows-gebruiker heeft zijn eigen geïsoleerde AppData-map met beperkte rechten

**Belangrijk:** Uitvoeren als Administrator lost dit NIET op - het maakt het erger! De app moet worden uitgevoerd als normale gebruiker.

---

## Oplossing 1: Schone Herinstallatie (Aanbevolen)

Dit is de meest betrouwbare oplossing.

### Stap 1: Bestaande Installatie Verwijderen

1. **Als de gebruiker die de app wil gebruiken** (niet als Administrator):
   - Druk op `Win + R`
   - Typ: `appwiz.cpl`
   - Druk op Enter
   - Zoek "BEAR LLM AI" in de lijst
   - Klik op "Verwijderen"

2. **Ruim overgebleven mappen op** (PowerShell als normale gebruiker):
   ```powershell
   # Verwijder alle gebruikersspecifieke app-gegevens
   Remove-Item -Recurse -Force "$env:LOCALAPPDATA\com.bearllm.ai" -ErrorAction SilentlyContinue

   # Verwijder WebView2-gegevensmappen voor deze app
   Get-ChildItem "$env:LOCALAPPDATA" -Directory | Where-Object { $_.Name -match "bearllm" } | Remove-Item -Recurse -Force
   ```

### Stap 2: Opnieuw Installeren voor Huidige Gebruiker

1. **Download het installatieprogramma** voor versie 0.0.2
2. **Voer het installatieprogramma uit als NORMALE GEBRUIKER** (niet als Administrator)
3. **GEEN rechtermuisklik → "Als administrator uitvoeren"**
4. Kies tijdens installatie **"Alleen voor huidige gebruiker installeren"**
5. Start de applicatie normaal (opnieuw, niet als Administrator)

### Stap 3: Installatie Controleren

```powershell
# Controleer of de app-gegevensmap bestaat en de juiste rechten heeft
Test-Path "$env:LOCALAPPDATA\com.bearllm.ai"
# Moet teruggeven: True

# Controleer map-eigenaar
Get-Acl "$env:LOCALAPPDATA\com.bearllm.ai" | Select-Object Owner
# Moet tonen: UwComputerNaam\UwGebruikersnaam
```

---

## Oplossing 2: Rechten Herstellen (Geavanceerd)

Als u niet opnieuw kunt installeren, probeer dan de rechten voor de bestaande installatie te herstellen.

### Controleer Huidige Eigendom

```powershell
# Controleer welke gebruiker eigenaar is van de WebView2-map
Get-Acl "$env:LOCALAPPDATA\com.bearllm.ai" | Format-List
```

### Geef Huidige Gebruiker Volledige Controle

```powershell
# Geef uzelf volledige controle over de map
$path = "$env:LOCALAPPDATA\com.bearllm.ai"
$acl = Get-Acl $path
$permission = "$env:USERNAME","FullControl","ContainerInherit,ObjectInherit","None","Allow"
$accessRule = New-Object System.Security.AccessControl.FileSystemAccessRule $permission
$acl.SetAccessRule($accessRule)
Set-Acl $path $acl

# Controleer rechten
Get-Acl $path | Format-List
```

### Neem Eigendom (Indien nodig)

```powershell
# Neem eigendom van de map
takeown /F "$env:LOCALAPPDATA\com.bearllm.ai" /R /D Y

# Geef uzelf volledige controle
icacls "$env:LOCALAPPDATA\com.bearllm.ai" /grant "%USERNAME%:(OI)(CI)F" /T
```

---

## Preventietips

1. **Installeer altijd als normale gebruiker** - Gebruik nooit "Als administrator uitvoeren" voor installatie
2. **Eén gebruiker per installatie** - Elke Windows-gebruiker moet zijn eigen kopie installeren
3. **Gedeelde computers** - Overweeg draagbare installatie in plaats van per-gebruiker installatie
4. **Administratoraccounts** - Als u adminrechten nodig heeft, installeer dan terwijl u bent ingelogd als die admin-gebruiker (niet via rechtermuisklik "Als administrator uitvoeren")

---

## Veelgestelde Vragen

**V: Kunnen meerdere gebruikers één installatie delen?**
A: Nee, niet met de huidige configuratie. Elke gebruiker heeft zijn eigen installatie nodig of u heeft een systeembrede installatie nodig.

**V: Waarom kan ik het niet uitvoeren als Administrator?**
A: Wanneer u "Als administrator uitvoeren" gebruikt, gebruikt Windows het profiel van de Administrator (`C:\Windows\System32\config\systemprofile\AppData\Local\`), niet uw gebruikersprofiel. Dit veroorzaakt rechtenconflicten.

**V: Ik heb het geïnstalleerd voor Gebruiker A maar Gebruiker B moet het gebruiken. Wat moet ik doen?**
A: Gebruiker B moet zijn eigen kopie installeren volgens "Oplossing 1: Schone Herinstallatie" hierboven.

**V: Waar worden mijn chatgeschiedenissen opgeslagen?**
A: In `C:\Users\[UwGebruikersnaam]\AppData\Local\com.bearllm.ai\data.db` - dit is gebruikersspecifiek.

---

## Nog Steeds Problemen?

Als u nog steeds problemen ondervindt:

1. **Controleer Windows Logboeken:**
   ```
   Win + R → eventvwr.msc → Windows-logboeken → Toepassing
   ```
   Zoek naar fouten gerelateerd aan "BEAR LLM AI" of "WebView2"

2. **Verifieer WebView2-installatie:**
   ```powershell
   Get-ItemProperty -Path "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
   ```
   Moet WebView2 runtime-versie tonen

3. **Maak een gedetailleerd bugrapport** met:
   - Windows-gebruikersnaam (huidige gebruiker)
   - Installatiemethode (installatiebestandstype, als admin of niet)
   - Exacte foutmelding
   - Logboeken van Logboeken

---

## This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
