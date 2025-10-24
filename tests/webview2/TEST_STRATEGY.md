# WebView2 Testing Strategy - BEAR LLM AI

## Executive Summary

This document outlines the comprehensive testing strategy for WebView2 functionality in BEAR LLM AI. The implementation has been updated based on the working reference implementation from BEAR-LLM repository to resolve persistent initialization crashes.

**Test Objective**: Validate that the WebView2 integration works correctly across all supported scenarios and that previous crash issues are fully resolved.

---

## 1. Implementation Analysis

### 1.1 Key Implementation Components

#### Pre-Initialization Checks (main.rs, lines 8-68)
- **WebView2 Runtime Detection**: Validates WebView2 runtime availability before Tauri starts
- **VC++ Runtime Detection**: Verifies Visual C++ redistributable installation
- **Comprehensive Logging**: Writes to `preinit.log` in LocalAppData

#### WebView2 User Data Folder Setup (main.rs, lines 70-154)
- **Early Configuration**: Sets WEBVIEW2_USER_DATA_FOLDER **before** Tauri initialization
- **Folder Validation**: Tests write permissions and handles corrupted cache
- **Auto-Recovery**: Recreates corrupted WebView2 directories automatically
- **Environment Variables**: Sets both WEBVIEW2_USER_DATA_FOLDER and WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS

#### Crash Handler (crash_handler.rs)
- **Runtime Detection Methods**:
  - Registry check for EdgeUpdate clients
  - System paths scanning
  - Edge browser detection
- **VC++ Runtime Validation**: Multiple fallback methods including registry and DLL checks
- **Comprehensive Diagnostics**: Logs all dependency checks to diagnostics.log

#### Initialization Flow (init.rs)
- **Post-Validation**: Verifies WebView2 folder writability during init
- **Database Initialization**: Safely initializes SQLite after environment setup
- **Window Management**: Shows window only after successful initialization

---

## 2. Test Coverage Matrix

### 2.1 Unit Testing

#### Test Suite 1: WebView2 Runtime Detection
```rust
#[cfg(test)]
mod webview2_runtime_tests {
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn test_webview2_detection_success() {
        // Test: WebView2 runtime detection returns success
        // Expected: Ok(String) with version or path information
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_webview2_detection_registry_method() {
        // Test: Registry-based detection
        // Expected: Version string from registry
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_webview2_detection_filesystem_method() {
        // Test: Filesystem-based detection
        // Expected: Path to WebView2 installation
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_webview2_detection_edge_fallback() {
        // Test: Edge browser fallback detection
        // Expected: Confirmation Edge provides WebView2
    }
}
```

#### Test Suite 2: VC++ Runtime Detection
```rust
#[cfg(test)]
mod vcredist_runtime_tests {
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn test_vcredist_detection_x64() {
        // Test: x64 VC++ runtime detection
        // Expected: Success with version information
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_vcredist_detection_x86() {
        // Test: x86 VC++ runtime detection
        // Expected: Success with version information
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_vcredist_dll_fallback() {
        // Test: DLL-based detection fallback
        // Expected: List of found DLL files
    }
}
```

#### Test Suite 3: WebView2 Folder Management
```rust
#[cfg(test)]
mod webview2_folder_tests {
    use super::*;

    #[test]
    fn test_webview2_folder_creation() {
        // Test: Create WebView2 user data folder
        // Expected: Folder created successfully
    }

    #[test]
    fn test_webview2_folder_permissions() {
        // Test: Verify write permissions
        // Expected: Can write test file successfully
    }

    #[test]
    fn test_webview2_corrupted_cache_recovery() {
        // Test: Recovery from corrupted cache
        // Expected: Old folder removed, new one created
    }

    #[test]
    fn test_webview2_environment_variables() {
        // Test: Environment variables are set correctly
        // Expected: WEBVIEW2_USER_DATA_FOLDER points to correct path
    }
}
```

### 2.2 Integration Testing

#### Test Suite 4: Initialization Sequence
```rust
#[cfg(test)]
mod initialization_tests {
    #[test]
    fn test_full_initialization_sequence() {
        // Test: Complete initialization from main() to window display
        // Steps:
        //   1. Pre-init checks run
        //   2. WebView2 folder configured
        //   3. Tauri builder succeeds
        //   4. Setup handler completes
        //   5. Window becomes visible
        // Expected: No panics, all steps complete successfully
    }

    #[test]
    fn test_initialization_with_missing_webview2() {
        // Test: Initialization when WebView2 not installed
        // Expected: Clear error message, diagnostic logs created
    }

    #[test]
    fn test_initialization_with_missing_vcredist() {
        // Test: Initialization when VC++ runtime not installed
        // Expected: Clear error message with download links
    }

    #[test]
    fn test_initialization_error_logging() {
        // Test: Error logging functionality
        // Expected: Errors written to fatal_error.log and preinit.log
    }
}
```

#### Test Suite 5: Cross-Component Integration
```rust
#[cfg(test)]
mod cross_component_tests {
    #[test]
    fn test_crash_handler_integration() {
        // Test: Crash handler catches initialization failures
        // Expected: Crash logged to crash.log
    }

    #[test]
    fn test_diagnostic_logging_integration() {
        // Test: Diagnostics written correctly during init
        // Expected: diagnostics.log contains all checks
    }

    #[test]
    fn test_database_initialization_after_webview() {
        // Test: Database initializes after WebView2 setup
        // Expected: No conflicts, both initialize successfully
    }
}
```

### 2.3 End-to-End Testing

#### Test Suite 6: Full Application Lifecycle
```
Test Case E2E-1: Clean Installation
  Prerequisites: Windows 10/11, WebView2 installed, VC++ installed
  Steps:
    1. Fresh install of application
    2. Launch application
    3. Verify window appears
    4. Interact with UI (create conversation)
    5. Close application
  Expected: All steps complete without errors

Test Case E2E-2: First Launch After Installation
  Prerequisites: Fresh Windows system, WebView2 NOT installed
  Steps:
    1. Install application
    2. Launch application
    3. Check for error message
    4. Install WebView2
    5. Relaunch application
  Expected: Clear error message, successful launch after installation

Test Case E2E-3: Corrupted Cache Recovery
  Prerequisites: Existing installation with corrupted WebView2 cache
  Steps:
    1. Manually corrupt WebView2 folder (remove permissions)
    2. Launch application
    3. Check logs
    4. Verify application recovers
  Expected: Application detects corruption, recreates folder, launches successfully

Test Case E2E-4: Upgrade Scenario
  Prerequisites: Previous version installed
  Steps:
    1. Install new version
    2. Launch application
    3. Verify existing data preserved
    4. Verify WebView2 functionality
  Expected: Smooth upgrade, no data loss

Test Case E2E-5: Multi-Instance Handling
  Prerequisites: Application running
  Steps:
    1. Launch first instance
    2. Launch second instance
    3. Verify behavior
  Expected: Proper multi-instance handling (or clear message if not supported)
```

---

## 3. Test Scenarios

### 3.1 Positive Test Scenarios

| ID | Scenario | Expected Outcome |
|----|----------|------------------|
| P01 | WebView2 installed, clean environment | Application starts successfully |
| P02 | WebView2 installed via Edge | Application detects Edge-provided WebView2 |
| P03 | Both x64 and x86 VC++ installed | All runtime checks pass |
| P04 | Restart after WebView2 folder corruption | Application auto-recovers |
| P05 | Multiple launches in sequence | No conflicts, each launch succeeds |

### 3.2 Negative Test Scenarios

| ID | Scenario | Expected Outcome |
|----|----------|------------------|
| N01 | WebView2 NOT installed | Clear error message with installation instructions |
| N02 | VC++ x64 missing | Error message with download link |
| N03 | VC++ x86 missing | Error message with download link |
| N04 | No write permissions to LocalAppData | Error logged, clear message to user |
| N05 | WebView2 folder locked by another process | Error message, recovery instructions |

### 3.3 Edge Cases

| ID | Scenario | Expected Outcome |
|----|----------|------------------|
| E01 | WebView2 partially installed | Fallback detection methods succeed |
| E02 | Registry corrupted but files exist | Filesystem detection succeeds |
| E03 | System32 permissions restricted | Diagnostic logging handles gracefully |
| E04 | LocalAppData path contains unicode | Paths handled correctly |
| E05 | System time incorrect | Timestamps still logged correctly |

---

## 4. Performance Testing

### 4.1 Startup Performance

```
Test: Measure time from process start to window visible

Metrics:
- Pre-initialization checks: < 100ms
- WebView2 folder setup: < 50ms
- Tauri initialization: < 2000ms
- Database initialization: < 500ms
- Total startup time: < 3000ms (3 seconds)

Test Method:
- Add timestamp logging at each stage
- Run 10 iterations
- Calculate average and standard deviation
```

### 4.2 Memory Usage

```
Test: Monitor memory consumption during initialization

Metrics:
- Process start: Baseline
- After WebView2 init: < +50MB
- After window display: < +150MB
- Steady state: < 200MB total

Test Method:
- Use Windows Performance Monitor
- Track Private Bytes metric
- Compare against reference implementation
```

---

## 5. Security Testing

### 5.1 File System Security

```
Test: Verify WebView2 folder permissions
- Folder accessible only by current user
- No world-writable permissions
- Proper ACLs on Windows

Test: Verify log file security
- Crash logs don't contain sensitive data
- Diagnostic logs properly sanitized
- No API keys or passwords in logs
```

### 5.2 Registry Access

```
Test: Verify registry access is read-only
- No registry modifications during detection
- Proper error handling for access denied
- No elevation requests
```

---

## 6. Regression Testing

### 6.1 Previous Issues Validation

#### Issue 1: Initialization Crash on Launch
**Original Error**: Application crashed immediately on launch with no window displayed

**Test Validation**:
1. ✅ Pre-initialization checks now run BEFORE Tauri initialization
2. ✅ WebView2 folder configured BEFORE window creation
3. ✅ Comprehensive error logging added
4. ✅ Crash handler captures all panics

**Regression Test**:
```
1. Launch application on clean system
2. Verify: Window appears (no crash)
3. Verify: preinit.log shows successful checks
4. Verify: No entries in fatal_error.log
```

#### Issue 2: Missing WebView2 Runtime
**Original Error**: No clear message when WebView2 missing

**Test Validation**:
1. ✅ Three detection methods implemented
2. ✅ Clear error messages in logs
3. ✅ Installation instructions provided

**Regression Test**:
```
1. Temporarily rename WebView2 installation
2. Launch application
3. Verify: preinit.log shows missing WebView2 warning
4. Verify: Error message includes installation help
5. Restore WebView2 and verify recovery
```

#### Issue 3: Corrupted WebView2 Cache
**Original Error**: Application failed with permission errors

**Test Validation**:
1. ✅ Write permission check added
2. ✅ Auto-recovery from corrupted cache
3. ✅ Detailed logging of recovery process

**Regression Test**:
```
1. Create WebView2 folder with no write permissions
2. Launch application
3. Verify: Application detects permission issue
4. Verify: Old folder removed and recreated
5. Verify: Successful launch after recovery
```

---

## 7. Manual Testing Checklist

### 7.1 Installation Testing
- [ ] Fresh install on Windows 10
- [ ] Fresh install on Windows 11
- [ ] Upgrade from previous version
- [ ] Install with WebView2 already present
- [ ] Install without WebView2
- [ ] Install without VC++ runtime

### 7.2 Runtime Testing
- [ ] First launch after install
- [ ] Second launch (verify cached data)
- [ ] Launch after system restart
- [ ] Launch with corrupted cache
- [ ] Launch with full disk (LocalAppData)
- [ ] Launch with read-only LocalAppData

### 7.3 Functionality Testing
- [ ] Window displays correctly
- [ ] UI is responsive
- [ ] Database operations work
- [ ] Settings persist across launches
- [ ] Conversations load correctly
- [ ] LLM interactions function

### 7.4 Error Handling Testing
- [ ] Graceful handling of missing dependencies
- [ ] Clear error messages displayed
- [ ] Logs created in correct locations
- [ ] Recovery from errors works
- [ ] No data loss on errors

### 7.5 Log Verification
- [ ] preinit.log contains pre-initialization checks
- [ ] diagnostics.log contains runtime checks
- [ ] fatal_error.log created on fatal errors
- [ ] crash.log captures panics
- [ ] All logs have timestamps
- [ ] No sensitive data in logs

---

## 8. Comparison with BEAR-LLM Reference

### 8.1 Implementation Alignment

| Component | BEAR-LLM (Reference) | Current Implementation | Status |
|-----------|----------------------|------------------------|--------|
| Pre-init checks | ✅ Present | ✅ Implemented | ✅ Aligned |
| WebView2 folder setup | ✅ Early setup | ✅ Before Tauri init | ✅ Aligned |
| Runtime detection | ✅ Multi-method | ✅ Registry + filesystem + Edge | ✅ Aligned |
| Error logging | ✅ Comprehensive | ✅ preinit.log + fatal_error.log | ✅ Aligned |
| Crash handler | ✅ Panic hook | ✅ Backtrace capture | ✅ Aligned |
| Diagnostics | ✅ Multiple checks | ✅ WebView2 + VC++ + system | ✅ Aligned |

### 8.2 Key Improvements

1. **Timing Fix**: WebView2 folder setup moved from init.rs to main.rs (before Tauri initialization)
2. **Enhanced Logging**: Added preinit.log for early stage logging
3. **Auto-Recovery**: Added corrupted cache detection and recreation
4. **Permission Checks**: Added write permission validation before use
5. **Multi-Variable Setup**: Sets both WEBVIEW2_USER_DATA_FOLDER and WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS

---

## 9. Test Execution Plan

### 9.1 Phase 1: Unit Tests (Day 1)
- Execute all unit test suites
- Fix any failures
- Achieve >90% code coverage for crash_handler.rs

### 9.2 Phase 2: Integration Tests (Day 2)
- Execute integration test suites
- Verify cross-component interactions
- Test error propagation paths

### 9.3 Phase 3: Manual Testing (Day 3)
- Execute manual testing checklist
- Test on multiple Windows versions
- Document any issues found

### 9.4 Phase 4: Regression Testing (Day 4)
- Verify all previous issues resolved
- Compare with BEAR-LLM reference behavior
- Validate against historical bug reports

### 9.5 Phase 5: Performance & Security (Day 5)
- Execute performance tests
- Run security validation
- Generate final report

---

## 10. Test Environment Requirements

### 10.1 Hardware
- Minimum: Windows 10 x64
- Recommended: Windows 11 x64
- RAM: 4GB minimum, 8GB recommended
- Disk: 2GB free space

### 10.2 Software
- Windows 10 version 1809 or later
- Windows 11 (all versions)
- WebView2 Runtime (for positive tests)
- Visual C++ 2015-2022 Redistributable (x64 and x86)

### 10.3 Test Configurations

**Config A - Full Stack**: WebView2 + VC++ x64 + VC++ x86 (all installed)
**Config B - Missing WebView2**: No WebView2, VC++ installed
**Config C - Missing VC++ x64**: WebView2 + VC++ x86 only
**Config D - Missing VC++ x86**: WebView2 + VC++ x64 only
**Config E - Minimal**: No WebView2, no VC++
**Config F - Edge Only**: No WebView2 runtime, only Edge browser

---

## 11. Success Criteria

### 11.1 Functional Criteria
- ✅ Application launches successfully with all dependencies
- ✅ Clear error messages when dependencies missing
- ✅ Auto-recovery from corrupted WebView2 cache
- ✅ No crashes during normal operation
- ✅ All logs created in correct locations

### 11.2 Performance Criteria
- ✅ Startup time < 3 seconds
- ✅ Pre-init checks < 100ms
- ✅ Memory usage < 200MB steady state

### 11.3 Quality Criteria
- ✅ Unit test coverage > 90%
- ✅ All integration tests pass
- ✅ All manual tests pass
- ✅ Zero critical issues
- ✅ All previous issues resolved

---

## 12. Risk Assessment

### 12.1 High Risk Areas
1. **WebView2 Runtime Detection**: Multiple methods needed for reliability
2. **Permission Handling**: LocalAppData folder permissions vary
3. **Timing Issues**: WebView2 setup must happen before Tauri init

### 12.2 Mitigation Strategies
1. Implemented three detection methods (registry, filesystem, Edge)
2. Added write permission checks with auto-recovery
3. Moved WebView2 setup to main.rs before any Tauri calls

---

## 13. Test Reporting

### 13.1 Test Execution Report Template
```
Date: [DATE]
Tester: [NAME]
Build Version: [VERSION]
OS: [Windows VERSION]

Test Results:
- Unit Tests: [PASSED/TOTAL]
- Integration Tests: [PASSED/TOTAL]
- Manual Tests: [PASSED/TOTAL]
- Regression Tests: [PASSED/TOTAL]

Issues Found: [COUNT]
Critical: [COUNT]
High: [COUNT]
Medium: [COUNT]
Low: [COUNT]

Overall Status: [PASS/FAIL]
```

### 13.2 Issue Report Template
```
Issue ID: [ID]
Severity: [Critical/High/Medium/Low]
Found in: [Test Phase]
Description: [DESCRIPTION]
Steps to Reproduce:
  1. [STEP 1]
  2. [STEP 2]
Expected: [EXPECTED BEHAVIOR]
Actual: [ACTUAL BEHAVIOR]
Logs: [LOG FILE PATHS]
Screenshots: [IF APPLICABLE]
```

---

## 14. Conclusion

This comprehensive testing strategy ensures that the WebView2 implementation in BEAR LLM AI is:
- **Robust**: Handles all error conditions gracefully
- **Reliable**: Consistent behavior across different environments
- **Maintainable**: Clear logging and diagnostics
- **User-Friendly**: Clear error messages and auto-recovery

The implementation aligns with the working BEAR-LLM reference and includes additional improvements for reliability and user experience.

---

**Document Version**: 1.0
**Last Updated**: 2025-10-24
**Next Review**: After test execution completion
