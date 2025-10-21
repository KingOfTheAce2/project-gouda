; BEAR LLM AI Custom NSIS Installer Script
; Force Administrator Privileges for Installation

!include "MUI2.nsh"

; Request application privileges for Windows Vista and later
RequestExecutionLevel admin

; Installer attributes
!define PRODUCT_NAME "BEAR LLM AI"
!define PRODUCT_VERSION "0.0.10"
!define PRODUCT_PUBLISHER "Ernst A.P. van Gassen"

; Check for admin privileges at startup
Function .onInit
    UserInfo::GetAccountType
    Pop $0
    ${If} $0 != "admin"
        MessageBox MB_ICONSTOP "Administrator privileges are required to install ${PRODUCT_NAME}.$\r$\n$\r$\nPlease run the installer as Administrator."
        SetErrorLevel 740 ; ERROR_ELEVATION_REQUIRED
        Quit
    ${EndIf}
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
