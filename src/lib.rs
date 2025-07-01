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
};
use std::fs;
use std::time::Instant;

pub fn run(filename: &str, code: Option<String>) -> Option<StandardError> {
    let mut contents = String::new();
    contents.push_str("fetch(\"library/default/lib.glang\");\n\n");

    if filename == "<stdin>" {
        contents.push_str(code.unwrap_or_else(|| "".to_string()).as_str());
    } else {
        let result = fs::read_to_string(filename);

        if !result.is_ok() {
            println!("Failed to read provided '.glang' file");

            return None;
        } else {
            contents.push_str(result.unwrap().as_str());
        }
    }

    let start = Instant::now();

    let mut lexer = Lexer::new(filename.to_string(), contents.clone());
    let (tokens, error) = lexer.make_tokens();

    if error.is_some() {
        // error exists
        return error;
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    if ast.error.is_some() {
        return ast.error;
    }

    let mut interpreter = Interpreter::new();
    let mut context = Context::new("<program>".to_string(), None, None);
    context.symbol_table = Some(interpreter.global_symbol_table.clone());
    let result = interpreter.visit(ast.node.unwrap(), &mut context);

    let duration = start.elapsed();

    if cfg!(feature = "benchmark") {
        println!("Time elapsed in ms: {:?}", &duration);
    }

    result.error
}
