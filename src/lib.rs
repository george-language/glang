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
    let contents = if filename == "<stdin>" {
        code.unwrap_or_default()
    } else {
        match fs::read_to_string(filename) {
            Ok(s) => s,
            Err(_) => {
                println!("Failed to read provided '.glang' file");

                return None;
            }
        }
    };

    let start = Instant::now();

    let mut lexer = Lexer::new(filename, contents.clone());
    let (tokens, error) = lexer.make_tokens();

    if error.is_some() {
        return error;
    }

    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();

    if ast.error.is_some() {
        return ast.error;
    }

    let mut interpreter = Interpreter::new();
    let mut context = Context::new("<program>".to_string(), None, None);
    context.symbol_table = Some(interpreter.global_symbol_table.clone());

    if let Some(e) = interpreter.evaluate(
        "fetch _env(\"GLANG_STD\") + \"/default/lib.glang\";",
        &mut context,
    ) {
        return Some(e);
    }

    let result = interpreter.visit(ast.node.unwrap(), &mut context);

    if cfg!(feature = "benchmark") {
        let duration = start.elapsed();

        println!("Time elapsed: {:?}ms", &duration.as_millis());
    }

    result.error
}
