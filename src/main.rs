use clap::{Parser, Subcommand};
use std::env;
use std::path::Path;

const VERSION: &str = "2.5";

#[derive(Parser)]
#[command(name = "glang", version = VERSION, about = "The George Programming Language")]
struct Cli {
    file: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create a glang project")]
    New { name: String },
    #[command(about = "Initialize a glang project in the current directory")]
    Init,
    #[command(about = "Install a glang kennel from the internet")]
    Install { name: String },
    #[command(about = "Remove an installed glang kennel")]
    Remove { name: String },
    #[command(about = "Update an installed glang kennel to the latest version")]
    Update { name: String },
}

fn main() {
    unsafe {
        let std_path = env::current_exe()
            .expect("Unable to retrieve current exe")
            .parent()
            .unwrap()
            .join("library")
            .to_string_lossy()
            .replace("\\", "/")
            .replace("target/debug/", "")
            .replace("target/release/", "");

        let pkg_path = env::current_exe()
            .expect("Unable to retrieve current exe")
            .parent()
            .unwrap()
            .join("kennels")
            .to_string_lossy()
            .replace("\\", "/")
            .replace("target/debug/", "")
            .replace("target/release/", "");

        env::set_var("GLANG_STD", &std_path);
        env::set_var("GLANG_PKG", &pkg_path);
    }

    glang::create_package_dir();

    let cli = Cli::parse();

    match (cli.command, cli.file) {
        (Some(Commands::New { name }), _) => {
            glang::new_project(Path::new(&name), false);
        }
        (Some(Commands::Init), _) => {
            glang::new_project(Path::new("."), true);
        }
        (Some(Commands::Install { name }), _) => {
            glang::add_package(&name);
        }
        (Some(Commands::Remove { name }), _) => {
            glang::remove_package(&name);
        }
        (Some(Commands::Update { name }), _) => {
            if glang::is_package_installed(&name) {
                glang::remove_package(&name);
                glang::add_package(&name);
            } else {
                glang::log_package_status(&name, false);
            }
        }
        (None, Some(file)) => {
            let error = glang::run(&file, None);

            if let Some(err) = error {
                println!("{err}");
            }
        }
        (None, None) => {
            glang::launch_repl(VERSION);
        }
    }
}
