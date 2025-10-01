use simply_colored::*;

pub fn log_header(msg: &str) {
    println!("  {msg}");
}

pub fn log_message(msg: &str) {
    println!("    -> {msg}");
}

pub fn log_error(msg: &str) {
    println!("{DIM_RED}{BOLD}error:{RESET} {msg}");
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
        "To {}, try {BOLD}`glang {} {}`{RESET}",
        if installed { "update" } else { "install" },
        if installed { "update" } else { "install" },
        &package
    ));
}
