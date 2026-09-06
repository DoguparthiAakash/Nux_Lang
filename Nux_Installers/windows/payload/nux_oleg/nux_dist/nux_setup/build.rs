use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root_dir = PathBuf::from(manifest_dir).parent().unwrap().to_path_buf();
    let payload_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("payload");

    // Recreate payload dir
    if payload_dir.exists() {
        fs::remove_dir_all(&payload_dir).unwrap();
    }
    fs::create_dir_all(&payload_dir).unwrap();

    // Copy nux.exe
    let nux_exe = root_dir.join("target").join("release").join("nux.exe");
    if nux_exe.exists() {
        fs::copy(&nux_exe, payload_dir.join("nux.exe")).expect("Failed to copy nux.exe");
    } else {
        panic!("nux.exe not found! Run cargo build --release in the root first.");
    }

    // Copy logo.png
    let logo_png = root_dir.join("logo.png");
    if logo_png.exists() {
        fs::copy(&logo_png, payload_dir.join("logo.png")).expect("Failed to copy logo.png");
    } else {
        panic!("logo.png not found!");
    }

    // Copy nux_file_icon.ico
    let nux_file_icon = root_dir.join("nux_file_icon.ico");
    if nux_file_icon.exists() {
        fs::copy(&nux_file_icon, payload_dir.join("nux_file_icon.ico")).expect("Failed to copy nux_file_icon.ico");
    } else {
        panic!("nux_file_icon.ico not found!");
    }

    // Copy nuxc_file_icon.ico
    let nuxc_file_icon = root_dir.join("nuxc_file_icon.ico");
    if nuxc_file_icon.exists() {
        fs::copy(&nuxc_file_icon, payload_dir.join("nuxc_file_icon.ico")).expect("Failed to copy nuxc_file_icon.ico");
    } else {
        panic!("nuxc_file_icon.ico not found!");
    }

    // Copy nux_remove.exe
    let uninstall_exe = root_dir.join("nux_uninstall").join("target").join("release").join("nux_remove.exe");
    if uninstall_exe.exists() {
        fs::copy(&uninstall_exe, payload_dir.join("nux_remove.exe")).expect("Failed to copy nux_remove.exe");
    } else {
        panic!("nux_remove.exe not found! Run cargo build --release in nux_uninstall first.");
    }

    // Copy lib folder
    let lib_dir_src = root_dir.join("lib");
    let lib_dir_dest = payload_dir.join("lib");
    fs::create_dir_all(&lib_dir_dest).unwrap();
    
    if lib_dir_src.exists() && lib_dir_src.is_dir() {
        for entry in fs::read_dir(lib_dir_src).unwrap() {
            let entry = entry.unwrap();
            if entry.path().is_file() {
                fs::copy(entry.path(), lib_dir_dest.join(entry.file_name())).unwrap();
            }
        }
    } else {
        panic!("lib directory not found!");
    }

    // Re-run if these change
    println!("cargo:rerun-if-changed=../target/release/nux.exe");
    println!("cargo:rerun-if-changed=../logo.png");
    println!("cargo:rerun-if-changed=../nux_file_icon.ico");
    println!("cargo:rerun-if-changed=../nuxc_file_icon.ico");
    println!("cargo:rerun-if-changed=../nux_uninstall/target/release/nux_remove.exe");
    println!("cargo:rerun-if-changed=../lib");
}
