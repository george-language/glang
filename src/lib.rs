mod lexing;
mod syntax;
use std::fs;

use crate::lexing::lexer::Lexer;

pub fn run(filename: &str, code: String) -> (String, String) {
    if filename == "<stdin>" {
        let mut lexer = Lexer::new(filename.to_string(), code.clone());
        let (tokens, _) = lexer.make_tokens();

        println!("{:?}", tokens);
    } else {
        let contents = fs::read_to_string(filename);
    }

    ("".to_string(), "".to_string())
}
