extern crate simply_colored;

pub mod logs;
pub mod manager;
pub mod package_path;

pub use logs::{log_error, log_header, log_message, log_package_status};
pub use manager::{add_package, create_package_dir, remove_package, update_package};
pub use package_path::get_package_path;
