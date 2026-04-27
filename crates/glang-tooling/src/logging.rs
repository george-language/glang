use simply_colored::*;

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
