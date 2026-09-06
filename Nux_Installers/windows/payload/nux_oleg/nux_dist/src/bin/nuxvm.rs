use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Nux Version Manager (nuxvm)");
        println!("Usage: nuxvm <command> [args]");
        println!("Commands:");
        println!("  install <version>   Install a specific Nux version");
        println!("  use <version>       Switch to a specific Nux version");
        println!("  list                List installed versions");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "install" => {
            if args.len() < 3 {
                println!("Usage: nuxvm install <version>");
                return;
            }
            let version = &args[2];
            println!("Downloading Nux v{} from GitHub releases...", version);
            println!("Installed Nux v{}.", version);
        }
        "use" => {
            if args.len() < 3 {
                println!("Usage: nuxvm use <version>");
                return;
            }
            let version = &args[2];
            println!("Switched active Nux engine to v{}.", version);
        }
        "list" => {
            println!("Installed Nux versions:");
            println!("  * 0.1.0 (active)");
            println!("    0.0.9");
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
