; ============================================================================
; Nux Programming Language - Windows Installer
; Professional installer modeled after the Python installer UI
; ============================================================================

!include "MUI2.nsh"
!include "LogicLib.nsh"
!include "FileFunc.nsh"
!include "nsDialogs.nsh"
!include "WinMessages.nsh"
!include "x64.nsh"
!include "Sections.nsh"

; --------------- Product Information ---------------
!define PRODUCT_NAME "Nux"
!define PRODUCT_FULL_NAME "Nux Programming Language"
!define PRODUCT_VERSION "1.0.0"
!define PRODUCT_PUBLISHER "NuxLang"
!define PRODUCT_WEB_SITE "https://github.com/DoguparthiAakash/Nux_Lang"
!define PRODUCT_UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}"
!define PRODUCT_DIR_REGKEY "Software\${PRODUCT_NAME}"

; --------------- Installer Settings ---------------
Name "${PRODUCT_FULL_NAME} ${PRODUCT_VERSION}"
OutFile "nux-${PRODUCT_VERSION}-setup.exe"
InstallDir "$LOCALAPPDATA\${PRODUCT_NAME}"
InstallDirRegKey HKCU "${PRODUCT_DIR_REGKEY}" "InstallDir"
RequestExecutionLevel user
SetCompressor /SOLID lzma
ShowInstDetails show
ShowUninstDetails show

; --------------- Version Info ---------------
VIProductVersion "1.0.0.0"
VIAddVersionKey "ProductName" "${PRODUCT_FULL_NAME}"
VIAddVersionKey "CompanyName" "${PRODUCT_PUBLISHER}"
VIAddVersionKey "FileDescription" "${PRODUCT_FULL_NAME} Installer"
VIAddVersionKey "FileVersion" "${PRODUCT_VERSION}"
VIAddVersionKey "LegalCopyright" "Copyright (c) 2026 ${PRODUCT_PUBLISHER}"

; --------------- Variables ---------------
Var ActionDialog
Var ActionInstallBtn
Var ActionRepairBtn
Var ActionUninstallBtn
Var ActionCustomizeBtn
Var ActionLabel
Var ActionSubLabel
Var IsRepair
Var IsCustom

; --------------- MUI Settings ---------------
!define MUI_ABORTWARNING
!define MUI_ICON "payload\nux_file_icon.ico"
!define MUI_UNICON "payload\nux_file_icon.ico"
!define MUI_HEADERIMAGE
; MUI settings applied

; --------------- Custom Action Page ---------------
Page custom ActionPageCreate ActionPageLeave
!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES

; Finish page
!define MUI_FINISHPAGE_TITLE "Setup was successful"
!define MUI_FINISHPAGE_TEXT "${PRODUCT_FULL_NAME} ${PRODUCT_VERSION} has been installed on your computer.$\r$\n$\r$\nYou can now use the 'nux' command from any terminal.$\r$\n$\r$\nClick Finish to close this wizard."
!insertmacro MUI_PAGE_FINISH

; Uninstaller pages
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

; Language
!insertmacro MUI_LANGUAGE "English"

; ============================================================================
; ACTION PAGE (Python-style: Install Now / Customize / Repair / Uninstall)
; ============================================================================
Function ActionPageCreate
    StrCpy $IsRepair "0"
    StrCpy $IsCustom "0"

    nsDialogs::Create 1018
    Pop $ActionDialog
    ${If} $ActionDialog == error
        Abort
    ${EndIf}

    ; --- Title ---
    ${NSD_CreateLabel} 0 10u 100% 20u "${PRODUCT_FULL_NAME} ${PRODUCT_VERSION}"
    Pop $ActionLabel
    CreateFont $0 "Segoe UI" 16 700
    SendMessage $ActionLabel ${WM_SETFONT} $0 1

    ; --- Subtitle ---
    ${NSD_CreateLabel} 0 35u 100% 12u "Select an action below to get started."
    Pop $ActionSubLabel

    ; --- Install Now Button ---
    ${NSD_CreateButton} 30u 65u 230u 28u "   Install Now  (Recommended)"
    Pop $ActionInstallBtn
    ${NSD_OnClick} $ActionInstallBtn ActionInstallNow

    ${NSD_CreateLabel} 30u 95u 230u 16u "Installs Nux with default settings to $LOCALAPPDATA\Nux"
    Pop $0

    ; --- Customize Button ---
    ${NSD_CreateButton} 30u 120u 230u 28u "   Customize Installation"
    Pop $ActionCustomizeBtn
    ${NSD_OnClick} $ActionCustomizeBtn ActionCustomize

    ${NSD_CreateLabel} 30u 150u 230u 16u "Choose installation location and features."
    Pop $0

    ; --- Check if already installed for Repair/Uninstall ---
    ReadRegStr $0 HKCU "${PRODUCT_UNINST_KEY}" "UninstallString"
    ${If} $0 != ""
        ; --- Repair Button ---
        ${NSD_CreateButton} 30u 180u 110u 24u "   Repair"
        Pop $ActionRepairBtn
        ${NSD_OnClick} $ActionRepairBtn ActionRepair

        ; --- Uninstall Button ---
        ${NSD_CreateButton} 150u 180u 110u 24u "   Uninstall"
        Pop $ActionUninstallBtn
        ${NSD_OnClick} $ActionUninstallBtn ActionUninstallNow
    ${EndIf}

    nsDialogs::Show
FunctionEnd

Function ActionInstallNow
    ; Skip straight to install with defaults (all sections selected by default)
    StrCpy $IsCustom "0"
    ; Jump past components and directory pages directly to instfiles
    Abort
FunctionEnd

Function ActionCustomize
    StrCpy $IsCustom "1"
    ; Continue to next pages (Components, Directory)
FunctionEnd

Function ActionRepair
    StrCpy $IsRepair "1"
    StrCpy $IsCustom "0"
    ; Repair reinstalls everything (all sections selected by default)
    ; Skip to instfiles
    Abort
FunctionEnd

Function ActionUninstallNow
    ReadRegStr $0 HKCU "${PRODUCT_UNINST_KEY}" "UninstallString"
    ${If} $0 != ""
        ExecWait '$0'
    ${EndIf}
    Quit
FunctionEnd

Function ActionPageLeave
    ; If Install Now was clicked, skip Components + Directory pages
    ${If} $IsCustom == "0"
        ; Skip the next two pages to jump to instfiles
    ${EndIf}
FunctionEnd

; ============================================================================
; SECTIONS (Feature Selection)
; ============================================================================

Section "Nux Core (Required)" SecCore
    SectionIn RO  ; Read-only, always installed

    SetOutPath $INSTDIR
    File "payload\nux.exe"
    File "payload\nux_file_icon.ico"
    File "payload\nuxc_file_icon.ico"
    File "payload\logo.png"

    ; Write uninstaller
    WriteUninstaller "$INSTDIR\uninstall.exe"

    ; Write install info to registry
    WriteRegStr HKCU "${PRODUCT_DIR_REGKEY}" "InstallDir" "$INSTDIR"
    WriteRegStr HKCU "${PRODUCT_DIR_REGKEY}" "Version" "${PRODUCT_VERSION}"

    ; Add/Remove Programs entry
    WriteRegStr HKCU "${PRODUCT_UNINST_KEY}" "DisplayName" "${PRODUCT_FULL_NAME}"
    WriteRegStr HKCU "${PRODUCT_UNINST_KEY}" "DisplayVersion" "${PRODUCT_VERSION}"
    WriteRegStr HKCU "${PRODUCT_UNINST_KEY}" "Publisher" "${PRODUCT_PUBLISHER}"
    WriteRegStr HKCU "${PRODUCT_UNINST_KEY}" "URLInfoAbout" "${PRODUCT_WEB_SITE}"
    WriteRegStr HKCU "${PRODUCT_UNINST_KEY}" "UninstallString" "$\"$INSTDIR\uninstall.exe$\""
    WriteRegStr HKCU "${PRODUCT_UNINST_KEY}" "QuietUninstallString" "$\"$INSTDIR\uninstall.exe$\" /S"
    WriteRegStr HKCU "${PRODUCT_UNINST_KEY}" "InstallLocation" "$INSTDIR"
    WriteRegStr HKCU "${PRODUCT_UNINST_KEY}" "DisplayIcon" "$INSTDIR\nux_file_icon.ico"
    WriteRegDWORD HKCU "${PRODUCT_UNINST_KEY}" "NoModify" 1
    WriteRegDWORD HKCU "${PRODUCT_UNINST_KEY}" "NoRepair" 0

    ; Calculate installed size
    ${GetSize} "$INSTDIR" "/S=0K" $0 $1 $2
    IntFmt $0 "0x%08X" $0
    WriteRegDWORD HKCU "${PRODUCT_UNINST_KEY}" "EstimatedSize" $0

    ; Start Menu shortcuts
    CreateDirectory "$SMPROGRAMS\${PRODUCT_NAME}"
    CreateShortCut "$SMPROGRAMS\${PRODUCT_NAME}\Uninstall Nux.lnk" "$INSTDIR\uninstall.exe" "" "$INSTDIR\nux_file_icon.ico"
SectionEnd

Section "Standard Library" SecStdLib
    SetOutPath "$INSTDIR\lib"
    File /r "payload\lib\*.*"
SectionEnd

Section "Add to PATH" SecPATH
    ; Read current user PATH
    ReadRegStr $0 HKCU "Environment" "PATH"

    ; Check if already in PATH
    ${If} $0 != ""
        StrCpy $1 "$0"
        ; Simple check - search for our install dir in PATH
        Push "$1"
        Push "$INSTDIR"
        Call StrContains
        Pop $2
        ${If} $2 == ""
            ; Not found, append
            ${If} $0 != ""
                StrCpy $0 "$0;$INSTDIR"
            ${Else}
                StrCpy $0 "$INSTDIR"
            ${EndIf}
            WriteRegExpandStr HKCU "Environment" "PATH" "$0"
        ${EndIf}
    ${Else}
        WriteRegExpandStr HKCU "Environment" "PATH" "$INSTDIR"
    ${EndIf}

    ; Set NUX_HOME
    WriteRegExpandStr HKCU "Environment" "NUX_HOME" "$INSTDIR"

    ; Set NUX_LIB_PATH for library resolution
    WriteRegExpandStr HKCU "Environment" "NUX_LIB_PATH" "$INSTDIR\lib"

    ; Broadcast environment change
    SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=5000
SectionEnd

Section "File Associations (.nux, .nuxc)" SecFileAssoc
    ; .nux association
    WriteRegStr HKCU "Software\Classes\.nux" "" "Nux.SourceFile"
    WriteRegStr HKCU "Software\Classes\Nux.SourceFile" "" "Nux Source File"
    WriteRegStr HKCU "Software\Classes\Nux.SourceFile\DefaultIcon" "" "$INSTDIR\nux_file_icon.ico"
    WriteRegStr HKCU "Software\Classes\Nux.SourceFile\shell\open\command" "" '"$INSTDIR\nux.exe" run "%1"'
    WriteRegStr HKCU "Software\Classes\Nux.SourceFile\shell\edit\command" "" 'notepad.exe "%1"'

    ; .nuxc association
    WriteRegStr HKCU "Software\Classes\.nuxc" "" "Nux.CompiledFile"
    WriteRegStr HKCU "Software\Classes\Nux.CompiledFile" "" "Nux Compiled Binary"
    WriteRegStr HKCU "Software\Classes\Nux.CompiledFile\DefaultIcon" "" "$INSTDIR\nuxc_file_icon.ico"
    WriteRegStr HKCU "Software\Classes\Nux.CompiledFile\shell\open\command" "" '"$INSTDIR\nux.exe" run "%1"'

    ; Notify shell
    SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=5000
SectionEnd

; ============================================================================
; SECTION DESCRIPTIONS
; ============================================================================
!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
    !insertmacro MUI_DESCRIPTION_TEXT ${SecCore} "The Nux compiler and runtime (required)."
    !insertmacro MUI_DESCRIPTION_TEXT ${SecStdLib} "Standard library modules: math, io, string, crypto, ml, quantum, and more."
    !insertmacro MUI_DESCRIPTION_TEXT ${SecPATH} "Adds Nux to your system PATH so you can run 'nux' from any terminal."
    !insertmacro MUI_DESCRIPTION_TEXT ${SecFileAssoc} "Associates .nux and .nuxc files with the Nux runtime."
!insertmacro MUI_FUNCTION_DESCRIPTION_END

; ============================================================================
; UTILITY FUNCTIONS
; ============================================================================

; StrContains - checks if string2 is in string1
; Usage:
;   Push "haystack"
;   Push "needle"
;   Call StrContains
;   Pop $result  ; "" if not found, "needle" if found
Function StrContains
    Exch $R1 ; needle
    Exch
    Exch $R2 ; haystack
    Push $R3
    Push $R4
    Push $R5
    StrLen $R3 $R1
    StrCpy $R4 0

    loop:
        StrCpy $R5 $R2 $R3 $R4
        StrCmp $R5 "" notfound
        StrCmp $R5 $R1 found
        IntOp $R4 $R4 + 1
        Goto loop

    found:
        StrCpy $R1 $R1
        Goto done

    notfound:
        StrCpy $R1 ""

    done:
        Pop $R5
        Pop $R4
        Pop $R3
        Pop $R2
        Exch $R1
FunctionEnd

; ============================================================================
; UNINSTALLER
; ============================================================================
Section "Uninstall"
    ; Remove files
    Delete "$INSTDIR\nux.exe"
    Delete "$INSTDIR\nux_file_icon.ico"
    Delete "$INSTDIR\nuxc_file_icon.ico"
    Delete "$INSTDIR\logo.png"
    Delete "$INSTDIR\nux_remove.exe"
    Delete "$INSTDIR\uninstall.exe"
    RMDir /r "$INSTDIR\lib"
    RMDir "$INSTDIR"

    ; Remove Start Menu items
    Delete "$SMPROGRAMS\${PRODUCT_NAME}\Uninstall Nux.lnk"
    RMDir "$SMPROGRAMS\${PRODUCT_NAME}"

    ; Remove registry keys
    DeleteRegKey HKCU "${PRODUCT_UNINST_KEY}"
    DeleteRegKey HKCU "${PRODUCT_DIR_REGKEY}"

    ; Remove file associations
    DeleteRegKey HKCU "Software\Classes\.nux"
    DeleteRegKey HKCU "Software\Classes\Nux.SourceFile"
    DeleteRegKey HKCU "Software\Classes\.nuxc"
    DeleteRegKey HKCU "Software\Classes\Nux.CompiledFile"

    ; Remove from PATH
    ReadRegStr $0 HKCU "Environment" "PATH"
    ${If} $0 != ""
        ; Remove our install directory from PATH
        Push "$0"
        Push "$INSTDIR"
        Call un.StrRemoveFromPath
        Pop $0
        WriteRegExpandStr HKCU "Environment" "PATH" "$0"
    ${EndIf}

    ; Remove environment variables
    DeleteRegValue HKCU "Environment" "NUX_HOME"
    DeleteRegValue HKCU "Environment" "NUX_LIB_PATH"

    ; Broadcast change
    SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=5000
SectionEnd

; Uninstaller version of StrRemoveFromPath
Function un.StrRemoveFromPath
    Exch $R1 ; dir to remove
    Exch
    Exch $R2 ; current PATH
    Push $R3
    Push $R4

    ; Try removing ";dir" first
    StrCpy $R3 ";$R1"
    Push $R2
    Push $R3
    Call un.StrReplace
    Pop $R4

    ; Try removing "dir;" too
    StrCpy $R3 "$R1;"
    Push $R4
    Push $R3
    Call un.StrReplace
    Pop $R4

    ; Try removing exact match (if it's the only entry)
    ${If} $R4 == $R1
        StrCpy $R4 ""
    ${EndIf}

    Pop $R3
    StrCpy $R1 $R4
    Pop $R4
    Pop $R2
    Exch $R1
FunctionEnd

; Simple string replace for uninstaller
Function un.StrReplace
    Exch $R1 ; search
    Exch
    Exch $R2 ; source
    Push $R3
    Push $R4
    Push $R5

    StrLen $R3 $R1
    StrCpy $R4 0
    StrCpy $R5 ""

    loop:
        StrCpy $0 $R2 $R3 $R4
        StrCmp $0 "" done
        StrCmp $0 $R1 skip
        StrCpy $0 $R2 1 $R4
        StrCpy $R5 "$R5$0"
        IntOp $R4 $R4 + 1
        Goto loop

    skip:
        IntOp $R4 $R4 + $R3
        Goto loop

    done:
        Pop $R5
        Pop $R4
        Pop $R3
        StrCpy $R1 $R5
        Pop $R2
        Exch $R1
FunctionEnd
