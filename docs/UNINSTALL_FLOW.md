# BEAR LLM AI - Uninstall Flow Diagram

## Visual Overview

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│         User Initiates Uninstall                            │
│         (Settings → Apps → Uninstall)                       │
│                                                             │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│         NSIS Uninstaller Starts                             │
│         (NSIS_HOOK_UNINSTALL macro executes)                │
│                                                             │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│   Step 1: Check for Application Data                       │
│   Location: %LOCALAPPDATA%\BEAR LLM AI                     │
│                                                             │
│   ┌─────────────────────────────────────────┐              │
│   │ Folder exists?                          │              │
│   └─────────┬────────────────┬──────────────┘              │
│             │                │                             │
│            NO              YES                             │
│             │                │                             │
│             ▼                ▼                             │
│   ┌─────────────┐   ┌────────────────────┐                │
│   │ Skip to End │   │ Proceed to Step 2  │                │
│   └─────────────┘   └────────────────────┘                │
│                                                             │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│   Step 2: User Prompt - Data Removal Decision              │
│                                                             │
│   ┌─────────────────────────────────────────────────────┐  │
│   │                                                     │  │
│   │  Do you want to remove all application data?       │  │
│   │                                                     │  │
│   │  • Settings and configuration                      │  │
│   │  • Conversation history                            │  │
│   │  • Log files                                       │  │
│   │  • Cache files                                     │  │
│   │                                                     │  │
│   │  Location: C:\Users\...\BEAR LLM AI                │  │
│   │                                                     │  │
│   │  Select 'No' to keep your data.                    │  │
│   │                                                     │  │
│   │         ┌─────┐        ┌─────┐                     │  │
│   │         │ YES │        │ NO  │                     │  │
│   │         └──┬──┘        └──┬──┘                     │  │
│   └────────────┼──────────────┼────────────────────────┘  │
│                │              │                           │
└────────────────┼──────────────┼───────────────────────────┘
                 │              │
       ┌─────────┴──────┐  ┌───┴──────────┐
       │                │  │              │
       ▼                │  ▼              │
┌──────────────┐        │  ┌──────────────┴───────────┐
│              │        │  │                          │
│ REMOVE ALL   │        │  │   REMOVE PARTIAL         │
│   DATA       │        │  │      DATA                │
│              │        │  │                          │
│  Actions:    │        │  │   Actions:               │
│              │        │  │                          │
│  1. Delete   │        │  │   1. Delete logs only:   │
│     entire   │        │  │      • preinit.log       │
│     folder:  │        │  │      • fatal_error.log   │
│              │        │  │      • crash.log         │
│     %LOCAL   │        │  │      • diagnostics.log   │
│     APPDATA% │        │  │                          │
│     \BEAR    │        │  │   2. Delete cache:       │
│     LLM AI\  │        │  │      • WebView2\         │
│              │        │  │                          │
│  2. Verify   │        │  │   3. Preserve:           │
│     removal  │        │  │      • Database          │
│              │        │  │      • Settings          │
│  3. Log:     │        │  │      • User data         │
│     "All     │        │  │                          │
│     data     │        │  │   4. Log:                │
│     removed" │        │  │      "User data          │
│              │        │  │       preserved"         │
│              │        │  │                          │
└──────┬───────┘        │  └──────┬───────────────────┘
       │                │         │
       └────────────────┼─────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│   Step 3: Remove Application Files                         │
│                                                             │
│   Locations:                                                │
│   • C:\Users\[User]\AppData\Local\Programs\BEAR LLM AI\    │
│   • Start Menu shortcuts                                   │
│   • Registry entries                                       │
│                                                             │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│   Step 4: Preserve System Components                       │
│                                                             │
│   NOT REMOVED (shared system components):                  │
│   • Visual C++ Runtime 2015-2022                           │
│   • WebView2 Runtime                                       │
│                                                             │
│   Reason: Other applications may depend on these           │
│                                                             │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│   Step 5: Completion                                       │
│                                                             │
│   ✓ Uninstallation complete                                │
│   ✓ Application removed from Windows                       │
│   ✓ User choice for data respected                         │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## What Happens in Each Scenario

### Scenario A: User Chooses "YES" (Remove All Data)

```
BEFORE UNINSTALL:
├── C:\Users\Gassen\AppData\Local\Programs\BEAR LLM AI\
│   ├── BEAR LLM AI.exe
│   ├── WebView2Loader.dll
│   └── resources\...
│
└── C:\Users\Gassen\AppData\Local\BEAR LLM AI\
    ├── preinit.log
    ├── fatal_error.log
    ├── crash.log
    ├── diagnostics.log
    ├── db.sqlite (user database)
    ├── settings.json
    └── WebView2\
        ├── Cache\...
        └── ...

AFTER UNINSTALL:
├── C:\Users\Gassen\AppData\Local\Programs\BEAR LLM AI\
│   [DELETED - Folder removed completely]
│
└── C:\Users\Gassen\AppData\Local\BEAR LLM AI\
    [DELETED - Folder removed completely]

✓ Clean uninstall
✓ No data remains
✓ Ready for fresh reinstall
```

### Scenario B: User Chooses "NO" (Keep User Data)

```
BEFORE UNINSTALL:
├── C:\Users\Gassen\AppData\Local\Programs\BEAR LLM AI\
│   ├── BEAR LLM AI.exe
│   ├── WebView2Loader.dll
│   └── resources\...
│
└── C:\Users\Gassen\AppData\Local\BEAR LLM AI\
    ├── preinit.log
    ├── fatal_error.log
    ├── crash.log
    ├── diagnostics.log
    ├── db.sqlite (user database)
    ├── settings.json
    └── WebView2\
        ├── Cache\...
        └── ...

AFTER UNINSTALL:
├── C:\Users\Gassen\AppData\Local\Programs\BEAR LLM AI\
│   [DELETED - Application removed]
│
└── C:\Users\Gassen\AppData\Local\BEAR LLM AI\
    ├── preinit.log [DELETED]
    ├── fatal_error.log [DELETED]
    ├── crash.log [DELETED]
    ├── diagnostics.log [DELETED]
    ├── db.sqlite ✓ PRESERVED
    ├── settings.json ✓ PRESERVED
    └── WebView2\ [DELETED - Cache only]

✓ Application removed
✓ User data preserved
✓ Settings available for reinstall
✓ Conversation history intact
```

## File-by-File Breakdown

| File/Folder | Remove All | Keep Data | Notes |
|-------------|-----------|-----------|-------|
| **Application Files** | | | |
| `BEAR LLM AI.exe` | ✓ Deleted | ✓ Deleted | Main executable |
| `WebView2Loader.dll` | ✓ Deleted | ✓ Deleted | WebView2 bootstrapper |
| `resources\*` | ✓ Deleted | ✓ Deleted | Application resources |
| **User Data Files** | | | |
| `db.sqlite` | ✓ Deleted | ✗ Preserved | Conversation history |
| `settings.json` | ✓ Deleted | ✗ Preserved | User preferences |
| `models.json` | ✓ Deleted | ✗ Preserved | Custom models |
| `prompts.json` | ✓ Deleted | ✗ Preserved | Custom prompts |
| **Log Files** | | | |
| `preinit.log` | ✓ Deleted | ✓ Deleted | Pre-init diagnostics |
| `fatal_error.log` | ✓ Deleted | ✓ Deleted | Fatal errors |
| `crash.log` | ✓ Deleted | ✓ Deleted | Crash reports |
| `diagnostics.log` | ✓ Deleted | ✓ Deleted | Dependency checks |
| **Cache Files** | | | |
| `WebView2\*` | ✓ Deleted | ✓ Deleted | Browser cache (regenerated) |
| **System Components** | | | |
| `VC++ Runtime` | ✗ Preserved | ✗ Preserved | Shared dependency |
| `WebView2 Runtime` | ✗ Preserved | ✗ Preserved | System component |

## Decision Tree for Users

```
                     Starting Uninstall
                            │
                            ▼
              ┌─────────────────────────────┐
              │ Why are you uninstalling?   │
              └──────────┬──────────────────┘
                         │
         ┌───────────────┼───────────────┐
         │               │               │
         ▼               ▼               ▼
    ┌─────────┐   ┌──────────┐   ┌──────────┐
    │ Trying  │   │ Upgrading│   │ Removing │
    │ to fix  │   │ to newer │   │ forever  │
    │ issues  │   │ version  │   │          │
    └────┬────┘   └─────┬────┘   └────┬─────┘
         │              │              │
         ▼              ▼              ▼
    ┌─────────┐   ┌──────────┐   ┌──────────┐
    │ Choose  │   │ Choose   │   │ Choose   │
    │   YES   │   │    NO    │   │   YES    │
    │         │   │          │   │          │
    │ (Fresh  │   │ (Keep    │   │ (Remove  │
    │  start) │   │  data)   │   │  all)    │
    └─────────┘   └──────────┘   └──────────┘
```

## Code Reference

The uninstall logic is implemented in:
- **File**: `/src-tauri/windows/hooks.nsh`
- **Macro**: `NSIS_HOOK_UNINSTALL`
- **Lines**: 105-165

Key sections:
1. **Line 113-117**: Check for LocalAppData folder
2. **Line 123-125**: User prompt for data removal
3. **Line 127-140**: Remove all data branch
4. **Line 142-156**: Remove partial data branch
5. **Line 162-163**: Completion

---

**Last Updated**: 2025-10-23
**Version**: 0.0.17
