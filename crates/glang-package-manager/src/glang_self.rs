use dirs::{cache_dir, home_dir};
use glang_logging::{log_header, log_message};
use reqwest::blocking::get;
use std::{
    fs::{self, File},
    io::{Cursor, Write, copy},
    path::PathBuf,
    process::{Command, exit},
};

/// Updates the glang binary and components
///
/// ```rust
/// update_self()
/// ```
///
/// This will download the platform specific binary and components.
/// - On Windows: download the .exe installer and run it.
/// - On MacOS: extract the .zip into the `Applications/` and overwrite the GeorgeLanguage folder
pub fn update_self() {
    if cfg!(target_os = "windows") {
        log_header("Downloading glang-latest for Windows");

        let download_path = cache_dir()
            .expect("Unable to get user cache dir")
            .with_file_name("glang-installer.exe");

        {
            log_message("Retrieving installer data");

            let mut resp = get(
            "https://github.com/george-language/glang/releases/latest/download/GeorgeLanguage+windows_setup.exe",
        ).expect("Unable to download windows content");

            log_message("Creating temporary installer file");

            let mut file =
                File::create(&download_path).expect("Unable to create glang installer file");

            log_message("Writing installer data to temporary installer file");

            copy(&mut resp, &mut file).expect("Unable to write installer file");
        }

        log_message("Launching glang installer");

        let _ = Command::new(&download_path)
            .spawn()
            .expect("Unable to launch installer");

        exit(0);
    } else if cfg!(target_os = "macos") {
        log_header("Downloading glang-latest for MacOS");

        let download_path = cache_dir()
            .expect("Unable to get user cache dir")
            .with_file_name("glang-binary.zip");

        {
            log_message("Retrieving zip data");

            let mut resp = get(
            "https://github.com/george-language/glang/releases/latest/download/GeorgeLanguage+macos_setup.zip",
        ).expect("Unable to download macos content");

            log_message("Creating temporary zip file");

            let mut file = File::create(&download_path).expect("Unable to create glang zip file");

            log_message("Writing component data to temporary zip file");

            copy(&mut resp, &mut file).expect("Unable to write zip file");
        }

        log_message("Extracting zip data into '/Applications/GeorgeLanguage'");

        let cmd = format!(
            "sleep 2 && unzip -o {} -d /Applications/GeorgeLanguage",
            download_path.to_string_lossy().to_string()
        );

        Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .spawn()
            .expect("Unable to spawn unzip process");

        exit(0);
    }
}

/// Uninstalls the glang binary and components (including all installed kennels)
///
/// ```rust
/// uninstall_self()
/// ```
pub fn uninstall_self() {}
