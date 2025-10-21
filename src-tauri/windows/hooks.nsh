; NSIS Installer Hooks for BEAR LLM AI
; Automatically installs Visual C++ Runtime and other dependencies
; This ensures the application works out of the box without manual dependency installation

!macro NSIS_HOOK_POSTINSTALL
  ; This hook runs after the application files have been installed
  ; but before the installer finishes

  DetailPrint "Checking for required runtime dependencies..."

  ; ========================================
  ; Visual C++ Redistributable 2015-2022
  ; ========================================

  ; First check if VC++ Runtime is already installed via registry
  ReadRegStr $0 HKLM "SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64" "Installed"
  ${If} $0 == "1"
    DetailPrint "Visual C++ Runtime already installed, skipping..."
    Goto VC_INSTALLED
  ${EndIf}

  ; Also check alternative registry location
  ReadRegStr $0 HKLM "SOFTWARE\WOW6432Node\Microsoft\VisualStudio\14.0\VC\Runtimes\x64" "Installed"
  ${If} $0 == "1"
    DetailPrint "Visual C++ Runtime already installed (WOW64), skipping..."
    Goto VC_INSTALLED
  ${EndIf}

  ; VC++ not found, install it
  DetailPrint "Installing Visual C++ Redistributable..."

  ; The VC++ installer is bundled in resources/windows/vc_redist.x64.exe
  SetOutPath "$TEMP"
  File "${RESOURCES}\windows\vc_redist.x64.exe"

  ; Run the installer silently
  ; /install = install mode
  ; /quiet = no UI
  ; /norestart = don't restart computer automatically
  ExecWait '"$TEMP\vc_redist.x64.exe" /install /quiet /norestart' $0

  ; Check exit code
  ${If} $0 == 0
    DetailPrint "Visual C++ Runtime installed successfully"
  ${ElseIf} $0 == 3010
    DetailPrint "Visual C++ Runtime installed (restart required)"
  ${ElseIf} $0 == 1638
    DetailPrint "Visual C++ Runtime: newer version already installed"
  ${Else}
    DetailPrint "Visual C++ Runtime installation returned code: $0"
  ${EndIf}

  ; Clean up temporary installer
  Delete "$TEMP\vc_redist.x64.exe"

  VC_INSTALLED:

  ; ========================================
  ; WebView2 Runtime Check
  ; ========================================
  ; Note: WebView2 is handled by Tauri's built-in downloadBootstrapper
  ; This is just a status check for logging purposes

  ReadRegStr $0 HKLM "SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" "pv"
  ${If} $0 != ""
    DetailPrint "WebView2 Runtime version $0 detected"
  ${Else}
    DetailPrint "WebView2 will be downloaded by Tauri installer..."
  ${EndIf}

  ; ========================================
  ; Final Status
  ; ========================================
  DetailPrint "Dependency check complete. Application is ready to use!"

!macroend

!macro NSIS_HOOK_PREINSTALL
  ; This hook runs before installation begins
  ; Can be used for additional pre-flight checks if needed

  DetailPrint "Preparing BEAR LLM AI installation..."

  ; Verify Windows version
  ; Tauri requires Windows 10 1809 or later
  ${If} ${AtLeastWin10}
    DetailPrint "Windows version check passed"
  ${Else}
    MessageBox MB_ICONSTOP "BEAR LLM AI requires Windows 10 version 1809 or later.$\r$\n$\r$\nPlease upgrade your Windows installation."
    Abort "Unsupported Windows version"
  ${EndIf}

!macroend

!macro NSIS_HOOK_UNINSTALL
  ; This hook runs during uninstallation
  ; We don't uninstall VC++ Runtime as other applications may depend on it

  DetailPrint "Uninstalling BEAR LLM AI..."
  DetailPrint "Note: Visual C++ Runtime is preserved (may be used by other applications)"

!macroend
