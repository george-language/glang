use glang::{command, run};
use std::env;
use std::path::Path;

const VERSION: &str = "2.0-alpha";

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

    let mut args = env::args();

    if args.len() > 1 {
        args.next();

        if let Some(first_arg) = args.next() {
            let first_arg = first_arg.as_str();

            match first_arg {
                "new" => {
                    if let Some(second_arg) = args.next() {
                        command::new_project(Path::new(second_arg.as_str()));
                    }
                }
                "init" => {
                    command::new_project(Path::new("."));
                }
                "--version" | "-v" => {
                    command::show_version(VERSION);
                }
                "--help" => {
                    command::show_help();
                }
                _ => {
                    if first_arg.ends_with(".glang") {
                        let error = run(first_arg, None);

                        if error.is_some() {
                            println!("{}", error.unwrap());
                        }
                    } else {
                        println!("Unrecognized command '{}'", first_arg);
                    }
                }
            }
        }
    } else {
        command::show_version(VERSION);
        command::launch_repl();
    }
}
