use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Nux Package Manager (nuxpm)");
        println!("Usage: nuxpm <command> [args]");
        println!("Commands:");
        println!("  init      Initialize a new nux project");
        println!("  install   Install a package");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "init" => {
            let nux_json = r#"{
  "name": "my_nux_project",
  "version": "0.1.0",
  "dependencies": {}
}"#;
            fs::write("nux.json", nux_json).unwrap();
            println!("Initialized new Nux project (created nux.json).");
        }
        "install" => {
            if args.len() < 3 {
                println!("Usage: nuxpm install <package_name>");
                return;
            }
            let package_name = &args[2];
            println!("Fetching package '{}' from Nux registry...", package_name);
            
            // Dummy implementation of package installation
            let mut lib_path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
            lib_path.push(".nux_modules");
            fs::create_dir_all(&lib_path).unwrap();
            
            let mut pkg_path = lib_path.clone();
            pkg_path.push(format!("{}.nux", package_name));
            
            fs::write(&pkg_path, format!("// Package {}\nfn {}_hello() {{ print(\"Hello from {}\"); }}", package_name, package_name, package_name)).unwrap();
            println!("Installed package '{}' to {}", package_name, pkg_path.display());
            println!("Ensure NUX_LIB_PATH is set to '{}' to use this package.", lib_path.display());
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
