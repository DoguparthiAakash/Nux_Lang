Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

$form = New-Object System.Windows.Forms.Form
$form.Text = 'Nux Language Setup'
$form.Size = New-Object System.Drawing.Size(400,300)
$form.StartPosition = 'CenterScreen'
$form.FormBorderStyle = 'FixedDialog'
$form.MaximizeBox = $false

$font = New-Object System.Drawing.Font("Segoe UI", 10)
$form.Font = $font

# Logo / Title
$labelTitle = New-Object System.Windows.Forms.Label
$labelTitle.Text = 'Install Nux'
$labelTitle.Font = New-Object System.Drawing.Font("Segoe UI", 16, [System.Drawing.FontStyle]::Bold)
$labelTitle.AutoSize = $true
$labelTitle.Location = New-Object System.Drawing.Point(20, 20)
$form.Controls.Add($labelTitle)

$labelSub = New-Object System.Windows.Forms.Label
$labelSub.Text = 'The native, high-performance systems language.'
$labelSub.AutoSize = $true
$labelSub.Location = New-Object System.Drawing.Point(22, 50)
$form.Controls.Add($labelSub)

# Checkbox
$checkboxPath = New-Object System.Windows.Forms.CheckBox
$checkboxPath.Text = 'Add Nux to PATH'
$checkboxPath.AutoSize = $true
$checkboxPath.Location = New-Object System.Drawing.Point(25, 100)
$checkboxPath.Checked = $true
$form.Controls.Add($checkboxPath)

# Status TextBox
$textBoxStatus = New-Object System.Windows.Forms.TextBox
$textBoxStatus.Multiline = $true
$textBoxStatus.ReadOnly = $true
$textBoxStatus.ScrollBars = 'Vertical'
$textBoxStatus.Location = New-Object System.Drawing.Point(25, 130)
$textBoxStatus.Size = New-Object System.Drawing.Size(335, 80)
$form.Controls.Add($textBoxStatus)

# Install Button
$buttonInstall = New-Object System.Windows.Forms.Button
$buttonInstall.Text = 'Install Now'
$buttonInstall.Location = New-Object System.Drawing.Point(25, 220)
$buttonInstall.Size = New-Object System.Drawing.Size(100, 30)
$form.Controls.Add($buttonInstall)

# Cancel Button
$buttonCancel = New-Object System.Windows.Forms.Button
$buttonCancel.Text = 'Cancel'
$buttonCancel.Location = New-Object System.Drawing.Point(260, 220)
$buttonCancel.Size = New-Object System.Drawing.Size(100, 30)
$form.Controls.Add($buttonCancel)

$buttonCancel.Add_Click({
    $form.Close()
})

$buttonInstall.Add_Click({
    $buttonInstall.Enabled = $false
    $buttonCancel.Enabled = $false
    $checkboxPath.Enabled = $false
    
    $textBoxStatus.Text = "Building Nux Compiler and VM...`r`n"
    $form.Refresh()
    
    $nux_src = "E:\nux\Nux_Lang\nux\nux_oleg\nux_portable"
    $nux_bin_dir = "$nux_src\target\release"
    $nux_bin = "$nux_bin_dir\nux.exe"
    
    # Run build in background to keep GUI responsive
    $buildJob = Start-Job -ScriptBlock {
        Set-Location $args[0]
        cargo build --release --bin nux 2>&1 | Out-String
    } -ArgumentList $nux_src
    
    while ($buildJob.State -eq 'Running') {
        Start-Sleep -Milliseconds 100
        [System.Windows.Forms.Application]::DoEvents()
    }
    
    $buildOutput = Receive-Job $buildJob
    Remove-Job $buildJob
    
    if (Test-Path $nux_bin) {
        $textBoxStatus.AppendText("Build successful!`r`n")
        
        if ($checkboxPath.Checked) {
            try {
                $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
                if ($userPath -notmatch [regex]::Escape($nux_bin_dir)) {
                    $newPath = $userPath + ";$nux_bin_dir"
                    [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
                    $textBoxStatus.AppendText("Successfully added Nux to User PATH.`r`n")
                } else {
                    $textBoxStatus.AppendText("Nux is already in your User PATH.`r`n")
                }
            } catch {
                $textBoxStatus.AppendText("Failed to add to PATH automatically.`r`n")
            }
        }
        $textBoxStatus.AppendText("`r`nInstallation Complete!")
        $buttonCancel.Text = "Close"
        $buttonCancel.Enabled = $true
    } else {
        $textBoxStatus.AppendText("Build failed! Check compiler output.`r`n$buildOutput")
        $buttonCancel.Text = "Close"
        $buttonCancel.Enabled = $true
    }
})

$form.ShowDialog() | Out-Null
