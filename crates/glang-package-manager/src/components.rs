use dirs::{download_dir, home_dir};
use glang_logging::{log_header, log_message};
use reqwest::blocking::get;
use std::{
    env,
    fs::{self, File},
    io::{Write, copy, stdin, stdout},
    process::{Command, exit},
};

/// Updates the glang binary and components
///
/// ```rust
/// use glang_package_manager::update_self;
///
/// update_self();
/// ```
///
/// This will download the platform specific binary and components.
/// - On Windows: download the .exe installer and run it.
/// - On MacOS: extract the .zip into the `Applications/` and overwrite the GeorgeLanguage folder
pub fn update_self() {
    if cfg!(target_os = "windows") {
        log_header("Downloading glang-latest for Windows");

        let download_path = download_dir()
            .expect("Unable to get user downloads directory")
            .with_file_name("glang-installer.exe");

        {
            log_message("Retrieving installer data");

            let mut resp = get(
            "https://github.com/george-language/glang/releases/latest/download/GeorgeLanguage+windows_setup.exe",
        ).expect("Unable to retrieve installer data");

            log_message("Creating installer file");

            let mut file =
                File::create(&download_path).expect("Unable to create glang installer file");

            log_message("Writing data");

            copy(&mut resp, &mut file).expect("Unable to write installer file");
        }

        log_message("Launching glang installer");

        let _ = Command::new(&download_path)
            .spawn()
            .expect("Unable to launch installer");

        exit(0);
    } else if cfg!(target_os = "macos") {
        log_header("Downloading glang-latest for macOS");

        let download_path = download_dir()
            .expect("Unable to get user downloads directory")
            .with_file_name("glang-binary.pkg");

        {
            log_message("Retrieving package data");

            let mut resp = get(
            "https://github.com/george-language/glang/releases/latest/download/GeorgeLanguage+macos_setup.pkg",
        ).expect("Unable to download macos content");

            log_message("Creating package file");

            let mut file =
                File::create(&download_path).expect("Unable to create glang package file");

            log_message("Writing data");

            copy(&mut resp, &mut file).expect("Unable to write package file");
        }

        log_message("Launching glang installer");

        let _ = Command::new("open")
            .arg(&download_path)
            .spawn()
            .expect("Unable to launch installer");

        exit(0);
    }
}

/// Uninstalls the glang binary and components (including all installed kennels)
///
/// ```rust
/// use glang_package_manager::uninstall_self;
///
/// uninstall_self();
/// ```
pub fn uninstall_self() {
    log_header("Uninstalling glang and all components");

    let mut confirmation = String::new();

    print!("    -> Are you sure you want to continue? [Y/n]: ");
    let _ = stdout().flush();

    stdin()
        .read_line(&mut confirmation)
        .expect("Input text was invalid");

    if !(confirmation.trim().to_lowercase() == "y") {
        log_message("Cancelling uninstallation");

        return;
    }

    log_message("Removing '.glang' directory");

    fs::remove_dir_all(
        home_dir()
            .expect("Unable to access user home directory")
            .join(".glang"),
    )
    .expect("Unable to remove '.glang' directory");

    if cfg!(target_os = "windows") {
        log_message("Running uninstaller script");

        let uninstaller = env::current_exe()
            .expect("Unable to retrieve current executable")
            .parent()
            .expect("Unable to retrieve current parent folder")
            .join("unins000.exe"); // inno setup script

        let _ = Command::new(&uninstaller)
            .spawn()
            .expect("Unable to spawn uninstaller script");

        exit(0)
    } else if cfg!(target_os = "macos") {
        log_message("Removing glang binary");

        let cmd = r#"
            sleep 2;
            rm -f /usr/local/bin/glang;
            rm -rf /Library/GeorgeLanguage;
        "#;

        log_message("Removing glang library");

        let _ = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .spawn()
            .expect("Unable to remove glang binary and library");

        exit(0);
    }
}
