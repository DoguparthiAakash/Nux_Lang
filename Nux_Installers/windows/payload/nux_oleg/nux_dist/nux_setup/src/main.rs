use rust_embed::RustEmbed;
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;

#[derive(RustEmbed)]
#[folder = "payload/"]
struct Payload;

fn main() {
    println!("====================================");
    println!(" Nux Programming Language Installer ");
    println!("====================================");
    
    let local_appdata = env::var("LOCALAPPDATA").expect("LOCALAPPDATA environment variable not found");
    let install_dir = PathBuf::from(&local_appdata).join("Nux");
    
    println!("\n[1/3] Extracting files to {}...", install_dir.display());
    
    // Create directory
    if !install_dir.exists() {
        fs::create_dir_all(&install_dir).expect("Failed to create installation directory");
    }
    
    // Unpack files
    for file in Payload::iter() {
        let file_path = file.as_ref();
        let dest_path = install_dir.join(file_path);
        
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent directory");
        }
        
        let embedded_file = Payload::get(file_path).unwrap();
        let mut f = fs::File::create(&dest_path).expect("Failed to create file");
        f.write_all(&embedded_file.data).expect("Failed to write file");
        
        println!("      Extracting: {}", file_path);
    }
    
    // 2. Setup PATH
    println!("\n[2/3] Updating PATH environment variable...");
    update_path(&install_dir.to_string_lossy());
    
    // 3. File Associations
    println!("\n[3/3] Setting up File Associations...");
    setup_file_associations(&install_dir.to_string_lossy());
    
    // 4. Notify Windows Explorer
    broadcast_settings_change();
    
    println!("\n====================================");
    println!(" Installation Successful!           ");
    println!(" You can now use the 'nux' command. ");
    println!("====================================");
    
    println!("\nPress Enter to exit...");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
}

fn update_path(install_dir: &str) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env_key = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)
        .expect("Failed to open Environment registry key");
    
    let current_path: String = env_key.get_value("PATH").unwrap_or_default();
    
    if !current_path.contains(install_dir) {
        let new_path = if current_path.is_empty() {
            install_dir.to_string()
        } else if current_path.ends_with(';') {
            format!("{}{}", current_path, install_dir)
        } else {
            format!("{};{}", current_path, install_dir)
        };
        
        env_key.set_value("PATH", &new_path).expect("Failed to update PATH");
        println!("      Added {} to User PATH.", install_dir);
    } else {
        println!("      PATH already contains Nux directory.");
    }
}

fn setup_file_associations(install_dir: &str) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let classes = hkcu.create_subkey_with_flags("Software\\Classes", KEY_READ | KEY_WRITE)
        .expect("Failed to open Software\\Classes").0;
        
    // Associate .nux
    let (nux_ext, _) = classes.create_subkey(".nux").unwrap();
    nux_ext.set_value("", &"Nux.File").unwrap();
    
    // Associate .nuxc
    let (nuxc_ext, _) = classes.create_subkey(".nuxc").unwrap();
    nuxc_ext.set_value("", &"Nux.Compiled").unwrap();
    
    // Create Nux.File ProgID
    let (prog_id, _) = classes.create_subkey("Nux.File").unwrap();
    prog_id.set_value("", &"Nux Source File").unwrap();
    let (icon_key, _) = prog_id.create_subkey("DefaultIcon").unwrap();
    let icon_path = format!("{}\\nux_file_icon.ico", install_dir);
    icon_key.set_value("", &icon_path).unwrap();
    let (cmd_key, _) = prog_id.create_subkey("shell\\open\\command").unwrap();
    let open_cmd = format!("\"{}\" run \"%1\"", format!("{}\\nux.exe", install_dir));
    cmd_key.set_value("", &open_cmd).unwrap();

    // Create Nux.Compiled ProgID
    let (comp_prog_id, _) = classes.create_subkey("Nux.Compiled").unwrap();
    comp_prog_id.set_value("", &"Nux Compiled Binary").unwrap();
    let (comp_icon_key, _) = comp_prog_id.create_subkey("DefaultIcon").unwrap();
    let comp_icon_path = format!("{}\\nuxc_file_icon.ico", install_dir);
    comp_icon_key.set_value("", &comp_icon_path).unwrap();
    let (comp_cmd_key, _) = comp_prog_id.create_subkey("shell\\open\\command").unwrap();
    let comp_open_cmd = format!("\"{}\" run \"%1\"", format!("{}\\nux.exe", install_dir));
    comp_cmd_key.set_value("", &comp_open_cmd).unwrap();
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
