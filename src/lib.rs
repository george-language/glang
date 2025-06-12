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

pub fn run(filename: &str, code: Option<String>) -> (&'static str, Option<StandardError>) {
    let mut contents = String::new();

    if filename == "<stdin>" {
        contents = code.unwrap_or_else(|| "".to_string());
    } else {
        let result = fs::read_to_string(filename);

        if !result.is_ok() {
            println!("Failed to read .glang file");

            return ("", None);
        } else {
            contents = result.unwrap();
        }
    }

    let mut lexer = Lexer::new(filename.to_string(), contents.clone());
    let (tokens, error) = lexer.make_tokens();

    if error.is_some() {
        // error exists
        return ("", error);
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    if ast.error.is_some() {
        return ("", ast.error);
    }

    let mut interpreter = Interpreter::new();

    ("", None)
}
