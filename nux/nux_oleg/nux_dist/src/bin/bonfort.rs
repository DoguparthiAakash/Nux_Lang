use std::env;
use std::process;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

use nux::package_manager::{self, InstallTarget};
use nux::bonfort_config::{BonfortConfig, BonfortLock, LockedPackage, PackageMetadata};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    // Parse global flag
    let mut target = InstallTarget::auto_detect();
    let mut args_to_process = Vec::new();
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--global" || args[i] == "-g" {
            target = InstallTarget::Global;
        } else {
            args_to_process.push(args[i].clone());
        }
        i += 1;
    }

    if args_to_process.is_empty() {
        print_usage();
        process::exit(1);
    }

    let command = args_to_process[0].as_str();
    let cmd_args = &args_to_process;

    match command {
        "install" => {
            if cmd_args.len() < 2 {
                // Auto-discovery: Check for Bonfort.toml first
                if std::path::Path::new("Bonfort.toml").exists() && target != InstallTarget::Global {
                    package_manager::install_from_config(target);
                    return;
                }
                // Fallback: Check for requirements.txt (deprecated)
                if std::path::Path::new("requirements.txt").exists() {
                    println!("Warning: requirements.txt is deprecated, please use Bonfort.toml");
                    package_manager::install_from_file("requirements.txt", target);
                    return;
                }
                eprintln!("Usage: bonfort install <package> [version] [--global]");
                eprintln!("       bonfort install -r <file> [--global]");
                eprintln!("       bonfort install (Auto-discovers Bonfort.toml)");
                process::exit(1);
            }
            if cmd_args[1] == "-r" || cmd_args[1] == "--requirement" {
                if cmd_args.len() < 3 {
                    eprintln!("Usage: bonfort install -r <file> [--global]");
                    process::exit(1);
                }
                package_manager::install_from_file(&cmd_args[2], target);
            } else {
                let version = if cmd_args.len() >= 3 { &cmd_args[2] } else { "*" };
                package_manager::install(&cmd_args[1], version, target);
            }
        }
        "remove" | "uninstall" => {
            if cmd_args.len() < 2 {
                eprintln!("Usage: bonfort remove <package> [--global]");
                process::exit(1);
            }
            cmd_remove(&cmd_args[1], target);
        }
        "list" => {
            package_manager::list(target);
        }
        "init" => {
            let project_name = if cmd_args.len() >= 2 {
                &cmd_args[1]
            } else {
                "my-nux-project"
            };
            cmd_init(project_name);
        }
        "search" => {
            if cmd_args.len() < 2 {
                eprintln!("Usage: bonfort search <query>");
                process::exit(1);
            }
            cmd_search(&cmd_args[1]);
        }
        "update" => {
            if cmd_args.len() >= 2 {
                package_manager::update_package(&cmd_args[1], target);
            } else {
                package_manager::update_all(target);
            }
        }
        "add" => {
            if cmd_args.len() < 2 {
                eprintln!("Usage: bonfort add <package> [version]");
                process::exit(1);
            }
            let package = &cmd_args[1];
            let version = if cmd_args.len() >= 3 {
                cmd_args[2].clone()
            } else {
                "*".to_string()
            };
            cmd_add(package, &version);
        }
        "show" | "info" => {
            if cmd_args.len() < 2 {
                eprintln!("Usage: bonfort show <package>");
                process::exit(1);
            }
            cmd_show(&cmd_args[1]);
        }
        "help" | "--help" | "-h" => {
            if cmd_args.len() >= 2 {
                print_command_help(&cmd_args[1]);
            } else {
                print_usage();
            }
        }
        "version" | "--version" | "-v" => {
            println!("bonfort 0.2.0");
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage();
            process::exit(1);
        }
    }
}

fn cmd_init(project_name: &str) {
    println!("Creating new Nux project: {}", project_name);
    
    // Create project directory
    if Path::new(project_name).exists() {
        eprintln!("Error: Directory '{}' already exists", project_name);
        process::exit(1);
    }
    
    fs::create_dir(project_name).expect("Failed to create project directory");
    
    // Create src directory
    let src_dir = Path::new(project_name).join("src");
    fs::create_dir(&src_dir).expect("Failed to create src directory");
    
    // Create main.nux
    let main_nux = src_dir.join("main.nux");
    let main_content = r#"// Welcome to Nux!
// This is your main entry point.

import io

fn main() {
    io.println("Hello, Nux!")
}
"#;
    fs::write(&main_nux, main_content).expect("Failed to create main.nux");
    
    // Create Bonfort.toml using the config struct
    let config = BonfortConfig {
        package: PackageMetadata {
            name: project_name.to_string(),
            version: "0.1.0".to_string(),
            authors: vec!["Your Name <you@example.com>".to_string()],
            edition: "2024".to_string(),
        },
        dependencies: HashMap::new(),
        dev_dependencies: HashMap::new(),
    };
    
    let bonfort_toml = Path::new(project_name).join("Bonfort.toml");
    config.to_file(&bonfort_toml).expect("Failed to create Bonfort.toml");
    
    // Create README.md
    let readme = Path::new(project_name).join("README.md");
    let readme_content = format!("# {}\n\nA Nux project.\n\n## Getting Started\n\n```bash\ncd {}\nbonfort install  # Install dependencies\nnux run src/main.nux\n```\n\n## Adding Dependencies\n\n```bash\nbonfort add gui\nbonfort add ai ^0.3\n```\n", project_name, project_name);
    fs::write(&readme, readme_content).expect("Failed to create README.md");
    
    println!("✓ Created project structure:");
    println!("  {}/", project_name);
    println!("  ├── Bonfort.toml");
    println!("  ├── README.md");
    println!("  └── src/");
    println!("      └── main.nux");
    println!();
    println!("Next steps:");
    println!("  cd {}", project_name);
    println!("  bonfort add <package>  # Add dependencies");
    println!("  bonfort install        # Install dependencies");
    println!("  nux run src/main.nux");
}

fn cmd_add(package: &str, version: &str) {
    let bonfort_toml = Path::new("Bonfort.toml");
    
    if !bonfort_toml.exists() {
        eprintln!("Error: Bonfort.toml not found");
        eprintln!("Run 'bonfort init' to create a new project");
        process::exit(1);
    }
    
    let mut config = BonfortConfig::from_file(bonfort_toml)
        .unwrap_or_else(|e| {
            eprintln!("Error reading Bonfort.toml: {}", e);
            process::exit(1);
        });
    
    config.add_dependency(package.to_string(), version.to_string());
    
    config.to_file(bonfort_toml)
        .unwrap_or_else(|e| {
            eprintln!("Error writing Bonfort.toml: {}", e);
            process::exit(1);
        });
    
    println!("✓ Added {} = \"{}\" to dependencies", package, version);
    println!("Run 'bonfort install' to install the package");
}
fn cmd_remove(package_name: &str, target: InstallTarget) {
    let bonfort_toml = Path::new("Bonfort.toml");
    
    // 1. Remove from local/venv/global lib/external
    package_manager::remove(package_name, target);
    
    // 2. Remove from Bonfort.toml if it exists and we are not in global mode
    if target != InstallTarget::Global && bonfort_toml.exists() {
        if let Ok(mut config) = BonfortConfig::from_file(bonfort_toml) {
            if config.remove_dependency(package_name) {
                if let Err(e) = config.to_file(bonfort_toml) {
                    eprintln!("Warning: Could not update Bonfort.toml: {}", e);
                } else {
                    println!("✓ Removed {} from Bonfort.toml", package_name);
                }
            }
        }
    }
}

fn cmd_search(query: &str) {
    println!("Searching for packages matching '{}'...", query);
    println!();
    
    // Mock registry search
    let packages = vec![
        ("std", "Standard library for Nux", "1.0.0"),
        ("gui", "GUI framework for Nux applications", "0.5.0"),
        ("ai", "AI and machine learning library", "0.3.0"),
        ("game", "Game development framework", "0.2.0"),
    ];
    
    let mut found = false;
    for (name, desc, version) in packages {
        if name.contains(query) || desc.to_lowercase().contains(&query.to_lowercase()) {
            println!("  {} ({})", name, version);
            println!("    {}", desc);
            println!();
            found = true;
        }
    }
    
    if !found {
        println!("No packages found matching '{}'", query);
    }
}

fn cmd_show(package_name: &str) {
    println!("Package: {}", package_name);
    println!();
    
    // Mock package info
    match package_name {
        "std" => {
            println!("Version: 1.0.0");
            println!("Description: Standard library for Nux");
            println!("Homepage: https://github.com/nux-lang/std");
            println!("License: MIT");
            println!();
            println!("Dependencies: None");
        }
        "gui" => {
            println!("Version: 0.5.0");
            println!("Description: GUI framework for Nux applications");
            println!("Homepage: https://github.com/nux-lang/gui");
            println!("License: MIT");
            println!();
            println!("Dependencies:");
            println!("  - std >= 1.0");
        }
        "ai" => {
            println!("Version: 0.3.0");
            println!("Description: AI and machine learning library");
            println!("Homepage: https://github.com/nux-lang/ai");
            println!("License: MIT");
            println!();
            println!("Dependencies:");
            println!("  - std >= 1.0");
        }
        _ => {
            eprintln!("Package '{}' not found in registry", package_name);
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Bonfort - Nux Package Manager");
    println!();
    println!("USAGE:");
    println!("    bonfort <COMMAND> [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    --global, -g          Process packages system-wide (root)");
    println!();
    println!("COMMANDS:");
    println!("    init [name]           Create a new Nux project");
    println!("    add <pkg> [version]   Add a dependency to Bonfort.toml");
    println!("    install <package>     Install a package");
    println!("    install -r <file>     Install from requirements file");
    println!("    install               Auto-install from Bonfort.toml");
    println!("    remove <package>      Remove a package");
    println!("    update [package]      Update package(s) to latest version");
    println!("    list                  List installed packages");
    println!("    search <query>        Search for packages");
    println!("    show <package>        Show package information");
    println!("    help [command]        Show help for a command");
    println!("    version               Show bonfort version");
    println!();
    println!("Run 'bonfort help <command>' for more information on a command.");
}

fn print_command_help(command: &str) {
    match command {
        "init" => {
            println!("bonfort init - Create a new Nux project");
            println!();
            println!("USAGE:");
            println!("    bonfort init [project-name]");
            println!();
            println!("DESCRIPTION:");
            println!("    Creates a new Nux project with the following structure:");
            println!("    - nux.toml (project metadata)");
            println!("    - requirements.txt (dependencies)");
            println!("    - src/main.nux (entry point)");
            println!("    - README.md");
            println!();
            println!("EXAMPLES:");
            println!("    bonfort init my-app");
            println!("    bonfort init");
        }
        "install" => {
            println!("bonfort install - Install packages");
            println!();
            println!("USAGE:");
            println!("    bonfort install <package>");
            println!("    bonfort install -r <file>");
            println!("    bonfort install");
            println!();
            println!("EXAMPLES:");
            println!("    bonfort install gui");
            println!("    bonfort install -r requirements.txt");
            println!("    bonfort install  # Auto-discovers requirements.txt");
        }
        "search" => {
            println!("bonfort search - Search for packages");
            println!();
            println!("USAGE:");
            println!("    bonfort search <query>");
            println!();
            println!("EXAMPLES:");
            println!("    bonfort search gui");
            println!("    bonfort search machine learning");
        }
        "update" => {
            println!("bonfort update - Update packages");
            println!();
            println!("USAGE:");
            println!("    bonfort update [package]");
            println!();
            println!("DESCRIPTION:");
            println!("    Updates the specified package to the latest version.");
            println!("    If no package is specified, updates all packages.");
            println!();
            println!("EXAMPLES:");
            println!("    bonfort update gui");
            println!("    bonfort update  # Updates all packages");
        }
        "show" => {
            println!("bonfort show - Show package information");
            println!();
            println!("USAGE:");
            println!("    bonfort show <package>");
            println!();
            println!("EXAMPLES:");
            println!("    bonfort show gui");
        }
        _ => {
            eprintln!("No help available for '{}'", command);
            print_usage();
        }
    }
}
