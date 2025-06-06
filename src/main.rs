use glang::run;
use std::env;
use std::io::{Write, stdin, stdout};

const VERSION: &str = "2.0";

fn main() {
    let arguments = env::args();

    if arguments.len() > 1 {
        for arg in arguments {
            if arg.ends_with(".glang") {}
        }
    } else {
        println!(
            "George Language {VERSION} Platform {}-{}",
            env::consts::OS,
            env::consts::ARCH
        );
        println!("Type 'exit()' to exit");

        loop {
            let mut code = String::new();

            print!(">>> ");
            let _ = stdout().flush();

            stdin()
                .read_line(&mut code)
                .expect("Did not enter a valid string");

            if code.trim().starts_with("exit()") {
                break;
            }

            let (_, error) = run("<stdin>", code);

            if !error.is_empty() {
                println!("{error}");
            }
        }
    }
}
