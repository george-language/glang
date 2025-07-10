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
use std::io::{Write, stdin, stdout};
use std::time::Instant;
use std::{fs, path::Path};

pub fn run(filename: &str, code: Option<String>) -> Option<StandardError> {
    let contents = if filename == "<stdin>" {
        code.unwrap_or_default()
    } else {
        match fs::read_to_string(filename) {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to read provided '.glang' file: {e}");

                return None;
            }
        }
    };

    let start = Instant::now();

    let mut lexer = Lexer::new(filename, contents.clone());
    let token_result = lexer.make_tokens();

    if token_result.is_err() {
        return token_result.err();
    }

    let mut parser = Parser::new(&token_result.ok().unwrap());
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
        println!("Time elapsed: {:?}ms", start.elapsed().as_millis());
    }

    result.error
}

pub fn launch_repl(version: &str) {
    println!("George Language {version}\nType '/exit' to exit");

    loop {
        let mut code = String::new();

        print!(">>> ");
        let _ = stdout().flush();

        stdin()
            .read_line(&mut code)
            .expect("Input text (stdin) was not a valid string");

        if code.trim() == "/exit" {
            break;
        }

        let error = run("<stdin>", Some(code));

        if let Some(e) = error {
            println!("{}", e);

            continue;
        }
    }
}

pub fn new_project(dir_name: &Path) {
    fs::create_dir(&dir_name).expect("Cannot create directory (invalid name)");
    fs::create_dir(&dir_name.join("src")).expect("'src/' directory already exists");

    let _ = fs::write(
        &dir_name.join("main.glang"),
        "func main() {\n    bark(\"Hello world!\");\n}\n\nmain();",
    );
    let _ = fs::write(
        &dir_name.join("README.md"),
        "# Welcome to GLang!\nTo get started, see our documentation [here](https://sites.google.com/view/george-lang/documentation).",
    );
}
