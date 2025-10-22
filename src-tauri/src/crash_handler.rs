// This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
// MIT License Copyright (c) 2024-present Frank Zhang

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::panic;
use chrono::Local;

/// Initialize crash logging system that writes panic information to a file
/// before the application terminates.
pub fn init_crash_handler(app_data_dir: &PathBuf) {
    let crash_log_path = app_data_dir.join("crash.log");
    let crash_log_path_clone = crash_log_path.clone();

    // Set up panic hook to capture crash information
    panic::set_hook(Box::new(move |panic_info| {
        let mut crash_log = String::new();

        crash_log.push_str(&format!(
            "\n{:=<80}\n",
            ""
        ));
        crash_log.push_str(&format!(
            "CRASH REPORT - {}\n",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        ));
        crash_log.push_str(&format!(
            "{:=<80}\n\n",
            ""
        ));

        // Capture panic location
        if let Some(location) = panic_info.location() {
            crash_log.push_str(&format!(
                "Location: {}:{}:{}\n",
                location.file(),
                location.line(),
                location.column()
            ));
        }

        // Capture panic message
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            crash_log.push_str(&format!("Panic message: {}\n", s));
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            crash_log.push_str(&format!("Panic message: {}\n", s));
        } else {
            crash_log.push_str("Panic message: <unknown>\n");
        }

        // Add backtrace if available
        crash_log.push_str("\nBacktrace:\n");
        crash_log.push_str(&format!("{:?}\n", std::backtrace::Backtrace::force_capture()));

        crash_log.push_str(&format!(
            "\n{:=<80}\n\n",
            ""
        ));

        // Write to file
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&crash_log_path_clone)
        {
            let _ = file.write_all(crash_log.as_bytes());
            let _ = file.flush();

            // Also print to stderr
            eprintln!("{}", crash_log);
            eprintln!("Crash log written to: {:?}", crash_log_path_clone);
        } else {
            eprintln!("Failed to write crash log to: {:?}", crash_log_path_clone);
            eprintln!("{}", crash_log);
        }
    }));

    log::info!("Crash handler initialized. Crash logs will be written to: {:?}", crash_log_path);
}

/// Write diagnostic information to a file for troubleshooting
pub fn write_diagnostic_info(app_data_dir: &PathBuf, info: &str) {
    let diagnostic_path = app_data_dir.join("diagnostics.log");

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let entry = format!("[{}] {}\n", timestamp, info);

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&diagnostic_path)
    {
        let _ = file.write_all(entry.as_bytes());
        log::info!("{}", info);
    }
}

/// Check if WebView2 runtime is installed on Windows
#[cfg(target_os = "windows")]
pub fn check_webview2_runtime() -> Result<String, String> {
    use std::process::Command;

    // Method 1: Check registry for WebView2 runtime version
    let registry_check = Command::new("reg")
        .args(&[
            "query",
            "HKLM\\SOFTWARE\\WOW6432Node\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}",
            "/v",
            "pv"
        ])
        .output();

    if let Ok(output) = registry_check {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Parse version from registry output
            if let Some(version_line) = stdout.lines().find(|line| line.contains("pv")) {
                if let Some(version) = version_line.split_whitespace().last() {
                    return Ok(format!("WebView2 Runtime found (Registry): version {}", version));
                }
            }
        }
    }

    // Method 2: Check for WebView2Loader.dll in system
    let system_paths = [
        r"C:\Program Files (x86)\Microsoft\EdgeWebView\Application",
        r"C:\Program Files\Microsoft\EdgeWebView\Application",
    ];

    for path in &system_paths {
        let webview2_path = std::path::Path::new(path);
        if webview2_path.exists() {
            return Ok(format!("WebView2 Runtime found (System): {:?}", path));
        }
    }

    // Method 3: Check if Edge browser is installed (provides WebView2)
    let edge_check = Command::new("reg")
        .args(&[
            "query",
            "HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths\\msedge.exe",
            "/ve"
        ])
        .output();

    if let Ok(output) = edge_check {
        if output.status.success() {
            return Ok("WebView2 Runtime found (Microsoft Edge installed)".to_string());
        }
    }

    Err("WebView2 Runtime NOT found - installation may fail".to_string())
}

#[cfg(not(target_os = "windows"))]
pub fn check_webview2_runtime() -> Result<String, String> {
    Ok("WebView2 check skipped (not Windows)".to_string())
}

/// Check if Visual C++ Runtime is installed with detailed diagnostics
#[cfg(target_os = "windows")]
pub fn check_vcredist_runtime() -> Result<String, String> {
    check_vcredist_runtime_detailed(false)
}

/// Internal function with optional verbose logging
#[cfg(target_os = "windows")]
fn check_vcredist_runtime_detailed(verbose: bool) -> Result<String, String> {
    use std::process::Command;

    // Registry paths to check for VC++ 2015-2022 runtime (most common)
    let registry_paths = [
        // x64 paths
        ("HKLM\\SOFTWARE\\Microsoft\\VisualStudio\\14.0\\VC\\Runtimes\\x64", "x64"),
        ("HKLM\\SOFTWARE\\WOW6432Node\\Microsoft\\VisualStudio\\14.0\\VC\\Runtimes\\x64", "x64 (WOW64)"),
        // x86 paths (also needed by some applications)
        ("HKLM\\SOFTWARE\\Microsoft\\VisualStudio\\14.0\\VC\\Runtimes\\x86", "x86"),
        ("HKLM\\SOFTWARE\\WOW6432Node\\Microsoft\\VisualStudio\\14.0\\VC\\Runtimes\\x86", "x86 (WOW64)"),
    ];

    let mut found_versions = Vec::new();
    let mut diagnostic_info = Vec::new();

    if verbose {
        diagnostic_info.push("Starting VC++ Runtime detection...".to_string());
    }

    // Check all registry paths
    for (path, arch) in &registry_paths {
        if verbose {
            diagnostic_info.push(format!("Checking registry: {}", path));
        }

        let vcredist_check = Command::new("reg")
            .args(&["query", path, "/v", "Installed"])
            .output();

        match vcredist_check {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);

                if verbose {
                    diagnostic_info.push(format!("  Registry output: {}", stdout.lines().collect::<Vec<_>>().join(" | ")));
                }

                // Check if the Installed value is 0x1 (indicating installed)
                if stdout.contains("Installed") && (stdout.contains("0x1") || (stdout.contains("REG_DWORD") && stdout.contains("1"))) {
                    // Try to get the version
                    let version_output = Command::new("reg")
                        .args(&["query", path, "/v", "Version"])
                        .output();

                    if let Ok(ver_out) = version_output {
                        let ver_stdout = String::from_utf8_lossy(&ver_out.stdout);
                        if let Some(version_line) = ver_stdout.lines().find(|line| line.contains("Version")) {
                            if let Some(version) = version_line.split_whitespace().last() {
                                found_versions.push(format!("{} - version {}", arch, version));
                                if verbose {
                                    diagnostic_info.push(format!("  ✓ Found: {} version {}", arch, version));
                                }
                                continue;
                            }
                        }
                    }
                    found_versions.push(format!("{}", arch));
                    if verbose {
                        diagnostic_info.push(format!("  ✓ Found: {} (version unknown)", arch));
                    }
                } else if verbose {
                    diagnostic_info.push(format!("  ✗ Not installed or invalid value"));
                }
            }
            Ok(output) => {
                if verbose {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    diagnostic_info.push(format!("  ✗ Registry query failed: {}", stderr));
                }
            }
            Err(e) => {
                if verbose {
                    diagnostic_info.push(format!("  ✗ Command error: {}", e));
                }
            }
        }
    }

    if !found_versions.is_empty() {
        let result = format!("Visual C++ Runtime installed: {}", found_versions.join(", "));
        if verbose {
            diagnostic_info.push(format!("Result: {}", result));
        }
        return Ok(result);
    }

    // Fallback: Check for actual VC++ runtime DLL files in System32
    if verbose {
        diagnostic_info.push("Registry check failed, checking DLL files...".to_string());
    }

    let system_paths = [
        r"C:\Windows\System32\vcruntime140.dll",
        r"C:\Windows\System32\msvcp140.dll",
        r"C:\Windows\SysWOW64\vcruntime140.dll",
        r"C:\Windows\SysWOW64\msvcp140.dll",
    ];

    let mut found_dlls = Vec::new();
    for dll_path in &system_paths {
        let exists = std::path::Path::new(dll_path).exists();
        if verbose {
            diagnostic_info.push(format!("  {} {}", if exists { "✓" } else { "✗" }, dll_path));
        }
        if exists {
            found_dlls.push(*dll_path);
        }
    }

    if !found_dlls.is_empty() {
        let result = format!("Visual C++ Runtime DLLs found: {} of {} files detected ({})",
            found_dlls.len(), system_paths.len(),
            found_dlls.join(", "));
        if verbose {
            diagnostic_info.push(format!("Result: {}", result));
        }
        return Ok(result);
    }

    // Not found - provide helpful error message
    let error_msg = format!(
        "Visual C++ Runtime NOT found - Application requires Microsoft Visual C++ 2015-2022 Redistributable (x64 and x86). \
        Download from: https://aka.ms/vs/17/release/vc_redist.x64.exe and https://aka.ms/vs/17/release/vc_redist.x86.exe"
    );

    if verbose {
        diagnostic_info.push(format!("Result: {}", error_msg));
        diagnostic_info.push("Diagnostic info:".to_string());
        for info in &diagnostic_info {
            eprintln!("{}", info);
        }
    }

    Err(error_msg)
}

#[cfg(not(target_os = "windows"))]
pub fn check_vcredist_runtime() -> Result<String, String> {
    Ok("VC++ Runtime check skipped (not Windows)".to_string())
}

/// Run comprehensive dependency checks and log results
pub fn run_dependency_diagnostics(app_data_dir: &PathBuf) {
    write_diagnostic_info(app_data_dir, "=== DEPENDENCY DIAGNOSTICS ===");

    // Check WebView2
    match check_webview2_runtime() {
        Ok(msg) => write_diagnostic_info(app_data_dir, &format!("✓ {}", msg)),
        Err(msg) => write_diagnostic_info(app_data_dir, &format!("✗ {}", msg)),
    }

    // Check VC++ Runtime
    match check_vcredist_runtime() {
        Ok(msg) => write_diagnostic_info(app_data_dir, &format!("✓ {}", msg)),
        Err(msg) => write_diagnostic_info(app_data_dir, &format!("✗ {}", msg)),
    }

    // Check WebView2Loader.dll in application directory
    #[cfg(target_os = "windows")]
    {
        use std::env;
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let webview2_loader = exe_dir.join("WebView2Loader.dll");
                if webview2_loader.exists() {
                    write_diagnostic_info(
                        app_data_dir,
                        &format!("✓ WebView2Loader.dll found at: {:?}", webview2_loader)
                    );
                } else {
                    write_diagnostic_info(
                        app_data_dir,
                        &format!("✗ WebView2Loader.dll NOT found at: {:?}", webview2_loader)
                    );
                }
            }
        }
    }

    // Windows version check
    #[cfg(target_os = "windows")]
    {
        use sysinfo::System;
        let _sys = System::new_all();
        write_diagnostic_info(
            app_data_dir,
            &format!("OS: {} {}", System::name().unwrap_or_default(), System::os_version().unwrap_or_default())
        );
    }

    write_diagnostic_info(app_data_dir, "=== END DIAGNOSTICS ===");
}
