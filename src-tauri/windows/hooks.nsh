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

  ; Use /nonfatal flag - if the file is not found, continue without error
  File /nonfatal "${RESOURCES}\windows\vc_redist.x64.exe"

  ; Check if the file was actually extracted
  IfFileExists "$TEMP\vc_redist.x64.exe" 0 VC_NOT_FOUND

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
  Goto VC_INSTALLED

  VC_NOT_FOUND:
    DetailPrint "Visual C++ Runtime installer not found in bundle, skipping..."
    DetailPrint "Note: Application may require manual installation of Visual C++ Runtime"

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

  ; Get LocalAppData folder
  ReadEnvStr $0 LOCALAPPDATA
  ${If} $0 == ""
    DetailPrint "Could not determine LocalAppData folder, skipping cleanup"
    Goto CLEANUP_DONE
  ${EndIf}

  ; Define the app data folder
  StrCpy $1 "$0\BEAR LLM AI"

  ; Check if the folder exists before attempting to delete
  ${If} ${FileExists} "$1\*.*"
    ; Ask user if they want to remove all data including settings and logs
    MessageBox MB_YESNO|MB_ICONQUESTION "Do you want to remove all application data including:$\r$\n$\r$\n  • Settings and configuration$\r$\n  • Conversation history$\r$\n  • Log files$\r$\n  • Cache files$\r$\n$\r$\nLocation: $1$\r$\n$\r$\nSelect 'No' to keep your data for future installations." /SD IDYES IDYES REMOVE_ALL_DATA IDNO REMOVE_PARTIAL_DATA

    REMOVE_ALL_DATA:
      DetailPrint "Removing all application data from: $1"

      ; Remove the entire application data folder
      RMDir /r "$1"

      ; Verify removal
      ${If} ${FileExists} "$1\*.*"
        DetailPrint "Warning: Some files could not be removed (may be in use)"
        DetailPrint "Please manually delete: $1"
      ${Else}
        DetailPrint "All application data removed successfully"
      ${EndIf}
      Goto CLEANUP_DONE

    REMOVE_PARTIAL_DATA:
      DetailPrint "Removing temporary data only (keeping settings and database)..."

      ; Remove only temporary/cache files
      Delete "$1\preinit.log"
      Delete "$1\fatal_error.log"
      Delete "$1\crash.log"
      Delete "$1\diagnostics.log"

      ; Remove WebView2 cache (can be regenerated)
      RMDir /r "$1\WebView2"

      DetailPrint "Temporary data cleanup complete"
      DetailPrint "User data preserved at: $1"
      Goto CLEANUP_DONE

  ${Else}
    DetailPrint "No application data found to clean up"
  ${EndIf}

  CLEANUP_DONE:
  DetailPrint "Uninstallation complete"

!macroend
