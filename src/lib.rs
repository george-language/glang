mod errors;
mod interpreting;
mod lexing;
mod nodes;
mod parsing;
mod syntax;
mod values;
use crate::{
    errors::standard_error::StandardError,
    interpreting::{context::Context, interpreter::Interpreter},
    lexing::lexer::Lexer,
    parsing::parser::Parser,
    values::{list::List, value::Value},
};
use std::fs;

pub fn run(filename: &str, code: Option<String>) -> (Option<String>, Option<StandardError>) {
    let mut contents = String::new();

    if filename == "<stdin>" {
        contents = code.unwrap_or_else(|| "".to_string());
    } else {
        let result = fs::read_to_string(filename);

        if !result.is_ok() {
            println!("Failed to read .glang file");

            return (None, None);
        } else {
            contents = result.unwrap();
        }
    }

    let mut lexer = Lexer::new(filename.to_string(), contents.clone());
    let (tokens, error) = lexer.make_tokens();

    if error.is_some() {
        // error exists
        return (None, error);
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    if ast.error.is_some() {
        return (None, ast.error);
    }

    let mut interpreter = Interpreter::new();
    let mut context = Context::new("<program>", None, None);
    context.symbol_table = Some(interpreter.global_symbol_table.clone());
    let result = interpreter.visit(ast.node.unwrap(), context);

    (
        Some(
            result
                .value
                .unwrap_or_else(|| Box::new(Value::ListValue(List::new(Vec::new()))))
                .as_string(),
        ),
        result.error,
    )
}
