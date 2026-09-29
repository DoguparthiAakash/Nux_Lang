[Setup]
AppName=Nux Programming Language
AppVersion=0.1.0
AppPublisher=NuxLang Team
AppPublisherURL=https://github.com/DoguparthiAakash/Nux_Lang
DefaultDirName={pf}\Nux
DefaultGroupName=Nux Programming Language
DisableProgramGroupPage=yes
OutputDir=Output
OutputBaseFilename=Nux_Setup_v0.1.0
Compression=lzma
SolidCompression=yes
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64
PrivilegesRequired=admin
UninstallDisplayName=Nux Programming Language 0.1.0
UninstallDisplayIcon={app}\bin\nux.exe
ChangesEnvironment=yes
ChangesAssociations=yes

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
; Core compiler binary
Source: "payload\bin\nux.exe";     DestDir: "{app}\bin"; Flags: ignoreversion
; Standard library
Source: "payload\lib\*";           DestDir: "{app}\lib"; Flags: ignoreversion recursesubdirs createallsubdirs
; VS Code extension (optional)
Source: "payload\nux-lang.vsix";   DestDir: "{app}";     Flags: ignoreversion skipifsourcedoesntexist

[Icons]
Name: "{group}\Nux Command Prompt"; Filename: "{cmd}"; Parameters: "/K ""set PATH={app}\bin;%PATH% && echo Nux v0.1.0 Ready!"""
Name: "{group}\Uninstall Nux";      Filename: "{uninstallexe}"

[Registry]
; File Associations: .nux (Source Code) and .ncx (Executable Bytecode)
Root: HKLM; Subkey: "Software\Classes\.nux"; ValueType: string; ValueName: ""; ValueData: "NuxSourceFile"; Flags: uninsdeletevalue
Root: HKLM; Subkey: "Software\Classes\NuxSourceFile"; ValueType: string; ValueName: ""; ValueData: "Nux Source Code"; Flags: uninsdeletekey
Root: HKLM; Subkey: "Software\Classes\NuxSourceFile\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\bin\nux.exe,0"

Root: HKLM; Subkey: "Software\Classes\.ncx"; ValueType: string; ValueName: ""; ValueData: "NuxBytecodeFile"; Flags: uninsdeletevalue
Root: HKLM; Subkey: "Software\Classes\NuxBytecodeFile"; ValueType: string; ValueName: ""; ValueData: "Nux Executable Bytecode"; Flags: uninsdeletekey
Root: HKLM; Subkey: "Software\Classes\NuxBytecodeFile\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\bin\nux.exe,0"
Root: HKLM; Subkey: "Software\Classes\NuxBytecodeFile\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\bin\nux.exe"" run ""%1"" %*"

; Add {app}\bin to system PATH
Root: HKLM; Subkey: "System\CurrentControlSet\Control\Session Manager\Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}\bin"; Check: NeedsAddPath(ExpandConstant('{app}\bin'))
; Set NUX_HOME
Root: HKLM; Subkey: "System\CurrentControlSet\Control\Session Manager\Environment"; ValueType: string; ValueName: "NUX_HOME"; ValueData: "{app}"

[UninstallDelete]
; Clean up the install directory on uninstall
Type: filesandordirs; Name: "{app}"

[Code]
function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE, 'System\CurrentControlSet\Control\Session Manager\Environment', 'Path', OrigPath)
  then begin
    Result := True;
    exit;
  end;
  Result := Pos(';' + Param + ';', ';' + OrigPath + ';') = 0;
end;
