extern crate simply_colored;

pub mod logs;
pub mod packages;
pub mod paths;

pub use logs::{log_error, log_header, log_message, log_package_status};
pub use packages::{add_package, create_package_dir, remove_package, update_package};
pub use paths::get_package_path;
