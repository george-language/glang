use clap::{Parser, Subcommand};
use glang;
use std::env;
use std::path::Path;

const VERSION: &str = "2.0";

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

    match (cli.command, cli.file) {
        (Some(Commands::New { name }), _) => {
            glang::new_project(Path::new(&name));
        }
        (Some(Commands::Init), _) => {
            glang::new_project(Path::new("."));
        }
        (None, Some(file)) => {
            let error = glang::run(&file, None);

            if let Some(err) = error {
                println!("{}", err);
            }
        }
        (None, None) => {
            glang::launch_repl(VERSION);
        }
    }
}
