import re

with open('src/main.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# Replace print_version
new_print_version = """fn print_version() {
    let logo_color1 = (0, 200, 255);
    let logo_color2 = (150, 50, 255);
    
    println!();
    println!("  {} {} {}", 
        "-".truecolor(logo_color1.0, logo_color1.1, logo_color1.2).bold(),
        "Nux".truecolor(logo_color2.0, logo_color2.1, logo_color2.2).bold(),
        env!("CARGO_PKG_VERSION").truecolor(150, 150, 150)
    );
    println!("  {}", "A High-Performance AI Programming Language".truecolor(100, 100, 100).italic());
}"""

content = re.sub(r'fn print_version\(\) \{.*?\n\}', new_print_version, content, flags=re.DOTALL)

# Replace print_help
new_print_help = """fn print_help() {
    print_version();
    println!();
    
    let box_color = (60, 60, 70);
    let category_color = (255, 100, 150);
    let cmd_color = (0, 200, 255);
    let desc_color = (180, 180, 180);
    
    println!("  {}", "o Usage".truecolor(255, 255, 255).bold());
    println!("  {} {} {}\n", "- ".truecolor(box_color.0, box_color.1, box_color.2), "nux".truecolor(cmd_color.0, cmd_color.1, cmd_color.2).bold(), "<command> [args]".truecolor(100, 100, 100));
    
    println!("  {}", "o Commands".truecolor(255, 255, 255).bold());
    
    let cmds = vec![
        ("o  Project", vec![
            ("new <name>", "Create a new Nux workspace"),
            ("build", "Compile project to bytecode"),
            ("run [file]", "Execute a script or project"),
            ("test", "Run test suite"),
            ("clean", "Remove build artifacts"),
        ]),
        ("o  Ecosystem", vec![
            ("pkg <cmd>", "Manage packages (install, remove, list)"),
            ("venv <cmd>", "Manage isolated virtual environments"),
        ]),
        ("o  Advanced", vec![
            ("compile <file>", "Compile to .nuxc executable"),
            ("build-ext <f>", "Compile .cux native extension"),
            ("repl", "Start the interactive console"),
        ]),
    ];
    
    for (i, (category, group)) in cmds.iter().enumerate() {
        println!("  {} {}", "-".truecolor(box_color.0, box_color.1, box_color.2), category.truecolor(category_color.0, category_color.1, category_color.2).bold());
        for (j, (cmd, desc)) in group.iter().enumerate() {
            let is_last_group = i == cmds.len() - 1;
            let prefix = if is_last_group { " " } else { "" };
            let sub_prefix = if j == group.len() - 1 { "-" } else { "-" };
            println!("  {}   {} {:<16} {}", 
                prefix.truecolor(box_color.0, box_color.1, box_color.2),
                sub_prefix.truecolor(box_color.0, box_color.1, box_color.2),
                cmd.truecolor(cmd_color.0, cmd_color.1, cmd_color.2).bold(),
                desc.truecolor(desc_color.0, desc_color.1, desc_color.2)
            );
        }
        if i != cmds.len() - 1 {
            println!("  {}", "".truecolor(box_color.0, box_color.1, box_color.2));
        }
    }
    println!();
}"""

content = re.sub(r'fn print_help\(\) \{.*?(?=\nfn cmd_new)', new_print_help, content, flags=re.DOTALL)

with open('src/main.rs', 'w', encoding='utf-8') as f:
    f.write(content)
print("Updated main.rs successfully!")
