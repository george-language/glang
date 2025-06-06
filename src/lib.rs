mod syntax;
mod token;
use std::fs;

pub fn run(filename: &str, code: String) -> (String, String) {
    if filename == "<stdin>" {
    } else {
        let contents = fs::read_to_string(filename);
    }

    ("".to_string(), "".to_string())
}
