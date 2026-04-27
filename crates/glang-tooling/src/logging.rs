use simply_colored::*;
use std::io::{Write, stdin, stdout};

pub fn log_header(msg: &str) {
    println!("  {msg}");
}

pub fn log_message(msg: &str) {
    println!("    -> {msg}");
}

pub fn log_error(msg: &str) {
    println!("{RED}{msg}{RESET}")
}

pub fn log_package_status(package: &str, installed: bool) {
    log_message(&format!(
        "Kennel '{}' is {}",
        package,
        if installed {
            "already installed"
        } else {
            "not installed"
        }
    ));
    log_message(&format!(
        "To install, download the '.kennel' file, then run {BOLD}glang install {}.kennel{RESET}",
        &package
    ));
}

pub fn wait_for_confirmation(msg: &str) -> bool {
    let mut confirmation = String::new();

    print!("    -> {msg} [Y/n]:");
    let _ = stdout().flush();

    stdin()
        .read_line(&mut confirmation)
        .expect("Input text was invalid");

    let confirmation = confirmation.trim().to_lowercase();

    if (confirmation == "y") || (confirmation == "") {
        return true;
    }

    false
}
