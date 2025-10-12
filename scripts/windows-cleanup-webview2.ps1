# This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
# Windows WebView2 Cleanup Script
# Use this script to completely clean up WebView2 data before reinstalling

param(
    [switch]$Force
)

$ErrorActionPreference = "Continue"

Write-Host "BEAR LLM AI - WebView2 Complete Cleanup" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "WARNING: This will delete all BEAR LLM AI data including:" -ForegroundColor Yellow
Write-Host "  - Chat histories" -ForegroundColor Yellow
Write-Host "  - Settings" -ForegroundColor Yellow
Write-Host "  - WebView2 cache and data" -ForegroundColor Yellow
Write-Host ""

if (-not $Force) {
    $response = Read-Host "Are you sure you want to continue? (yes/N)"
    if ($response -ne "yes") {
        Write-Host "Cleanup cancelled." -ForegroundColor Green
        exit 0
    }
}

$AppDataDir = "$env:LOCALAPPDATA\com.bearllm.ai"

Write-Host ""
Write-Host "Removing app data from: $AppDataDir" -ForegroundColor Yellow

if (Test-Path $AppDataDir) {
    try {
        # Take ownership first
        Write-Host "Taking ownership..." -ForegroundColor Cyan
        takeown /F "$AppDataDir" /R /D Y 2>&1 | Out-Null

        # Grant permissions
        Write-Host "Granting permissions..." -ForegroundColor Cyan
        icacls "$AppDataDir" /grant "${env:USERNAME}:(OI)(CI)F" /T 2>&1 | Out-Null

        # Remove directory
        Write-Host "Deleting files..." -ForegroundColor Cyan
        Remove-Item -Recurse -Force "$AppDataDir" -ErrorAction Stop

        Write-Host "Successfully removed: $AppDataDir" -ForegroundColor Green
    } catch {
        Write-Host "Error removing directory: $_" -ForegroundColor Red
        Write-Host ""
        Write-Host "Manual cleanup required:" -ForegroundColor Yellow
        Write-Host "  1. Close all BEAR LLM AI windows" -ForegroundColor Cyan
        Write-Host "  2. Open File Explorer" -ForegroundColor Cyan
        Write-Host "  3. Navigate to: $env:LOCALAPPDATA" -ForegroundColor Cyan
        Write-Host "  4. Delete the 'com.bearllm.ai' folder" -ForegroundColor Cyan
        exit 1
    }
} else {
    Write-Host "App data directory does not exist (already clean)" -ForegroundColor Green
}

# Clean up any related WebView2 folders
Write-Host ""
Write-Host "Checking for related WebView2 folders..." -ForegroundColor Cyan
$relatedFolders = Get-ChildItem "$env:LOCALAPPDATA" -Directory -ErrorAction SilentlyContinue | Where-Object { $_.Name -match "bearllm" -or $_.Name -match "EBWebView" }

if ($relatedFolders) {
    foreach ($folder in $relatedFolders) {
        Write-Host "Found: $($folder.FullName)" -ForegroundColor Yellow
        try {
            Remove-Item -Recurse -Force $folder.FullName -ErrorAction Stop
            Write-Host "Removed: $($folder.Name)" -ForegroundColor Green
        } catch {
            Write-Host "Could not remove: $($folder.Name)" -ForegroundColor Red
        }
    }
} else {
    Write-Host "No related folders found" -ForegroundColor Green
}

Write-Host ""
Write-Host "Cleanup complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "  1. Download the latest installer" -ForegroundColor White
Write-Host "  2. Run the installer as a REGULAR USER (not as Administrator)" -ForegroundColor White
Write-Host "  3. Choose 'Install for current user only'" -ForegroundColor White
Write-Host "  4. Launch the application normally" -ForegroundColor White
Write-Host ""
