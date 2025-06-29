use glang::run;
use std::io::{Write, stdin, stdout};
use std::path::Path;
use std::{env, fs};

const VERSION: &str = "2.0";

fn main() {
    let mut args = env::args();

    if args.len() > 1 {
        args.next(); // skip .exe

        if let Some(first_arg) = args.next() {
            if first_arg == "new" {
                if let Some(second_arg) = args.next() {
                    let dir_name = Path::new(second_arg.as_str());

                    fs::create_dir(&dir_name).expect("Cannot create directory (invalid name)");
                    let _ = fs::create_dir(&dir_name.join("src"));

                    let _ = fs::write(
                        &dir_name.join("main.glang"),
                        "func main() {\n    bark(\"Hello world!\");\n}\n\nmain();",
                    );
                    let _ = fs::write(
                        &dir_name.join("README.md"),
                        "# Welcome to GLang!\nTo get started, see our documentation [here](https://sites.google.com/view/george-lang/documentation).",
                    );
                }
            } else if first_arg == "init" {
                let dir_name = Path::new(".");

                let _ = fs::create_dir(&dir_name.join("src"));

                let _ = fs::write(
                    &dir_name.join("main.glang"),
                    "func main() {\n    bark(\"Hello world!\");\n}\n\nmain();",
                );
                let _ = fs::write(
                    &dir_name.join("README.md"),
                    "# Welcome to GLang!\nTo get started, see our documentation [here](https://sites.google.com/view/george-lang/documentation).",
                );
            } else if first_arg == "--version" || first_arg == "-v" {
                println!("George Language {VERSION}");
            } else if first_arg.ends_with(".glang") {
                let error = run(first_arg.as_str(), None);

                if error.is_some() {
                    println!("{}", error.unwrap());
                }
            } else {
                println!("Unrecognized command '{}'", first_arg);
            }
        }
    } else {
        println!("George Language {VERSION}");
        println!("Type '/exit' to exit");

        loop {
            let mut code = String::new();

            print!(">>> ");
            let _ = stdout().flush();

            stdin()
                .read_line(&mut code)
                .expect("Did not enter a valid string");

            if code.trim().starts_with("/exit") {
                break;
            }

            let error = run("<stdin>", Some(code));

            if error.is_some() {
                println!("{}", error.unwrap());
                continue;
            }
        }
    }
}
