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
InstallDir "$LOCALAPPDATA\${PRODUCT_NAME}\${PRODUCT_VERSION}"
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
Var ActionAddPathCb
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

; ActionPage functions moved to the bottom so they can reference Section indices.

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

    ; Create junction for 'current'
    ; Remove old junction if it exists
    nsExec::Exec 'cmd /c rmdir "$LOCALAPPDATA\${PRODUCT_NAME}\current"'
    ; Create new junction
    nsExec::Exec 'cmd /c mklink /J "$LOCALAPPDATA\${PRODUCT_NAME}\current" "$INSTDIR"'
SectionEnd

Section "Standard Library" SecStdLib
    SetOutPath "$INSTDIR\lib"
    File /r "payload\lib\*.*"
SectionEnd

Section "Add to PATH" SecPATH
    ; Read current user PATH
    ReadRegStr $0 HKCU "Environment" "PATH"
    
    StrCpy $1 "$LOCALAPPDATA\${PRODUCT_NAME}\current"

    ; Check if already in PATH
    ${If} $0 != ""
        StrCpy $2 "$0"
        ; Simple check - search for our install dir in PATH
        Push "$2"
        Push "$1"
        Call StrContains
        Pop $3
        ${If} $3 == ""
            ; Not found, append
            StrCpy $0 "$0;$1"
            WriteRegExpandStr HKCU "Environment" "PATH" "$0"
        ${EndIf}
    ${Else}
        WriteRegExpandStr HKCU "Environment" "PATH" "$1"
    ${EndIf}

    ; Set NUX_HOME
    WriteRegExpandStr HKCU "Environment" "NUX_HOME" "$1"

    ; Set NUX_LIB_PATH for library resolution
    WriteRegExpandStr HKCU "Environment" "NUX_LIB_PATH" "$1\lib"

    ; Broadcast environment change
    SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=5000
SectionEnd

Section "File Associations (.nux, .nuxc)" SecFileAssoc
    ; .nux association
    WriteRegStr HKCU "Software\Classes\.nux" "" "Nux.SourceFile"
    WriteRegStr HKCU "Software\Classes\Nux.SourceFile" "" "Nux Source File"
    WriteRegStr HKCU "Software\Classes\Nux.SourceFile\DefaultIcon" "" "$INSTDIR\nux_file_icon.ico"
    WriteRegStr HKCU "Software\Classes\Nux.SourceFile\shell\open\command" "" '"$LOCALAPPDATA\${PRODUCT_NAME}\current\nux.exe" run "%1"'
    WriteRegStr HKCU "Software\Classes\Nux.SourceFile\shell\edit\command" "" 'notepad.exe "%1"'

    ; .nuxc association
    WriteRegStr HKCU "Software\Classes\.nuxc" "" "Nux.CompiledFile"
    WriteRegStr HKCU "Software\Classes\Nux.CompiledFile" "" "Nux Compiled Binary"
    WriteRegStr HKCU "Software\Classes\Nux.CompiledFile\DefaultIcon" "" "$INSTDIR\nuxc_file_icon.ico"
    WriteRegStr HKCU "Software\Classes\Nux.CompiledFile\shell\open\command" "" '"$LOCALAPPDATA\${PRODUCT_NAME}\current\nux.exe" run "%1"'

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
    ${NSD_CreateLabel} 0 0 100% 24u "${PRODUCT_FULL_NAME} ${PRODUCT_VERSION}"
    Pop $ActionLabel
    CreateFont $0 "Segoe UI" 14 700
    SendMessage $ActionLabel ${WM_SETFONT} $0 1

    ; --- Check if already installed ---
    ReadRegStr $0 HKCU "${PRODUCT_UNINST_KEY}" "UninstallString"
    ${If} $0 == ""
        ; --- Install Now Button ---
        ${NSD_CreateButton} 15u 35u 270u 32u "Install Now"
        Pop $ActionInstallBtn
        ${NSD_OnClick} $ActionInstallBtn ActionInstallNow
        CreateFont $1 "Segoe UI" 11 700
        SendMessage $ActionInstallBtn ${WM_SETFONT} $1 1

        ${NSD_CreateLabel} 25u 70u 260u 16u "Installs to: $LOCALAPPDATA\${PRODUCT_NAME}\${PRODUCT_VERSION}$\r$\nIncludes Core Compiler, StdLib, and File Associations."
        Pop $0

        ; --- Customize Button ---
        ${NSD_CreateButton} 15u 95u 270u 24u "Customize installation"
        Pop $ActionCustomizeBtn
        ${NSD_OnClick} $ActionCustomizeBtn ActionCustomize
        CreateFont $2 "Segoe UI" 10 400
        SendMessage $ActionCustomizeBtn ${WM_SETFONT} $2 1

        ; --- PATH Checkbox ---
        ${NSD_CreateCheckbox} 15u 125u 270u 12u "Add Nux to PATH"
        Pop $ActionAddPathCb
        ${NSD_Check} $ActionAddPathCb ; checked by default
    ${Else}
        ; --- Repair Button ---
        ${NSD_CreateButton} 15u 35u 270u 32u "Repair Nux"
        Pop $ActionRepairBtn
        ${NSD_OnClick} $ActionRepairBtn ActionRepair
        CreateFont $1 "Segoe UI" 11 700
        SendMessage $ActionRepairBtn ${WM_SETFONT} $1 1

        ; --- Uninstall Button ---
        ${NSD_CreateButton} 15u 75u 270u 32u "Uninstall Nux"
        Pop $ActionUninstallBtn
        ${NSD_OnClick} $ActionUninstallBtn ActionUninstallNow
        CreateFont $2 "Segoe UI" 11 700
        SendMessage $ActionUninstallBtn ${WM_SETFONT} $2 1
    ${EndIf}

    nsDialogs::Show
FunctionEnd

Function ActionInstallNow
    ${NSD_GetState} $ActionAddPathCb $0
    ${If} $0 == 1
        !insertmacro SelectSection ${SecPATH}
    ${Else}
        !insertmacro UnselectSection ${SecPATH}
    ${EndIf}
    
    StrCpy $IsCustom "0"
    Abort
FunctionEnd

Function ActionCustomize
    ${NSD_GetState} $ActionAddPathCb $0
    ${If} $0 == 1
        !insertmacro SelectSection ${SecPATH}
    ${Else}
        !insertmacro UnselectSection ${SecPATH}
    ${EndIf}

    StrCpy $IsCustom "1"
FunctionEnd

Function ActionRepair
    StrCpy $IsRepair "1"
    StrCpy $IsCustom "0"
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
    ${If} $IsCustom == "0"
        ; Skip the next two pages to jump to instfiles
    ${EndIf}
FunctionEnd


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
    ; Remove files for this version
    Delete "$INSTDIR\nux.exe"
    Delete "$INSTDIR\nux_file_icon.ico"
    Delete "$INSTDIR\nuxc_file_icon.ico"
    Delete "$INSTDIR\logo.png"
    Delete "$INSTDIR\nux_remove.exe"
    Delete "$INSTDIR\uninstall.exe"
    RMDir /r "$INSTDIR\lib"
    RMDir "$INSTDIR"
    
    ; If 'current' junction points to us, remove it
    nsExec::Exec 'cmd /c rmdir "$LOCALAPPDATA\${PRODUCT_NAME}\current"'

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
        ; Remove current dir from PATH
        Push "$0"
        Push "$LOCALAPPDATA\${PRODUCT_NAME}\current"
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
