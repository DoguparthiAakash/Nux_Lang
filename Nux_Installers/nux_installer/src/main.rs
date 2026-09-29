#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod extract;
mod platform;
mod gui;

use gui::InstallerApp;

fn main() -> eframe::Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap_or(());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_title("Nux Setup"),
        ..Default::default()
    };

    eframe::run_native(
        "Nux Installer",
        options,
        Box::new(|_cc| Ok(Box::new(InstallerApp::default()))),
    )
}
