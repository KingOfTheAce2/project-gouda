npx claude-flow@alpha hive-mind spawn "<context>There are persistant WebView 2 issues. There have been multiple attempts to fix it. I have a GitHub repo of BEAR-LLM, where this error is not. After installing and running the program once, I get these logs: 
[2025-10-24 20:25:42] === PRE-INITIALIZATION CHECK ===
[2025-10-24 20:25:42] ✓ WebView2 Runtime found (Registry): version 141.0.3537.92
[2025-10-24 20:25:42] ✓ Visual C++ Runtime installed: x64 - version v14.44.35211.00, x64 (WOW64) - version v14.44.35211.00, x86 (WOW64) - version v14.44.35211.00
[2025-10-24 20:25:42] Pre-initialization check complete. Log: "C:\\Users\\evgga\\AppData\\Local\\BEAR LLM AI\\preinit.log"
[2025-10-24 20:25:42] Proceeding to Tauri initialization...

[2025-10-24 20:25:48] ✓ WebView2 user data folder configured: "C:\\Users\\evgga\\AppData\\Local\\BEAR LLM AI\\WebView2"

[2025-10-24 20:31:43] === PRE-INITIALIZATION CHECK ===
[2025-10-24 20:31:43] ✓ WebView2 Runtime found (Registry): version 141.0.3537.92
[2025-10-24 20:31:43] ✓ Visual C++ Runtime installed: x64 - version v14.44.35211.00, x64 (WOW64) - version v14.44.35211.00, x86 (WOW64) - version v14.44.35211.00
[2025-10-24 20:31:43] Pre-initialization check complete. Log: "C:\\Users\\evgga\\AppData\\Local\\BEAR LLM AI\\preinit.log"
[2025-10-24 20:31:43] Proceeding to Tauri initialization...

[2025-10-24 20:31:49] Existing WebView2 folder detected, verifying integrity...
[2025-10-24 20:31:49] ✓ WebView2 folder is writable
[2025-10-24 20:31:49] ✓ WebView2 user data folder configured: "C:\\Users\\evgga\\AppData\\Local\\BEAR LLM AI\\WebView2"
</context>. <task>Your task is to check the fixes and patches. Delete all old non-working code and implement a working solution if you find anything that does not work. Add more detailed logging locally if needed</task>" --claude

Why are these not deleted upon deinstall:
"C:\Users\evgga\AppData\Local\BEAR LLM AI\WebView2"
"C:\Users\evgga\AppData\Local\BEAR LLM AI\preinit.log"