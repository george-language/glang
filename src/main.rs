use std::env;

const VERSION: &str = "2.0";

fn main() {
    let arguments = env::args();

    if arguments.len() > 0 {
        for arg in arguments {
            if arg.ends_with(".glang") {}
        }
    } else {
        println!("George Language {VERSION} Platform ");
        println!("Type 'exit()' to exit");

        // GLang terminal
        // loop {}
    }
}
