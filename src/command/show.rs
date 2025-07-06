use simply_colored::*;

pub fn show_help() {
    println!("{BOLD}Usage: {ITALIC}glang [command] ... [options]{RESET}\n");
    println!("  glang:                    use glang interactively in the terminal");
    println!("  glang <file>:             run a '.glang' file");
    println!("  glang new <project_name>: create a glang project");
    println!("  glang init:               initialize a glang project in the current directory");
    println!("  glang --version:          print the glang version");
    println!("  glang --v:                print the glang version");
    println!("  glang --help:             see this message again");
    println!("");
}

pub fn show_version(version: &str) {
    println!("George Language {version}");
}
