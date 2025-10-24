@echo off
REM Fix WebView2 Issues for BEAR LLM AI
REM This script helps diagnose and fix common WebView2 initialization problems

echo ========================================
echo BEAR LLM AI - WebView2 Fix Script
echo ========================================
echo.

REM Check if running as administrator
net session >nul 2>&1
if %errorLevel% == 0 (
    echo Running with administrator privileges
) else (
    echo WARNING: Not running as administrator
    echo Some operations may fail without admin rights
)
echo.

REM Step 1: Clear WebView2 user data cache
echo [1/5] Clearing WebView2 user data cache...
set APPDATA_DIR=%LOCALAPPDATA%\BEAR LLM AI\WebView2
if exist "%APPDATA_DIR%" (
    echo Found WebView2 cache at: %APPDATA_DIR%
    echo Removing cache...
    rd /s /q "%APPDATA_DIR%" 2>nul
    if exist "%APPDATA_DIR%" (
        echo WARNING: Failed to remove cache completely
        echo Please close all instances of BEAR LLM AI and try again
    ) else (
        echo ✓ Cache cleared successfully
    )
) else (
    echo ✓ No cache found (clean state)
)
echo.

REM Step 2: Verify WebView2 Runtime installation
echo [2/5] Checking WebView2 Runtime...
reg query "HKLM\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" /v pv >nul 2>&1
if %errorLevel% == 0 (
    echo ✓ WebView2 Runtime is installed
    for /f "tokens=3" %%a in ('reg query "HKLM\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" /v pv ^| findstr pv') do (
        echo   Version: %%a
    )
) else (
    echo ✗ WebView2 Runtime NOT detected
    echo.
    echo DOWNLOAD REQUIRED:
    echo Please download and install WebView2 Runtime from:
    echo https://go.microsoft.com/fwlink/p/?LinkId=2124703
    echo.
)
echo.

REM Step 3: Check Visual C++ Runtime
echo [3/5] Checking Visual C++ Runtime...
if exist "C:\Windows\System32\vcruntime140.dll" (
    echo ✓ Visual C++ Runtime (x64) installed
) else (
    echo ✗ Visual C++ Runtime (x64) NOT found
    echo Download from: https://aka.ms/vs/17/release/vc_redist.x64.exe
)
if exist "C:\Windows\SysWOW64\vcruntime140.dll" (
    echo ✓ Visual C++ Runtime (x86) installed
) else (
    echo ✗ Visual C++ Runtime (x86) NOT found
    echo Download from: https://aka.ms/vs/17/release/vc_redist.x86.exe
)
echo.

REM Step 4: Check folder permissions
echo [4/5] Checking folder permissions...
set TEST_DIR=%LOCALAPPDATA%\BEAR LLM AI
if not exist "%TEST_DIR%" (
    mkdir "%TEST_DIR%" 2>nul
)
echo test > "%TEST_DIR%\permission_test.tmp" 2>nul
if exist "%TEST_DIR%\permission_test.tmp" (
    del "%TEST_DIR%\permission_test.tmp"
    echo ✓ Folder is writable: %TEST_DIR%
) else (
    echo ✗ Cannot write to: %TEST_DIR%
    echo Please check folder permissions
)
echo.

REM Step 5: Show logs location
echo [5/5] Log file locations:
echo   App Data: %LOCALAPPDATA%\BEAR LLM AI
echo   Pre-init log: %LOCALAPPDATA%\BEAR LLM AI\preinit.log
echo   Fatal error log: %LOCALAPPDATA%\BEAR LLM AI\fatal_error.log
echo   Crash log: %LOCALAPPDATA%\BEAR LLM AI\crash.log
echo.

REM Open logs folder
echo Opening logs folder...
explorer "%LOCALAPPDATA%\BEAR LLM AI"

echo.
echo ========================================
echo Fix script completed!
echo ========================================
echo.
echo Next steps:
echo 1. Install any missing dependencies (WebView2, VC++ Runtime)
echo 2. Restart your computer
echo 3. Try launching BEAR LLM AI again
echo 4. Check the log files if issues persist
echo.
pause
