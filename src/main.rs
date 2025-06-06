use std::env;
use std::fs;
use std::path::Path;

const VERSION: &str = "2.0";

fn main() {
    let arguments = env::args();

    if arguments.len() > 0 {
        for arg in arguments {
            if arg.ends_with(".glang") {}
        }
    }
}
