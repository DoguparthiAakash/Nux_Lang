[Setup]
AppName=Nux Programming Language
AppVersion=1.1.0
AppPublisher=NuxLang Team
AppPublisherURL=https://github.com/DoguparthiAakash/Nux_Lang
DefaultDirName={localappdata}\Nux\1.1.0
DefaultGroupName=Nux Programming Language
DisableProgramGroupPage=yes
OutputBaseFilename=Nux_Setup
Compression=lzma
SolidCompression=yes
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64
PrivilegesRequired=lowest

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
; The payload folder is populated by build_exe.ps1 before running ISCC
Source: "payload\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\Nux Command Prompt"; Filename: "{cmd}"; Parameters: "/K ""set PATH={app};%PATH% && echo Nux Environment Ready!"""
Name: "{group}\Uninstall Nux"; Filename: "{uninstallexe}"

[Registry]
; Add to current user PATH
Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; Check: NeedsAddPath('{app}')

[Code]
function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', OrigPath)
  then begin
    Result := True;
    exit;
  end;
  Result := Pos(';' + Param + ';', ';' + OrigPath + ';') = 0;
end;
