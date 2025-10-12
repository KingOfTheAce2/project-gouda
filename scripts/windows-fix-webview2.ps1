# This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
# Windows WebView2 User Data Folder Fix Script
# This script fixes permission issues for WebView2 when switching users

param(
    [switch]$Force,
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"

# Get the app data directory
$AppDataDir = "$env:LOCALAPPDATA\com.bearllm.ai"
$WebView2Dir = "$AppDataDir\EBWebView"

Write-Host "BEAR LLM AI - WebView2 Permissions Fix" -ForegroundColor Cyan
Write-Host "=======================================" -ForegroundColor Cyan
Write-Host ""

# Check if running as Administrator
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")

if ($isAdmin) {
    Write-Host "WARNING: You are running as Administrator!" -ForegroundColor Yellow
    Write-Host "This script should be run as a regular user for best results." -ForegroundColor Yellow
    Write-Host ""

    if (-not $Force) {
        $response = Read-Host "Continue anyway? (y/N)"
        if ($response -ne "y" -and $response -ne "Y") {
            Write-Host "Exiting..." -ForegroundColor Red
            exit 1
        }
    }
}

# Display current user
Write-Host "Current user: $env:USERNAME" -ForegroundColor Green
Write-Host "App data directory: $AppDataDir" -ForegroundColor Green
Write-Host ""

# Check if app data directory exists
if (-not (Test-Path $AppDataDir)) {
    Write-Host "App data directory does not exist. Creating..." -ForegroundColor Yellow
    New-Item -ItemType Directory -Path $AppDataDir -Force | Out-Null
    Write-Host "Created: $AppDataDir" -ForegroundColor Green
}

# Check WebView2 directory
if (Test-Path $WebView2Dir) {
    Write-Host "WebView2 directory exists: $WebView2Dir" -ForegroundColor Yellow

    # Check ownership
    $acl = Get-Acl $WebView2Dir
    Write-Host "Current owner: $($acl.Owner)" -ForegroundColor Cyan

    # Test write permissions
    $testFile = "$WebView2Dir\.permission_test"
    try {
        [System.IO.File]::WriteAllText($testFile, "test")
        Remove-Item $testFile -Force
        Write-Host "Write permissions: OK" -ForegroundColor Green
    } catch {
        Write-Host "Write permissions: FAILED" -ForegroundColor Red
        Write-Host "Error: $_" -ForegroundColor Red

        if ($Force) {
            Write-Host "Attempting to fix permissions..." -ForegroundColor Yellow

            # Take ownership
            Write-Host "Taking ownership of $WebView2Dir..." -ForegroundColor Yellow
            takeown /F "$WebView2Dir" /R /D Y 2>&1 | Out-Null

            # Grant full control to current user
            Write-Host "Granting full control to $env:USERNAME..." -ForegroundColor Yellow
            icacls "$WebView2Dir" /grant "${env:USERNAME}:(OI)(CI)F" /T 2>&1 | Out-Null

            # Test again
            try {
                [System.IO.File]::WriteAllText($testFile, "test")
                Remove-Item $testFile -Force
                Write-Host "Permissions fixed successfully!" -ForegroundColor Green
            } catch {
                Write-Host "Failed to fix permissions. Please run the cleanup script." -ForegroundColor Red
                exit 1
            }
        } else {
            Write-Host ""
            Write-Host "Run with -Force to attempt automatic fix, or delete the folder manually:" -ForegroundColor Yellow
            Write-Host "  Remove-Item -Recurse -Force '$WebView2Dir'" -ForegroundColor Cyan
            exit 1
        }
    }
} else {
    Write-Host "WebView2 directory does not exist yet (will be created on first run)" -ForegroundColor Green
}

Write-Host ""
Write-Host "Permission check complete!" -ForegroundColor Green
Write-Host ""

# Display recommendations
Write-Host "Recommendations:" -ForegroundColor Cyan
Write-Host "  1. Always run BEAR LLM AI as a regular user (not as Administrator)" -ForegroundColor White
Write-Host "  2. Each Windows user should install their own copy of the app" -ForegroundColor White
Write-Host "  3. Use 'Install for current user only' during installation" -ForegroundColor White
Write-Host ""

# Show WebView2 runtime info
Write-Host "Checking WebView2 runtime..." -ForegroundColor Cyan
try {
    $webview2Version = Get-ItemProperty -Path "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv -ErrorAction SilentlyContinue
    if ($webview2Version) {
        Write-Host "WebView2 runtime version: $($webview2Version.pv)" -ForegroundColor Green
    } else {
        Write-Host "WebView2 runtime not found. It will be installed automatically." -ForegroundColor Yellow
    }
} catch {
    Write-Host "Could not check WebView2 runtime version" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Done!" -ForegroundColor Green
