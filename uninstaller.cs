using System;
using System.Drawing;
using System.IO;
using System.Windows.Forms;

namespace NuxUninstaller
{
    public class UninstallerForm : Form
    {
        private Button btnUninstall;
        private Button btnCancel;
        private TextBox txtStatus;
        private Label lblTitle;
        private Label lblSub;
        private Label lblInstallDir;
        private ProgressBar progressBar;
        private CheckBox chkKeepData;

        private string nuxDir;
        private string binDir;

        public UninstallerForm()
        {
            string localAppData = Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData);
            nuxDir = Path.Combine(localAppData, "Nux");
            binDir = Path.Combine(nuxDir, "bin");

            this.Text = "Nux Language Uninstaller";
            this.Size = new Size(480, 360);
            this.StartPosition = FormStartPosition.CenterScreen;
            this.FormBorderStyle = FormBorderStyle.FixedDialog;
            this.MaximizeBox = false;
            this.BackColor = Color.FromArgb(24, 24, 28);

            lblTitle = new Label();
            lblTitle.Text = "Uninstall Nux";
            lblTitle.Font = new Font("Segoe UI", 20f, FontStyle.Bold);
            lblTitle.ForeColor = Color.FromArgb(255, 100, 100);
            lblTitle.AutoSize = true;
            lblTitle.Location = new Point(20, 18);
            this.Controls.Add(lblTitle);

            lblSub = new Label();
            lblSub.Text = "This will remove the Nux Language from your system.";
            lblSub.Font = new Font("Segoe UI", 9f);
            lblSub.ForeColor = Color.FromArgb(180, 180, 185);
            lblSub.AutoSize = true;
            lblSub.Location = new Point(22, 54);
            this.Controls.Add(lblSub);

            bool isInstalled = Directory.Exists(nuxDir);
            string installStatus = isInstalled
                ? string.Format("Installed at: {0}", nuxDir)
                : "Nux does not appear to be installed in the default location.";

            lblInstallDir = new Label();
            lblInstallDir.Text = installStatus;
            lblInstallDir.Font = new Font("Segoe UI", 8f);
            lblInstallDir.ForeColor = isInstalled ? Color.FromArgb(120, 120, 130) : Color.FromArgb(255, 160, 80);
            lblInstallDir.AutoSize = true;
            lblInstallDir.Location = new Point(22, 74);
            this.Controls.Add(lblInstallDir);

            // Separator
            Panel sep = new Panel();
            sep.BackColor = Color.FromArgb(50, 50, 60);
            sep.Location = new Point(0, 96);
            sep.Size = new Size(480, 1);
            this.Controls.Add(sep);

            chkKeepData = new CheckBox();
            chkKeepData.Text = "Keep user data files (.nux programs) in install folder";
            chkKeepData.Font = new Font("Segoe UI", 9f);
            chkKeepData.ForeColor = Color.FromArgb(220, 220, 225);
            chkKeepData.AutoSize = true;
            chkKeepData.Location = new Point(22, 110);
            chkKeepData.Checked = false;
            chkKeepData.BackColor = Color.Transparent;
            this.Controls.Add(chkKeepData);

            txtStatus = new TextBox();
            txtStatus.Multiline = true;
            txtStatus.ReadOnly = true;
            txtStatus.ScrollBars = ScrollBars.Vertical;
            txtStatus.Location = new Point(22, 145);
            txtStatus.Size = new Size(432, 110);
            txtStatus.BackColor = Color.FromArgb(18, 18, 22);
            txtStatus.ForeColor = Color.FromArgb(220, 160, 160);
            txtStatus.Font = new Font("Consolas", 8.5f);
            txtStatus.BorderStyle = BorderStyle.FixedSingle;
            this.Controls.Add(txtStatus);

            progressBar = new ProgressBar();
            progressBar.Location = new Point(22, 262);
            progressBar.Size = new Size(432, 14);
            progressBar.Style = ProgressBarStyle.Continuous;
            progressBar.Visible = false;
            this.Controls.Add(progressBar);

            btnUninstall = new Button();
            btnUninstall.Text = "Uninstall";
            btnUninstall.Location = new Point(22, 285);
            btnUninstall.Size = new Size(120, 34);
            btnUninstall.FlatStyle = FlatStyle.Flat;
            btnUninstall.BackColor = Color.FromArgb(180, 40, 40);
            btnUninstall.ForeColor = Color.White;
            btnUninstall.Font = new Font("Segoe UI", 9f, FontStyle.Bold);
            btnUninstall.FlatAppearance.BorderSize = 0;
            btnUninstall.Enabled = isInstalled;
            btnUninstall.Click += BtnUninstall_Click;
            this.Controls.Add(btnUninstall);

            btnCancel = new Button();
            btnCancel.Text = "Cancel";
            btnCancel.Location = new Point(350, 285);
            btnCancel.Size = new Size(104, 34);
            btnCancel.FlatStyle = FlatStyle.Flat;
            btnCancel.BackColor = Color.FromArgb(50, 50, 58);
            btnCancel.ForeColor = Color.FromArgb(200, 200, 205);
            btnCancel.Font = new Font("Segoe UI", 9f);
            btnCancel.FlatAppearance.BorderSize = 0;
            btnCancel.Click += (s, e) => { this.Close(); };
            this.Controls.Add(btnCancel);
        }

        private void Log(string message)
        {
            txtStatus.AppendText(message + "\r\n");
            Application.DoEvents();
        }

        private void BtnUninstall_Click(object sender, EventArgs e)
        {
            // Confirm
            DialogResult confirm = MessageBox.Show(
                "Are you sure you want to uninstall Nux Language?\nThis action cannot be undone.",
                "Confirm Uninstall",
                MessageBoxButtons.YesNo,
                MessageBoxIcon.Warning
            );

            if (confirm != DialogResult.Yes)
                return;

            btnUninstall.Enabled = false;
            chkKeepData.Enabled = false;
            btnCancel.Enabled = false;
            progressBar.Visible = true;
            progressBar.Style = ProgressBarStyle.Marquee;

            try
            {
                // Step 1: Remove from PATH
                Log("> Removing Nux from PATH...");
                string userPath = Environment.GetEnvironmentVariable("PATH", EnvironmentVariableTarget.User) ?? "";
                if (userPath.Contains(binDir))
                {
                    string newPath = userPath.Replace(";" + binDir, "").Replace(binDir + ";", "").Replace(binDir, "");
                    Environment.SetEnvironmentVariable("PATH", newPath, EnvironmentVariableTarget.User);
                    Log("> Removed from PATH successfully.");
                }
                else
                {
                    Log("> Nux was not found in PATH (already clean).");
                }

                // Step 2: Remove files
                if (Directory.Exists(nuxDir))
                {
                    if (chkKeepData.Checked)
                    {
                        // Only remove bin/ and src/ but keep .nux user files
                        Log("> Keeping user .nux files...");
                        string[] subDirs = { "bin", "nux", "nux_oleg" };
                        foreach (string sub in subDirs)
                        {
                            string subPath = Path.Combine(nuxDir, sub);
                            if (Directory.Exists(subPath))
                            {
                                Directory.Delete(subPath, true);
                                Log(string.Format(">   Removed: {0}\\", sub));
                            }
                        }
                        // Remove README
                        string readme = Path.Combine(nuxDir, "README.txt");
                        if (File.Exists(readme)) File.Delete(readme);
                    }
                    else
                    {
                        // Full removal
                        Log(string.Format("> Removing {0}...", nuxDir));
                        Directory.Delete(nuxDir, true);
                        Log("> Installation directory removed.");
                    }
                }
                else
                {
                    Log("> Installation directory not found (already removed?).");
                }

                Log("");
                Log("=== Nux has been uninstalled. ===");
                Log("Thank you for using Nux!");
            }
            catch (Exception ex)
            {
                Log(string.Format("[ERROR] {0}", ex.Message));
            }

            progressBar.Style = ProgressBarStyle.Continuous;
            progressBar.Value = 100;
            btnCancel.Text = "Close";
            btnCancel.BackColor = Color.FromArgb(0, 120, 215);
            btnCancel.ForeColor = Color.White;
            btnCancel.Enabled = true;
        }
    }

    static class Program
    {
        [STAThread]
        static void Main()
        {
            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
            Application.Run(new UninstallerForm());
        }
    }
}
