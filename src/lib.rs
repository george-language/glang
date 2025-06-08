mod errors;
mod lexing;
mod syntax;
use crate::lexing::lexer::Lexer;
use std::fs;

pub fn run(filename: &str, code: String) -> (String, String) {
    if filename == "<stdin>" {
        let mut lexer = Lexer::new(filename.to_string(), code.clone());
        let (tokens, e) = lexer.make_tokens();

        println!("{:?}", tokens);

        if let Some(error) = e {
            println!("{error}")
        }
    } else {
        let contents = fs::read_to_string(filename);
    }

    ("".to_string(), "".to_string())
}
