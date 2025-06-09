mod errors;
mod lexing;
mod nodes;
mod parsing;
mod syntax;
use crate::{errors::standard_error::StandardError, lexing::lexer::Lexer, parsing::parser::Parser};
use std::fs;

pub fn run(filename: &str, code: String) -> (String, Option<StandardError>) {
    if filename == "<stdin>" {
        let mut lexer = Lexer::new(filename.to_string(), code.clone());
        let (tokens, error) = lexer.make_tokens();

        if let Some(_) = error {
            // error exists
            return ("".to_string(), error);
        }

        let parser = Parser::new(tokens);
        parser.parse();
    } else {
        let contents = fs::read_to_string(filename);
    }

    ("".to_string(), None)
}
