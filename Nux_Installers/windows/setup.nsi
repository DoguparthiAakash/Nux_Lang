!define APPNAME "Nux"
!define COMPANYNAME "NuxLang"
!define DESCRIPTION "Nux Programming Language"
!define VERSIONMAJOR 1
!define VERSIONMINOR 0
!define VERSIONBUILD 0

Name "${APPNAME}"
OutFile "nux-setup.exe"
InstallDir "$LOCALAPPDATA\Nux"

!include "MUI2.nsh"
!include "EnvVarUpdate.nsh"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

!insertmacro MUI_LANGUAGE "English"

Section "Install"
    SetOutPath $INSTDIR
    
    ; Extract the Nux binary and libraries
    ; Assuming the pack_nux.py archive was extracted here or we point directly to the bin
    ; For now, we package everything in the same dir as the nsi script for the build
    File /r "payload\*"
    
    ; Write Uninstaller
    WriteUninstaller "$INSTDIR\uninstall.exe"
    
    ; Update PATH using EnvVarUpdate macro (make sure EnvVarUpdate.nsh is available, or manually set it)
    ; For simplicity in a raw NSIS script without external plugins, we use EnVar plugin or manually modify registry
    WriteRegExpandStr HKCU "Environment" "NUX_HOME" "$INSTDIR"
    
    ; Setup File Associations
    WriteRegStr HKCU "Software\Classes\.nux" "" "Nux.File"
    WriteRegStr HKCU "Software\Classes\Nux.File" "" "Nux Source File"
    WriteRegStr HKCU "Software\Classes\Nux.File\shell\open\command" "" '"$INSTDIR\nux.exe" run "%1"'

    WriteRegStr HKCU "Software\Classes\.nuxc" "" "Nux.Compiled"
    WriteRegStr HKCU "Software\Classes\Nux.Compiled" "" "Nux Compiled Binary"
    WriteRegStr HKCU "Software\Classes\Nux.Compiled\shell\open\command" "" '"$INSTDIR\nux.exe" run "%1"'

    ; Add to PATH manually in registry
    ReadRegStr $0 HKCU "Environment" "PATH"
    WriteRegExpandStr HKCU "Environment" "PATH" "$0;$INSTDIR"

    ; Notify Windows Explorer of the PATH change
    SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=5000
SectionEnd

Section "Uninstall"
    ; Remove files
    RMDir /r "$INSTDIR"
    
    ; Remove Registry keys
    DeleteRegKey HKCU "Software\Classes\.nux"
    DeleteRegKey HKCU "Software\Classes\Nux.File"
    DeleteRegKey HKCU "Software\Classes\.nuxc"
    DeleteRegKey HKCU "Software\Classes\Nux.Compiled"
    DeleteRegValue HKCU "Environment" "NUX_HOME"
    
    ; Note: Removing from PATH cleanly via NSIS requires an advanced script or plugin (like EnVar plugin).
    ; We recommend users use the uninstaller, but removing PATH requires string manipulation.
    ; For now, the files are deleted so nux.exe won't run.
SectionEnd
