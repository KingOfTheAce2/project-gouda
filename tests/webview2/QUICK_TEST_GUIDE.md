# WebView2 Quick Testing Guide

## Quick Start - Manual Testing (5 Minutes)

### Test 1: Normal Launch
```bash
# Build and run the application
cd /workspaces/project-gouda
cargo build --release
./target/release/bear-llm-ai
```

**Expected**: Application window appears, no errors

**Check logs**:
```powershell
# Windows
type "%LOCALAPPDATA%\BEAR LLM AI\preinit.log"
```

**Look for**:
- ✓ WebView2 Runtime found
- ✓ Visual C++ Runtime installed
- ✓ WebView2 user data folder configured

---

### Test 2: Verify WebView2 Folder
```powershell
# Check that WebView2 folder was created
dir "%LOCALAPPDATA%\BEAR LLM AI\WebView2"
```

**Expected**: Folder exists with WebView2 cache files

---

### Test 3: Test Cache Recovery
```powershell
# Stop application if running
# Delete WebView2 folder
rmdir /s /q "%LOCALAPPDATA%\BEAR LLM AI\WebView2"

# Create corrupted folder (read-only)
mkdir "%LOCALAPPDATA%\BEAR LLM AI\WebView2"
attrib +r "%LOCALAPPDATA%\BEAR LLM AI\WebView2"

# Launch application
./target/release/bear-llm-ai
```

**Expected**:
- Application detects permission issue
- Removes corrupted folder
- Creates new folder
- Launches successfully

**Check preinit.log for**:
- "Existing WebView2 folder detected"
- "WebView2 folder permission error" (if corrupted)
- "Attempting to recreate WebView2 folder"

---

## Automated Testing (10 Minutes)

### Run Integration Tests
```bash
cd /workspaces/project-gouda
cargo test --test integration_tests -- --nocapture
```

**Expected Output**:
```
running 15 tests
test webview2_integration_tests::test_webview2_folder_creation ... ok
test webview2_integration_tests::test_webview2_folder_write_permissions ... ok
test webview2_integration_tests::test_corrupted_cache_recovery ... ok
test webview2_integration_tests::test_environment_variable_setup ... ok
test webview2_integration_tests::test_log_file_creation ... ok
test webview2_integration_tests::test_multiple_sequential_initializations ... ok
test webview2_integration_tests::test_path_with_special_characters ... ok
test windows_specific_tests::test_webview2_registry_access ... ok
test windows_specific_tests::test_vcredist_registry_access ... ok
test windows_specific_tests::test_webview2_filesystem_access ... ok
test windows_specific_tests::test_vcredist_dll_detection ... ok
test error_handling_tests::test_create_in_nonexistent_parent ... ok
test performance_tests::test_folder_creation_performance ... ok
test performance_tests::test_permission_check_performance ... ok

test result: ok. 15 passed; 0 failed; 0 ignored
```

---

## Quick Regression Check (2 Minutes)

### Issue 1: Launch Crash
```bash
# Test: Launch application
./target/release/bear-llm-ai
```
✅ **PASS**: Window appears, no crash

### Issue 2: Missing WebView2 Error
```powershell
# Temporarily hide WebView2 (admin required)
# Skip on systems where WebView2 cannot be disabled
```
✅ **PASS**: Clear error message in preinit.log

### Issue 3: Corrupted Cache
```powershell
# Create corrupted cache (see Test 3 above)
```
✅ **PASS**: Auto-recovery works

---

## Performance Quick Check (1 Minute)

### Measure Startup Time
```powershell
# Windows PowerShell
Measure-Command { ./target/release/bear-llm-ai }
```

**Expected**: < 3 seconds from launch to window display

---

## Log Files Quick Reference

| Log File | Location | Purpose |
|----------|----------|---------|
| preinit.log | `%LOCALAPPDATA%\BEAR LLM AI\` | Pre-initialization checks |
| fatal_error.log | `%LOCALAPPDATA%\BEAR LLM AI\` | Fatal build errors |
| crash.log | `%APPDATA%\com.bear-llm-ai\` | Panic handler output |
| diagnostics.log | `%APPDATA%\com.bear-llm-ai\` | Runtime diagnostics |

---

## Checklist - Sign Off

- [ ] Test 1: Normal launch works
- [ ] Test 2: WebView2 folder created
- [ ] Test 3: Cache recovery works
- [ ] Automated tests pass
- [ ] Regression issues resolved
- [ ] Startup time acceptable
- [ ] All logs created correctly

**Tester**: ________________
**Date**: ________________
**Status**: PASS / FAIL
**Notes**: ________________________________

---

## Troubleshooting

### Error: "WebView2 Runtime NOT found"
**Solution**: Install WebView2 Runtime
```
Download: https://go.microsoft.com/fwlink/p/?LinkId=2124703
```

### Error: "Visual C++ Runtime NOT found"
**Solution**: Install VC++ Redistributables (both x64 and x86)
```
x64: https://aka.ms/vs/17/release/vc_redist.x64.exe
x86: https://aka.ms/vs/17/release/vc_redist.x86.exe
```

### Error: "Cannot create WebView2 folder"
**Solution**: Check permissions on LocalAppData folder
```powershell
icacls "%LOCALAPPDATA%"
```

### Application crashes with no error
**Solution**: Check fatal_error.log and crash.log
```powershell
type "%LOCALAPPDATA%\BEAR LLM AI\fatal_error.log"
type "%APPDATA%\com.bear-llm-ai\crash.log"
```

---

## Quick Commands

```bash
# Build
cargo build --release

# Run tests
cargo test --test integration_tests

# Check logs (Windows PowerShell)
type "$env:LOCALAPPDATA\BEAR LLM AI\preinit.log"
type "$env:APPDATA\com.bear-llm-ai\diagnostics.log"

# Clean WebView2 cache
Remove-Item "$env:LOCALAPPDATA\BEAR LLM AI\WebView2" -Recurse -Force

# View WebView2 status
Get-ItemProperty "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}"
```

---

**Document Version**: 1.0
**Last Updated**: 2025-10-24
