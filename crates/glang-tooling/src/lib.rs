mod components;
mod logging;
mod manager;

pub use components::{install_library, uninstall_self, update_self};
pub use logging::{log_error, log_header, log_message, log_package_status, wait_for_confirmation};
pub use manager::{
    add_package, create_package_folder, create_project_folder, get_latest_version, read_registry,
    remove_package, write_package_file, write_registry,
};
