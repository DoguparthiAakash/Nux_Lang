using System;
using System.Drawing;
using System.IO;
using System.IO.Compression;
using System.Reflection;
using System.Windows.Forms;

namespace NuxInstaller
{
    public class InstallerForm : Form
    {
        private CheckBox chkPath;
        private Button btnInstall;
        private Button btnClose;
        private TextBox txtStatus;
        private Label lblTitle;
        private Label lblSub;
        private Label lblVersion;
        private ProgressBar progressBar;

        public InstallerForm()
        {
            this.Text = "Nux Language Setup";
            this.Size = new Size(480, 380);
            this.StartPosition = FormStartPosition.CenterScreen;
            this.FormBorderStyle = FormBorderStyle.FixedDialog;
            this.MaximizeBox = false;
            this.BackColor = Color.FromArgb(24, 24, 28);

            lblTitle = new Label();
            lblTitle.Text = "Nux Language";
            lblTitle.Font = new Font("Segoe UI", 20f, FontStyle.Bold);
            lblTitle.ForeColor = Color.FromArgb(110, 200, 255);
            lblTitle.AutoSize = true;
            lblTitle.Location = new Point(20, 18);
            this.Controls.Add(lblTitle);

            lblSub = new Label();
            lblSub.Text = "The native, high-performance systems language.";
            lblSub.Font = new Font("Segoe UI", 9f);
            lblSub.ForeColor = Color.FromArgb(180, 180, 185);
            lblSub.AutoSize = true;
            lblSub.Location = new Point(22, 54);
            this.Controls.Add(lblSub);

            lblVersion = new Label();
            lblVersion.Text = "Version 0.1.0-alpha  |  Windows x86-64";
            lblVersion.Font = new Font("Segoe UI", 8f);
            lblVersion.ForeColor = Color.FromArgb(120, 120, 130);
            lblVersion.AutoSize = true;
            lblVersion.Location = new Point(22, 74);
            this.Controls.Add(lblVersion);

            // Separator
            Panel sep = new Panel();
            sep.BackColor = Color.FromArgb(50, 50, 60);
            sep.Location = new Point(0, 98);
            sep.Size = new Size(480, 1);
            this.Controls.Add(sep);

            chkPath = new CheckBox();
            chkPath.Text = "Add Nux to PATH (recommended)";
            chkPath.Font = new Font("Segoe UI", 9f);
            chkPath.ForeColor = Color.FromArgb(220, 220, 225);
            chkPath.AutoSize = true;
            chkPath.Location = new Point(22, 112);
            chkPath.Checked = true;
            chkPath.BackColor = Color.Transparent;
            this.Controls.Add(chkPath);

            Label lblInstDir = new Label();
            lblInstDir.Text = string.Format("Install location:  {0}\\Nux", 
                Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData));
            lblInstDir.Font = new Font("Segoe UI", 8f);
            lblInstDir.ForeColor = Color.FromArgb(120, 120, 130);
            lblInstDir.AutoSize = true;
            lblInstDir.Location = new Point(22, 140);
            this.Controls.Add(lblInstDir);

            txtStatus = new TextBox();
            txtStatus.Multiline = true;
            txtStatus.ReadOnly = true;
            txtStatus.ScrollBars = ScrollBars.Vertical;
            txtStatus.Location = new Point(22, 165);
            txtStatus.Size = new Size(432, 110);
            txtStatus.BackColor = Color.FromArgb(18, 18, 22);
            txtStatus.ForeColor = Color.FromArgb(180, 220, 180);
            txtStatus.Font = new Font("Consolas", 8.5f);
            txtStatus.BorderStyle = BorderStyle.FixedSingle;
            this.Controls.Add(txtStatus);

            progressBar = new ProgressBar();
            progressBar.Location = new Point(22, 282);
            progressBar.Size = new Size(432, 14);
            progressBar.Style = ProgressBarStyle.Continuous;
            progressBar.Visible = false;
            this.Controls.Add(progressBar);

            btnInstall = new Button();
            btnInstall.Text = "Install Now";
            btnInstall.Location = new Point(22, 305);
            btnInstall.Size = new Size(120, 34);
            btnInstall.FlatStyle = FlatStyle.Flat;
            btnInstall.BackColor = Color.FromArgb(0, 120, 215);
            btnInstall.ForeColor = Color.White;
            btnInstall.Font = new Font("Segoe UI", 9f, FontStyle.Bold);
            btnInstall.FlatAppearance.BorderSize = 0;
            btnInstall.Click += BtnInstall_Click;
            this.Controls.Add(btnInstall);

            btnClose = new Button();
            btnClose.Text = "Cancel";
            btnClose.Location = new Point(350, 305);
            btnClose.Size = new Size(104, 34);
            btnClose.FlatStyle = FlatStyle.Flat;
            btnClose.BackColor = Color.FromArgb(50, 50, 58);
            btnClose.ForeColor = Color.FromArgb(200, 200, 205);
            btnClose.Font = new Font("Segoe UI", 9f);
            btnClose.FlatAppearance.BorderSize = 0;
            btnClose.Click += (s, e) => { this.Close(); };
            this.Controls.Add(btnClose);
        }

        private void Log(string message)
        {
            txtStatus.AppendText(message + "\r\n");
            Application.DoEvents();
        }

        private void BtnInstall_Click(object sender, EventArgs e)
        {
            btnInstall.Enabled = false;
            chkPath.Enabled = false;
            btnClose.Enabled = false;
            progressBar.Visible = true;
            progressBar.Style = ProgressBarStyle.Marquee;

            string localAppData = Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData);
            string nuxDir = Path.Combine(localAppData, "Nux");
            string nuxBin = Path.Combine(nuxDir, "bin", "nux.exe");

            try
            {
                // Step 1: Load the embedded zip from resources
                Log("> Loading embedded Nux runtime package...");
                Assembly assembly = Assembly.GetExecutingAssembly();
                byte[] zipBytes = null;

                using (Stream stream = assembly.GetManifestResourceStream("Nux_Source.zip"))
                {
                    if (stream == null)
                    {
                        Log("[ERROR] Embedded Nux_Source.zip not found in installer!");
                        goto Finish;
                    }
                    zipBytes = new byte[stream.Length];
                    stream.Read(zipBytes, 0, zipBytes.Length);
                }
                Log(string.Format("> Package loaded ({0} KB).", zipBytes.Length / 1024));

                // Step 2: Remove old installation if present
                if (Directory.Exists(nuxDir))
                {
                    Log("> Removing previous installation...");
                    Directory.Delete(nuxDir, true);
                }
                Directory.CreateDirectory(nuxDir);
                Log(string.Format("> Install directory: {0}", nuxDir));

                // Step 3: Extract zip
                Log("> Extracting files...");
                using (MemoryStream ms = new MemoryStream(zipBytes))
                using (ZipArchive archive = new ZipArchive(ms, ZipArchiveMode.Read))
                {
                    foreach (ZipArchiveEntry entry in archive.Entries)
                    {
                        string destPath = Path.Combine(nuxDir, entry.FullName);
                        string destDir = Path.GetDirectoryName(destPath);
                        if (!Directory.Exists(destDir))
                            Directory.CreateDirectory(destDir);

                        if (!string.IsNullOrEmpty(entry.Name))
                            entry.ExtractToFile(destPath, true);
                    }
                }
                Log("> Extraction complete.");

                // Extract bundled uninstaller into the install dir
                Log("> Installing uninstaller...");
                string uninstallerDest = Path.Combine(nuxDir, "NuxUninstall.exe");
                using (Stream us = assembly.GetManifestResourceStream("NuxUninstall.exe"))
                {
                    if (us != null)
                    {
                        using (FileStream fs = new FileStream(uninstallerDest, FileMode.Create, FileAccess.Write))
                            us.CopyTo(fs);
                        Log("> Uninstaller placed at: " + uninstallerDest);
                    }
                }

                // Verify binary
                if (!File.Exists(nuxBin))
                {
                    Log("[ERROR] nux.exe not found after extraction. Installation may be corrupt.");
                    goto Finish;
                }
                Log("> nux.exe verified.");

                // Step 4: Update PATH
                if (chkPath.Checked)
                {
                    Log("> Updating PATH environment variable...");
                    string binDir = Path.Combine(nuxDir, "bin");
                    string userPath = Environment.GetEnvironmentVariable("PATH", EnvironmentVariableTarget.User) ?? "";
                    if (!userPath.Contains(binDir))
                    {
                        if (!userPath.EndsWith(";")) userPath += ";";
                        Environment.SetEnvironmentVariable("PATH", userPath + binDir, EnvironmentVariableTarget.User);
                        Log("> PATH updated. Restart your terminal to use 'nux'.");
                    }
                    else
                    {
                        Log("> Nux is already in PATH.");
                    }
                }

                Log("");
                Log("=== Installation complete! ===");
                Log("Run: nux run yourprogram.nux");
            }
            catch (Exception ex)
            {
                Log(string.Format("[ERROR] {0}", ex.Message));
            }

            Finish:
            progressBar.Style = ProgressBarStyle.Continuous;
            progressBar.Value = 100;
            btnClose.Text = "Close";
            btnClose.BackColor = Color.FromArgb(0, 120, 215);
            btnClose.ForeColor = Color.White;
            btnClose.Enabled = true;
        }
    }

    static class Program
    {
        [STAThread]
        static void Main()
        {
            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
            Application.Run(new InstallerForm());
        }
    }
}
