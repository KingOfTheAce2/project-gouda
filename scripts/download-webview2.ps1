# WebView2 Runtime Downloader for Fixed Runtime Bundling
# Use this script if you want to bundle the full WebView2 runtime (fixedRuntime mode)

param(
    [string]$Version = "130.0.2849.56",  # Update to latest stable version
    [string]$OutputPath = "src-tauri/WebView2Runtime.exe"
)

Write-Host "WebView2 Fixed Runtime Downloader" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan
Write-Host ""

# Check if we're in the correct directory
if (-not (Test-Path "src-tauri")) {
    Write-Host "Error: Must run from project root directory" -ForegroundColor Red
    exit 1
}

Write-Host "Version: $Version" -ForegroundColor Yellow
Write-Host "Output: $OutputPath" -ForegroundColor Yellow
Write-Host ""

# Construct download URL
$url = "https://msedge.sf.dl.delivery.mp.microsoft.com/filestreamingservice/files/${Version}/MicrosoftEdgeWebView2RuntimeInstaller${Version}.exe"

Write-Host "Downloading WebView2 Runtime..." -ForegroundColor Green
try {
    Invoke-WebRequest -Uri $url -OutFile $OutputPath -UseBasicParsing
    Write-Host "✓ Downloaded successfully" -ForegroundColor Green

    # Verify file
    $fileInfo = Get-Item $OutputPath
    Write-Host "✓ File size: $([math]::Round($fileInfo.Length / 1MB, 2)) MB" -ForegroundColor Green

    Write-Host ""
    Write-Host "Next Steps:" -ForegroundColor Cyan
    Write-Host "1. Update tauri.conf.json with:" -ForegroundColor White
    Write-Host '   "webviewInstallMode": {' -ForegroundColor Gray
    Write-Host '     "type": "fixedRuntime",' -ForegroundColor Gray
    Write-Host '     "path": "WebView2Runtime.exe"' -ForegroundColor Gray
    Write-Host '   }' -ForegroundColor Gray
    Write-Host "2. Build your app: npm run tauri build" -ForegroundColor White
    Write-Host ""
    Write-Host "Note: This will increase installer size by ~150MB but enables" -ForegroundColor Yellow
    Write-Host "      completely offline installation with no runtime dependencies." -ForegroundColor Yellow

} catch {
    Write-Host "✗ Download failed: $_" -ForegroundColor Red
    exit 1
}
