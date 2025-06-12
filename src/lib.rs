mod errors;
mod interpreting;
mod lexing;
mod nodes;
mod parsing;
mod syntax;
use crate::{
    errors::standard_error::StandardError, interpreting::interpreter::Interpreter,
    lexing::lexer::Lexer, parsing::parser::Parser,
};
use std::fs;

pub fn run(filename: &str, code: String) -> (String, Option<StandardError>) {
    if filename == "<stdin>" {
        let mut lexer = Lexer::new(filename.to_string(), code.clone());
        let (tokens, error) = lexer.make_tokens();

        if error.is_some() {
            // error exists
            return ("".to_string(), error);
        }

        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        if ast.error.is_some() {
            return ("".to_string(), ast.error);
        }

        let mut interpreter = Interpreter::new();
    } else {
        let contents = fs::read_to_string(filename);
    }

    ("".to_string(), None)
}
