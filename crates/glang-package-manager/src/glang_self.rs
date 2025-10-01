use dirs::cache_dir;
use glang_logging::log_header;
use reqwest::blocking::get;
use std::{
    fs::File,
    io::{Write, copy},
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
        log_header("Downloading Windows Installer");

        let download_path = cache_dir()
            .expect("Unable to get user cache dir")
            .with_file_name("glang-installer.exe");

        {
            let mut resp = get(
            "https://github.com/george-language/glang/releases/latest/download/GeorgeLanguage+windows_setup.exe",
        ).expect("Unable to download windows content");
            let mut file =
                File::create(&download_path).expect("Unable to create glang installer file");
            copy(&mut resp, &mut file).expect("Unable to write installer file");
        }

        let _ = Command::new(&download_path)
            .spawn()
            .expect("Unable to launch installer");

        exit(0);
    } else if cfg!(target_os = "macos") {
    }
}

/// Uninstalls the glang binary and components (including all installed kennels)
///
/// ```rust
/// uninstall_self()
/// ```
pub fn uninstall_self() {}
