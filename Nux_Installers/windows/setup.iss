[Setup]
AppId=Nux
AppName=Nux
AppVersion=0.1.0
DefaultDirName={pf}\Nux
DefaultGroupName=Nux
UninstallDisplayIcon={app}\nux.exe
Compression=lzma2
SolidCompression=yes
OutputDir=Output
OutputBaseFilename=Nux_Setup_v0.1.0
WizardStyle=modern
DisableWelcomePage=yes
DisableDirPage=yes
DisableProgramGroupPage=yes
DisableReadyPage=yes
DisableFinishedPage=yes
PrivilegesRequired=admin

[Files]
; Main application files
Source: "..\..\nux\nux_oleg\nux_dist\*"; Excludes: "target\*,target2\*,target3\*,.git\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs
; Images for custom UI
Source: "C:\Users\dogup\.gemini\antigravity-ide\brain\c33c2945-462c-4ce3-9c08-a272c98e1cbc\bg.bmp"; DestDir: "{tmp}"; Flags: dontcopy
Source: "..\..\logo\logo.bmp"; DestDir: "{tmp}"; Flags: dontcopy

[Icons]
Name: "{group}\Nux"; Filename: "{app}\nux.exe"

[Registry]
Root: HKCR; Subkey: ".nuxi"; ValueType: string; ValueName: ""; ValueData: "NuxApp"; Flags: uninsdeletevalue
Root: HKCR; Subkey: "NuxApp"; ValueType: string; ValueName: ""; ValueData: "Nux Executable"; Flags: uninsdeletekey
Root: HKCR; Subkey: "NuxApp\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\nux.exe,0"
Root: HKCR; Subkey: "NuxApp\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\nux.exe"" run ""%1"""

[Code]
var
  BgImage: TBitmapImage;
  LogoImage: TBitmapImage;
  InstallButton: TNewButton;
  PathCheckbox: TNewCheckBox;
  TitleLabel: TNewStaticText;
  DescLabel: TNewStaticText;
  ProgressLbl: TNewStaticText;

  IsUpgrade: Boolean;
  OldVersion: String;
  RepairBtn: TNewButton;
  UninstallBtn: TNewButton;

procedure UninstallButtonClick(Sender: TObject);
var
  UninstallString: String;
  ResultCode: Integer;
begin
  if RegQueryStringValue(HKEY_LOCAL_MACHINE, 'Software\Microsoft\Windows\CurrentVersion\Uninstall\Nux_is1', 'UninstallString', UninstallString) or
     RegQueryStringValue(HKEY_CURRENT_USER, 'Software\Microsoft\Windows\CurrentVersion\Uninstall\Nux_is1', 'UninstallString', UninstallString) then
  begin
    UninstallString := RemoveQuotes(UninstallString);
    Exec(UninstallString, '/SILENT', '', SW_SHOW, ewWaitUntilTerminated, ResultCode);
    WizardForm.Close;
  end;
end;

procedure InstallButtonClick(Sender: TObject);
begin
  // Start the actual installation process
  WizardForm.NextButton.OnClick(WizardForm.NextButton);
end;

procedure InitializeWizard;
begin
  // Hide standard wizard elements to create a single-page feel
  WizardForm.OuterNotebook.Hide;
  WizardForm.InnerNotebook.Hide;
  WizardForm.Bevel.Hide;
  WizardForm.NextButton.Hide;
  WizardForm.BackButton.Hide;
  WizardForm.CancelButton.Hide;
  
  // Make the wizard form a bit larger for a modern look
  WizardForm.ClientWidth := 700;
  WizardForm.ClientHeight := 450;
  WizardForm.Position := poScreenCenter;
  WizardForm.Color := clWhite;

  // Background Image
  ExtractTemporaryFile('bg.bmp');
  BgImage := TBitmapImage.Create(WizardForm);
  BgImage.Parent := WizardForm;
  BgImage.SetBounds(0, 0, WizardForm.ClientWidth, WizardForm.ClientHeight);
  BgImage.Bitmap.LoadFromFile(ExpandConstant('{tmp}\bg.bmp'));
  BgImage.Stretch := True;

  // Logo Image
  ExtractTemporaryFile('logo.bmp');
  LogoImage := TBitmapImage.Create(WizardForm);
  LogoImage.Parent := WizardForm;
  LogoImage.SetBounds(50, 20, 48, 48);
  LogoImage.Bitmap.LoadFromFile(ExpandConstant('{tmp}\logo.bmp'));
  LogoImage.Stretch := True;
  LogoImage.BackColor := clWhite;

  // Title Label
  TitleLabel := TNewStaticText.Create(WizardForm);
  TitleLabel.Parent := WizardForm;
  TitleLabel.Caption := 'Install Nux';
  TitleLabel.Font.Size := 24;
  TitleLabel.Font.Style := [fsBold];
  TitleLabel.Font.Color := clBlack;
  TitleLabel.SetBounds(110, 25, 400, 50);

  // Description
  DescLabel := TNewStaticText.Create(WizardForm);
  DescLabel.Parent := WizardForm;
  DescLabel.Caption := 'The ultimate programming language. Fast, safe, and elegant.';
  DescLabel.Font.Size := 12;
  DescLabel.Font.Color := $00404040; // Dark Gray
  DescLabel.SetBounds(110, 80, 400, 40);

  // Add to PATH Checkbox
  PathCheckbox := TNewCheckBox.Create(WizardForm);
  PathCheckbox.Parent := WizardForm;
  PathCheckbox.Caption := ' Add Nux to system PATH (Recommended)';
  PathCheckbox.Checked := True;
  PathCheckbox.Font.Size := 10;
  PathCheckbox.SetBounds(50, WizardForm.ClientHeight - 120, 300, 20);

  // Big Install Button
  InstallButton := TNewButton.Create(WizardForm);
  InstallButton.Parent := WizardForm;
  InstallButton.Caption := 'Install Now';
  InstallButton.Font.Size := 14;
  InstallButton.Font.Style := [fsBold];
  InstallButton.SetBounds(50, WizardForm.ClientHeight - 80, 200, 45);
  InstallButton.OnClick := @InstallButtonClick;
  
  if RegQueryStringValue(HKEY_LOCAL_MACHINE, 'Software\Microsoft\Windows\CurrentVersion\Uninstall\Nux_is1', 'DisplayVersion', OldVersion) or
     RegQueryStringValue(HKEY_CURRENT_USER, 'Software\Microsoft\Windows\CurrentVersion\Uninstall\Nux_is1', 'DisplayVersion', OldVersion) then
  begin
    IsUpgrade := True;
    TitleLabel.Caption := 'Manage Nux';
    DescLabel.Caption := 'Nux version ' + OldVersion + ' is currently installed. What would you like to do?';
    
    InstallButton.Hide;
    
    RepairBtn := TNewButton.Create(WizardForm);
    RepairBtn.Parent := WizardForm;
    RepairBtn.Caption := 'Repair / Upgrade';
    RepairBtn.Font.Size := 12;
    RepairBtn.Font.Style := [fsBold];
    RepairBtn.SetBounds(50, WizardForm.ClientHeight - 80, 180, 45);
    RepairBtn.OnClick := @InstallButtonClick;
    
    UninstallBtn := TNewButton.Create(WizardForm);
    UninstallBtn.Parent := WizardForm;
    UninstallBtn.Caption := 'Uninstall';
    UninstallBtn.Font.Size := 12;
    UninstallBtn.Font.Style := [fsBold];
    UninstallBtn.SetBounds(250, WizardForm.ClientHeight - 80, 150, 45);
    UninstallBtn.OnClick := @UninstallButtonClick;
  end else
  begin
    IsUpgrade := False;
  end;
  
  ProgressLbl := TNewStaticText.Create(WizardForm);
  ProgressLbl.Parent := WizardForm;
  ProgressLbl.Caption := 'Installing...';
  ProgressLbl.Font.Size := 12;
  ProgressLbl.SetBounds(50, WizardForm.ClientHeight - 50, 300, 30);
  ProgressLbl.Hide;
end;

procedure FinishButtonClick(Sender: TObject);
begin
  WizardForm.NextButton.OnClick(WizardForm.NextButton);
end;

procedure CurPageChanged(CurPageID: Integer);
begin
  if CurPageID = wpInstalling then
  begin
    InstallButton.Hide;
    if IsUpgrade then
    begin
      RepairBtn.Hide;
      UninstallBtn.Hide;
    end;
    PathCheckbox.Hide;
    TitleLabel.Caption := 'Installing Nux...';
    DescLabel.Caption := 'Please wait while Nux is being installed on your system.';
    ProgressLbl.Show;
    
    // Position progress bar
    WizardForm.ProgressGauge.Parent := WizardForm;
    WizardForm.ProgressGauge.SetBounds(50, WizardForm.ClientHeight - 80, WizardForm.ClientWidth - 100, 20);
    WizardForm.ProgressGauge.Show;
  end;
  
  if CurPageID = wpFinished then
  begin
    WizardForm.ProgressGauge.Hide;
    ProgressLbl.Hide;
    TitleLabel.Caption := 'Installation Complete';
    DescLabel.Caption := 'Nux has been successfully installed. You can now use it from the command line.';
    
    InstallButton.Caption := 'Finish';
    InstallButton.OnClick := @FinishButtonClick;
    InstallButton.Show;
  end;
end;

procedure CurStepChanged(CurStep: TSetupStep);
var
  Path: String;
begin
  if CurStep = ssPostInstall then
  begin
    if PathCheckbox.Checked then
    begin
      // Add to PATH logic via registry
      RegQueryStringValue(HKEY_LOCAL_MACHINE, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', Path);
      if Pos(';' + ExpandConstant('{app}'), ';' + Path) = 0 then
      begin
        Path := Path + ';' + ExpandConstant('{app}');
        RegWriteStringValue(HKEY_LOCAL_MACHINE, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', Path);
      end;
    end;
  end;
end;
