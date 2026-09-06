import re

with open('src/bin/bonfort.rs', 'r', encoding='utf-8') as f:
    content = f.read()

new_print_usage = """fn print_usage() {
    let logo_color1 = (255, 165, 0);
    let logo_color2 = (255, 100, 0);
    
    println!();
    println!("  {} {} {}", 
        "-".truecolor(logo_color1.0, logo_color1.1, logo_color1.2).bold(),
        "Bonfort".truecolor(logo_color2.0, logo_color2.1, logo_color2.2).bold(),
        "v0.1.0".truecolor(150, 150, 150)
    );
    println!("  {}", "The elegant package manager for the Nux programming language".truecolor(100, 100, 100).italic());
    println!();
    
    let box_color = (60, 60, 70);
    let category_color = (255, 100, 150);
    let cmd_color = (0, 200, 255);
    let desc_color = (180, 180, 180);
    
    println!("  {}", "o Usage".truecolor(255, 255, 255).bold());
    println!("  {} {} {}\n", "- ".truecolor(box_color.0, box_color.1, box_color.2), "bonfort".truecolor(cmd_color.0, cmd_color.1, cmd_color.2).bold(), "<command> [options]".truecolor(100, 100, 100));
    
    println!("  {}", "o Options".truecolor(255, 255, 255).bold());
    println!("  {}   {:<16} {}\n", "- ".truecolor(box_color.0, box_color.1, box_color.2), "--global, -g".truecolor(cmd_color.0, cmd_color.1, cmd_color.2).bold(), "Process packages system-wide (root)".truecolor(desc_color.0, desc_color.1, desc_color.2));
    
    println!("  {}", "o Commands".truecolor(255, 255, 255).bold());
    
    let cmds = vec![
        ("o  Project", vec![
            ("init [name]", "Create a new Nux project"),
            ("add <pkg>", "Add a dependency to Bonfort.toml"),
        ]),
        ("o  Packages", vec![
            ("install [pkg]", "Install packages"),
            ("remove <pkg>", "Remove a package"),
            ("update [pkg]", "Update package(s)"),
        ]),
        ("o  Information", vec![
            ("list", "List installed packages"),
            ("search <query>", "Search for packages"),
            ("show <pkg>", "Show package information"),
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

content = re.sub(r'fn print_usage\(\) \{.*?(?=\nfn print_command_help)', new_print_usage, content, flags=re.DOTALL)

with open('src/bin/bonfort.rs', 'w', encoding='utf-8') as f:
    f.write(content)
print("Updated bonfort.rs successfully!")
