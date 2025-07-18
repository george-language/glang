use simply_colored::*;

pub fn package_not_installed(package: &str) {
    println!("🤔 Kennel '{}' not installed", &package);
    println!(
        "💡 To install, try {BOLD}`glang install {}`{RESET}",
        &package
    );
}
