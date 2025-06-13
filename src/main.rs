use glang::run;
use std::env;
use std::io::{Write, stdin, stdout};

const VERSION: &str = "2.0";

fn main() {
    let mut args = env::args();

    if args.len() > 1 {
        args.next(); // skip .exe

        if let Some(first_arg) = args.next() {
            if first_arg.ends_with(".glang") {
                let (result, error) = run(first_arg.as_str(), None);

                if error.is_some() {
                    println!("{}", error.unwrap());
                }

                if result.is_some() {
                    println!("{}", result.unwrap());
                }
            }
        }
    } else {
        println!(
            "George Language {VERSION}: Platform {}-{}",
            env::consts::OS,
            env::consts::ARCH
        );
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

            let (result, error) = run("<stdin>", Some(code));

            if error.is_some() {
                println!("{}", error.unwrap());
                continue;
            }

            if result.is_some() {
                println!("{}", result.unwrap());
            }
        }
    }
}
