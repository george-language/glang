use clap::{Parser, Subcommand};
use glang;
use std::env;
use std::path::Path;

const VERSION: &str = "2.0-beta";

#[derive(Parser)]
#[command(name = "glang", version = VERSION, about = "The George Programming Language")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create a glang project")]
    New { name: String },
    #[command(about = "Initialize a glang project in the current directory")]
    Init,
    #[command(about = "Run a '.glang' source file")]
    Run { file: String },
}

fn main() {
    unsafe {
        let path = env::current_exe()
            .expect("Unable to retrieve current exe")
            .parent()
            .unwrap()
            .join("library")
            .to_string_lossy()
            .replace("\\", "/")
            .replace("target/debug/", "")
            .replace("target/release/", "");

        env::set_var("GLANG_STD", &path);
    }

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::New { name }) => {
            glang::command::new_project(Path::new(&name));
        }
        Some(Commands::Init) => {
            glang::command::new_project(Path::new("."));
        }
        Some(Commands::Run { file }) => {
            let error = glang::run(&file, None);

            if let Some(err) = error {
                println!("{}", err);
            }
        }
        None => {
            glang::command::show_version(VERSION);
            glang::command::launch_repl();
        }
    }
}
