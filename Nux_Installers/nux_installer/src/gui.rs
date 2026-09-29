use eframe::egui;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

#[cfg(windows)]
use crate::platform::{PlatformInstaller, WindowsInstaller};

#[cfg(not(windows))]
use crate::platform::{PlatformInstaller, UnixInstaller};

#[cfg(windows)]
fn get_platform_installer() -> Box<dyn PlatformInstaller> {
    Box::new(WindowsInstaller)
}

#[cfg(not(windows))]
fn get_platform_installer() -> Box<dyn PlatformInstaller> {
    Box::new(UnixInstaller)
}

#[derive(PartialEq)]
enum ViewState {
    Welcome,
    Installing,
    Installed,
    Error(String),
}

pub struct InstallerApp {
    state: ViewState,
    install_dir: PathBuf,
    is_installed: bool,
    add_to_path: bool,
    progress: Arc<Mutex<f32>>,
    status_msg: Arc<Mutex<String>>,
}

impl Default for InstallerApp {
    fn default() -> Self {
        let platform = get_platform_installer();
        let install_dir = platform.get_install_dir();
        let is_installed = platform.detect_existing().is_some();
        Self {
            state: ViewState::Welcome,
            install_dir,
            is_installed,
            add_to_path: true,
            progress: Arc::new(Mutex::new(0.0)),
            status_msg: Arc::new(Mutex::new("Ready".to_string())),
        }
    }
}

impl eframe::App for InstallerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Install image loaders if using images
        egui_extras::install_image_loaders(ctx);

        // Sidebar for logo (like Python installer)
        egui::SidePanel::left("sidebar")
            .exact_width(150.0)
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(240, 240, 240)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    // Load the logo
                    let logo = egui::include_image!("../../../logo/logo.png");
                    ui.add(egui::Image::new(logo).max_width(120.0));
                    ui.add_space(20.0);
                    ui.label(egui::RichText::new("Nux").size(24.0).color(egui::Color32::DARK_GRAY).strong());
                    ui.label(egui::RichText::new("v0.1.0").color(egui::Color32::GRAY));
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Check state transition
            if self.state == ViewState::Installing {
                let status = self.status_msg.lock().unwrap().clone();
                if status == "DONE" {
                    self.state = ViewState::Installed;
                }
            }

            ui.add_space(10.0);

            match &self.state {
                ViewState::Welcome => self.show_welcome(ui, ctx),
                ViewState::Installing => self.show_installing(ui),
                ViewState::Installed => self.show_installed(ui),
                ViewState::Error(msg) => {
                    let msg_clone = msg.clone();
                    self.show_error(ui, &msg_clone);
                }
            }
        });
    }
}

impl InstallerApp {
    fn show_welcome(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading(egui::RichText::new("Install Nux").size(24.0).strong());
        ui.add_space(20.0);

        if self.is_installed {
            ui.label(egui::RichText::new("An existing installation of Nux was detected on this system.").color(egui::Color32::DARK_GRAY));
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                if ui.button(egui::RichText::new("Repair / Upgrade").size(16.0)).clicked() {
                    self.start_installation(ctx.clone());
                }
                ui.add_space(10.0);
                if ui.button(egui::RichText::new("Uninstall").size(16.0)).clicked() {
                    self.start_uninstallation(ctx.clone());
                }
            });
        } else {
            // Main install button
            if ui.add(egui::Button::new(
                egui::RichText::new("Install Now")
                    .size(20.0)
                    .strong()
            ).fill(egui::Color32::from_rgb(0, 120, 215))).clicked() {
                self.start_installation(ctx.clone());
            }
            ui.label(egui::RichText::new(format!("Includes Nux compiler, standard library, and tools.\nInstalls to: {}", self.install_dir.display())).small().color(egui::Color32::GRAY));
            
            ui.add_space(20.0);
            
            // Checkboxes
            ui.checkbox(&mut self.add_to_path, "Add Nux to PATH");
        }
    }

    fn show_installing(&mut self, ui: &mut egui::Ui) {
        ui.heading(egui::RichText::new("Setup Progress").size(24.0).strong());
        ui.add_space(20.0);
        
        let msg = self.status_msg.lock().unwrap().clone();
        ui.label(egui::RichText::new(msg).color(egui::Color32::DARK_GRAY));
        
        ui.add_space(10.0);
        let p = *self.progress.lock().unwrap();
        ui.add(egui::ProgressBar::new(p).show_percentage().animate(true));
    }

    fn show_installed(&mut self, ui: &mut egui::Ui) {
        ui.heading(egui::RichText::new("Setup was successful").size(24.0).strong());
        ui.add_space(20.0);
        ui.label("Nux has been successfully installed on your computer.");
        ui.add_space(30.0);
        if ui.button(egui::RichText::new("Close").size(16.0)).clicked() {
            std::process::exit(0);
        }
    }

    fn show_error(&mut self, ui: &mut egui::Ui, msg: &str) {
        ui.heading(egui::RichText::new("Setup failed").size(24.0).strong().color(egui::Color32::RED));
        ui.add_space(20.0);
        ui.label("An error occurred during installation:");
        ui.label(egui::RichText::new(msg).monospace());
        ui.add_space(30.0);
        if ui.button(egui::RichText::new("Close").size(16.0)).clicked() {
            std::process::exit(1);
        }
    }

    fn start_installation(&mut self, ctx: egui::Context) {
        self.state = ViewState::Installing;
        let progress = Arc::clone(&self.progress);
        let status = Arc::clone(&self.status_msg);
        let install_dir = self.install_dir.clone();
        let add_to_path = self.add_to_path;

        thread::spawn(move || {
            *status.lock().unwrap() = "Extracting files...".to_string();
            
            let exe_path = match std::env::current_exe() {
                Ok(p) => p,
                Err(e) => {
                    *status.lock().unwrap() = format!("Failed to get current executable path: {}", e);
                    return;
                }
            };
            
            let file = match std::fs::File::open(exe_path) {
                Ok(f) => f,
                Err(e) => {
                    *status.lock().unwrap() = format!("Failed to open current executable: {}", e);
                    return;
                }
            };
            
            let mut archive = match zip::ZipArchive::new(file) {
                Ok(a) => a,
                Err(e) => {
                    *status.lock().unwrap() = format!("Failed to open payload archive: {}", e);
                    return;
                }
            };
            
            if let Err(e) = crate::extract::extract_zip_archive(&mut archive, &install_dir) {
                *status.lock().unwrap() = format!("Extraction failed: {}", e);
                // Also wait a bit before turning to error?
                // For simplicity, we just leave the message on screen if it fails.
                return;
            }

            *progress.lock().unwrap() = 0.5;
            *status.lock().unwrap() = "Configuring environment...".to_string();
            let platform = get_platform_installer();
            
            if add_to_path {
                if let Err(e) = platform.install_env_vars(&install_dir) {
                    *status.lock().unwrap() = format!("Failed to set env vars: {}", e);
                    return;
                }
            }

            *progress.lock().unwrap() = 0.8;
            *status.lock().unwrap() = "Setting up file associations...".to_string();
            
            if let Err(e) = platform.create_file_associations(&install_dir) {
                *status.lock().unwrap() = format!("Failed to associate files: {}", e);
                return;
            }

            *progress.lock().unwrap() = 1.0;
            *status.lock().unwrap() = "DONE".to_string();
            ctx.request_repaint();
        });
    }

    fn start_uninstallation(&mut self, ctx: egui::Context) {
        self.state = ViewState::Installing;
        let progress = Arc::clone(&self.progress);
        let status = Arc::clone(&self.status_msg);
        let install_dir = self.install_dir.clone();

        thread::spawn(move || {
            *progress.lock().unwrap() = 0.2;
            *status.lock().unwrap() = "Removing files...".to_string();
            let _ = std::fs::remove_dir_all(&install_dir);

            *progress.lock().unwrap() = 0.6;
            *status.lock().unwrap() = "Cleaning up environment...".to_string();
            let platform = get_platform_installer();
            let _ = platform.remove_env_vars(&install_dir);
            let _ = platform.remove_file_associations();

            *progress.lock().unwrap() = 1.0;
            *status.lock().unwrap() = "DONE".to_string();
            ctx.request_repaint();
        });
    }
}
