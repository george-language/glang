pub mod glang_self;
pub mod manager;
pub mod package_path;

pub use glang_self::{uninstall_self, update_self};
pub use manager::{add_package, create_package_dir, remove_package, update_package};
pub use package_path::get_package_path;
