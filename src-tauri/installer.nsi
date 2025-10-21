; BEAR LLM AI Custom NSIS Installer Script
; Support both per-user and per-machine installation

!include "MUI2.nsh"

; Request user privileges (will prompt for admin if user chooses system-wide)
RequestExecutionLevel user

; Installer attributes
!define PRODUCT_NAME "BEAR LLM AI"
!define PRODUCT_VERSION "0.0.10"
!define PRODUCT_PUBLISHER "Ernst A.P. van Gassen"

; Variables for installation mode
Var InstallMode

; Installation mode selection page
!include "Sections.nsh"
!include "LogicLib.nsh"

; Custom page for installation mode selection
Function InstallModePageCreate
    nsDialogs::Create 1018
    Pop $0
    ${If} $0 == error
        Abort
    ${EndIf}

    ; Add description text
    ${NSD_CreateLabel} 0 0 100% 24u "Choose installation type:"
    Pop $0

    ; Create radio buttons
    ${NSD_CreateRadioButton} 10 30u 100% 12u "Install for current user only (recommended, no admin required)"
    Pop $1
    ${NSD_CreateRadioButton} 10 50u 100% 12u "Install for all users (requires administrator privileges)"
    Pop $2

    ; Set default to current user
    ${NSD_Check} $1

    nsDialogs::Show
FunctionEnd

Function InstallModePageLeave
    ${NSD_GetState} $1 $0
    ${If} $0 == ${BST_CHECKED}
        StrCpy $InstallMode "CurrentUser"
        SetShellVarContext current
    ${Else}
        StrCpy $InstallMode "AllUsers"
        ; Check for admin privileges
        UserInfo::GetAccountType
        Pop $0
        ${If} $0 != "admin"
            MessageBox MB_YESNO "Installing for all users requires administrator privileges.$\r$\n$\r$\nDo you want to install for current user only instead?" IDYES InstallCurrentUser
            MessageBox MB_ICONSTOP "Please run the installer as Administrator to install for all users, or choose 'Install for current user only'."
            Abort

            InstallCurrentUser:
            StrCpy $InstallMode "CurrentUser"
            SetShellVarContext current
        ${Else}
            SetShellVarContext all
        ${EndIf}
    ${EndIf}
FunctionEnd

; Initialization
Function .onInit
    ; Default to current user mode
    StrCpy $InstallMode "CurrentUser"
    SetShellVarContext current
FunctionEnd

; Additional NSIS configuration
!insertmacro MUI_LANGUAGE "English"

; Installation settings
SetCompressor /SOLID lzma
SetCompress auto
SetDatablockOptimize on

; Show installation details
ShowInstDetails show
ShowUnInstDetails show
