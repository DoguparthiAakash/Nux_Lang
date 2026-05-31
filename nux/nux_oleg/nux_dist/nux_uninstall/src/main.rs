use std::env;
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

fn main() {
    println!("====================================");
    println!(" Nux Programming Language Uninstaller");
    println!("====================================");

    let local_appdata = env::var("LOCALAPPDATA").expect("LOCALAPPDATA not found");
    let install_dir = format!("{}\\_Nux", local_appdata).replace("_", ""); // Just building string safely

    println!("\n[1/3] Removing from PATH...");
    remove_from_path(&install_dir);

    println!("\n[2/3] Removing File Associations...");
    remove_file_associations();

    println!("\n[3/3] Notifying Windows Explorer...");
    broadcast_settings_change();

    println!("\n====================================");
    println!(" Registry Cleanup Successful.       ");
    println!(" Nux directory will be deleted now. ");
    println!("====================================");

    // Spawn a detached cmd process that waits 2 seconds and then deletes the directory
    // This allows the uninstaller to exit so its own file is no longer locked by Windows
    let cmd = format!(
        "timeout /t 2 /nobreak > NUL & rmdir /s /q \"{}\"",
        install_dir
    );

    Command::new("cmd")
        .args(&["/C", &cmd])
        .spawn()
        .expect("Failed to start deletion process");
}

fn remove_from_path(install_dir: &str) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(env_key) = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE) {
        if let Ok(current_path) = env_key.get_value::<String, _>("PATH") {
            if current_path.contains(install_dir) {
                // Remove the directory from PATH
                let new_path = current_path
                    .split(';')
                    .filter(|p| !p.eq_ignore_ascii_case(install_dir))
                    .collect::<Vec<_>>()
                    .join(";");

                let _ = env_key.set_value("PATH", &new_path);
                println!("      Removed {} from User PATH.", install_dir);
            } else {
                println!("      Directory not found in PATH.");
            }
        }
    }
}

fn remove_file_associations() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(classes) = hkcu.open_subkey_with_flags("Software\\Classes", KEY_READ | KEY_WRITE) {
        let _ = classes.delete_subkey_all(".nux");
        let _ = classes.delete_subkey_all(".nuxc");
        let _ = classes.delete_subkey_all("Nux.File");
        println!("      Removed .nux and .nuxc associations.");
    }
}

fn broadcast_settings_change() {
    use std::ptr;
    #[link(name = "user32")]
    extern "system" {
        fn SendMessageTimeoutA(
            hWnd: *mut std::ffi::c_void,
            Msg: u32,
            wParam: usize,
            lParam: usize,
            fuFlags: u32,
            uTimeout: u32,
            lpdwResult: *mut usize,
        ) -> isize;
    }

    const HWND_BROADCAST: *mut std::ffi::c_void = 0xffff as *mut _;
    const WM_SETTINGCHANGE: u32 = 0x001A;
    const SMTO_ABORTIFHUNG: u32 = 0x0002;

    let mut result = 0;
    unsafe {
        SendMessageTimeoutA(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            0,
            "Environment\0".as_ptr() as usize,
            SMTO_ABORTIFHUNG,
            5000,
            &mut result,
        );
    }
}
