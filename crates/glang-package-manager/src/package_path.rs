use std::env;
use std::path::PathBuf;

pub fn get_package_path() -> PathBuf {
    env::var("GLANG_PKG")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("kennels"))
}
