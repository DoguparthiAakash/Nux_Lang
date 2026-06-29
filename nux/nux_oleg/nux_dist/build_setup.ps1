$ErrorActionPreference = "Stop"

Write-Host "====================================" -ForegroundColor Cyan
Write-Host " Building NuxSetup.exe Installer" -ForegroundColor Cyan
Write-Host "====================================" -ForegroundColor Cyan

# 1. Build the zip file payload (we can just use the existing nux_release.zip that we built earlier)
# But we need to make sure the zip file doesn't have a subfolder if we extract it directly.
# The previous zip file `nux_release.zip` has everything at the root, which is perfect.
$ZipPath = ".\nux_release.zip"
if (-not (Test-Path $ZipPath)) {
    Write-Host "Error: nux_release.zip not found! Please run build_dist.ps1 first." -ForegroundColor Red
    exit 1
}

# 2. Write the C# Installer Source Code
$CSharpCode = @"
using System;
using System.IO;
using System.IO.Compression;
using System.Reflection;
using Microsoft.Win32;
using System.Runtime.InteropServices;

class Installer {
    [DllImport("Shell32.dll", CharSet = CharSet.Auto, SetLastError = true)]
    public static extern void SHChangeNotify(uint wEventId, uint uFlags, IntPtr dwItem1, IntPtr dwItem2);

    static void Main() {
        Console.Title = "Nux Installer";
        Console.WriteLine("====================================");
        Console.WriteLine(" Nux Programming Language Installer ");
        Console.WriteLine("====================================");
        
        try {
            string installDir = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "Nux");
            
            Console.WriteLine("\n[1/4] Preparing installation directory...");
            if (Directory.Exists(installDir)) {
                // Delete existing files
                foreach (string file in Directory.GetFiles(installDir, "*", SearchOption.AllDirectories)) {
                    try { File.Delete(file); } catch { }
                }
            } else {
                Directory.CreateDirectory(installDir);
            }

            Console.WriteLine("[2/4] Extracting Nux compiler and libraries...");
            using (Stream resStream = Assembly.GetExecutingAssembly().GetManifestResourceStream("nux_release.zip")) {
                if (resStream == null) {
                    Console.WriteLine("Error: Could not find embedded payload!");
                    Console.ReadLine();
                    return;
                }
                using (ZipArchive archive = new ZipArchive(resStream, ZipArchiveMode.Read)) {
                    // Extracting all files manually to handle overwrites if needed
                    foreach (ZipArchiveEntry entry in archive.Entries) {
                        string destPath = Path.Combine(installDir, entry.FullName);
                        if (string.IsNullOrEmpty(entry.Name)) {
                            Directory.CreateDirectory(destPath);
                        } else {
                            Directory.CreateDirectory(Path.GetDirectoryName(destPath));
                            entry.ExtractToFile(destPath, true);
                        }
                    }
                }
            }

            Console.WriteLine("[3/4] Configuring Environment PATH...");
            string path = Environment.GetEnvironmentVariable("PATH", EnvironmentVariableTarget.User);
            if (path == null || !path.Contains(installDir)) {
                string newPath = string.IsNullOrEmpty(path) ? installDir : path + ";" + installDir;
                Environment.SetEnvironmentVariable("PATH", newPath, EnvironmentVariableTarget.User);
            }

            Console.WriteLine("[4/4] Registering file associations (.nux, .nuxc)...");
            RegisterExtension(".nux", "Nux.Source", "Nux Source File", Path.Combine(installDir, "nux.exe"), Path.Combine(installDir, "nux_icon.ico"));
            RegisterExtension(".nuxc", "Nux.Compiled", "Nux Compiled Object", Path.Combine(installDir, "nux.exe"), Path.Combine(installDir, "nuxc_icon.ico"));
            
            // Notify Explorer of registry changes (SHCNE_ASSOCCHANGED = 0x08000000, SHCNF_IDLIST = 0)
            SHChangeNotify(0x08000000, 0, IntPtr.Zero, IntPtr.Zero);

            Console.WriteLine("\n====================================");
            Console.WriteLine("       Installation Complete!       ");
            Console.WriteLine("====================================");
            Console.WriteLine("\nYou can now run 'nux' from any new command prompt.");
            Console.WriteLine("Press any key to exit...");
            Console.ReadKey();
        } catch (Exception ex) {
            Console.WriteLine("\nAn error occurred during installation:");
            Console.WriteLine(ex.Message);
            Console.WriteLine("\nPress any key to exit...");
            Console.ReadKey();
        }
    }

    static void RegisterExtension(string ext, string progId, string desc, string exePath, string iconPath) {
        using (RegistryKey key = Registry.CurrentUser.CreateSubKey(string.Format(@"Software\Classes\{0}", ext))) {
            key.SetValue("", progId);
        }
        using (RegistryKey key = Registry.CurrentUser.CreateSubKey(string.Format(@"Software\Classes\{0}", progId))) {
            key.SetValue("", desc);
            using (RegistryKey defaultIcon = key.CreateSubKey("DefaultIcon")) {
                defaultIcon.SetValue("", iconPath);
            }
            using (RegistryKey shell = key.CreateSubKey(@"shell\open\command")) {
                shell.SetValue("", string.Format("\"{0}\" run \"%1\"", exePath));
            }
        }
    }
}
"@

Set-Content -Path ".\Setup.cs" -Value $CSharpCode -Encoding UTF8

# 3. Compile the C# Code and Embed the ZIP as a Resource
Write-Host "`n[2/2] Compiling Setup.exe..." -ForegroundColor Yellow
$csc = "C:\Windows\Microsoft.NET\Framework64\v4.0.30319\csc.exe"
$outExe = ".\NuxSetup.exe"

& $csc /nologo /out:$outExe /res:$ZipPath,nux_release.zip /r:System.IO.Compression.dll /r:System.IO.Compression.FileSystem.dll .\Setup.cs

if ($LASTEXITCODE -eq 0) {
    Write-Host "`nSuccessfully created NuxSetup.exe!" -ForegroundColor Green
    Remove-Item ".\Setup.cs" -Force
} else {
    Write-Host "`nFailed to compile Setup.exe" -ForegroundColor Red
    exit 1
}
