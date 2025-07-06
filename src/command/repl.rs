use crate::run;
use std::io::{Write, stdin, stdout};

pub fn launch_repl() {
    println!("Type '/exit' to exit");

    loop {
        let mut code = String::new();

        print!(">>> ");
        let _ = stdout().flush();

        stdin()
            .read_line(&mut code)
            .expect("Did not enter a valid string");

        if code.trim() == "/exit" {
            break;
        }

        let error = run("<stdin>", Some(code));

        if error.is_some() {
            println!("{}", error.unwrap());

            continue;
        }
    }
}
