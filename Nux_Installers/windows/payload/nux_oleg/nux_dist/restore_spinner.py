import re

with open('src/main.rs', 'r', encoding='utf-8') as f:
    content = f.read()

create_spinner_code = """
fn create_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        .template("{spinner:.bright_magenta} {msg}").unwrap());
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

fn cmd_new"""

content = content.replace('\nfn cmd_new', create_spinner_code, 1)

with open('src/main.rs', 'w', encoding='utf-8') as f:
    f.write(content)
print("Restored create_spinner!")
