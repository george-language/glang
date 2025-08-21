use clap::{Parser as ClapParser, Subcommand};
use glang_attributes::StandardError;
use glang_interpreter::{Context, Interpreter};
use glang_lexer::Lexer;
use glang_parser::Parser;
use simply_colored::*;
use std::cell::RefCell;
use std::{
    env, fs,
    io::{Write, stdin, stdout},
    path::Path,
    rc::Rc,
    time::Instant,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(ClapParser)]
#[command(name = "glang", version = VERSION, about = "The George Programming Language")]
struct Cli {
    file: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create a glang project")]
    New { name: String },
    #[command(about = "Initialize a glang project in the current directory")]
    Init,
    #[command(about = "Install a glang kennel from the internet")]
    Install { name: String },
    #[command(about = "Remove an installed glang kennel")]
    Remove { name: String },
    #[command(about = "Update an installed glang kennel to the latest version")]
    Update { name: String },
}

fn main() {
    unsafe {
        let std_path = env::current_exe()
            .expect("Unable to retrieve executable path")
            .parent()
            .unwrap()
            .join("library")
            .to_string_lossy()
            .replace("\\", "/")
            .replace("target/debug/", "")
            .replace("target/release/", "");

        let pkg_path = dirs::home_dir()
            .expect("Unable to retrieve user home directory")
            .join(".glang")
            .join("kennels")
            .to_string_lossy()
            .replace("\\", "/");

        env::set_var("GLANG_STD", &std_path);
        env::set_var("GLANG_PKG", &pkg_path);
    }

    glang_package_manager::create_package_dir();

    let cli = Cli::parse();

    match (cli.command, cli.file) {
        (Some(Commands::New { name }), _) => {
            new_project(Path::new(&name), false);
        }
        (Some(Commands::Init), _) => {
            new_project(Path::new("."), true);
        }
        (Some(Commands::Install { name }), _) => {
            glang_package_manager::add_package(&name);
        }
        (Some(Commands::Remove { name }), _) => {
            glang_package_manager::remove_package(&name);
        }
        (Some(Commands::Update { name }), _) => {
            glang_package_manager::update_package(&name);
        }
        (None, Some(file)) => {
            let error = run(&file, None);

            if let Some(err) = error {
                println!("{err}");
            }
        }
        (None, None) => {
            launch_repl(VERSION);
        }
    }
}

fn run(filename: &str, code: Option<String>) -> Option<StandardError> {
    let contents = if filename == "<stdin>" {
        code.unwrap_or_default()
    } else {
        match fs::read_to_string(filename) {
            Ok(s) => s,
            Err(e) => {
                println!("{DIM_RED}Failed to read provided '.glang' file: {e}{RESET}");

                return None;
            }
        }
    };

    let total_time = Instant::now();

    let lexing_time = Instant::now();
    let mut lexer = Lexer::new(filename, contents.clone());
    let token_result = lexer.make_tokens();

    if token_result.is_err() {
        return token_result.err();
    }

    let lexing_time = lexing_time.elapsed();

    let parsing_time = Instant::now();
    let mut parser = Parser::new(&token_result.ok().unwrap());
    let ast = parser.parse();

    if ast.error.is_some() {
        return ast.error;
    }

    let parsing_time = parsing_time.elapsed();

    let interpreting_time = Instant::now();
    let mut interpreter = Interpreter::new();
    let context = Rc::new(RefCell::new(Context::new(
        "<program>".to_string(),
        None,
        None,
    )));
    context.borrow_mut().symbol_table = Some(interpreter.global_symbol_table.clone());

    if let Some(e) = interpreter.evaluate(
        "fetch _env(\"GLANG_STD\") + \"/default/lib.glang\";",
        context.clone(),
    ) {
        return Some(e);
    }

    let result = interpreter.visit(ast.node.unwrap(), context.clone());

    let interpreting_time = interpreting_time.elapsed();

    if cfg!(feature = "benchmark") {
        println!(
            "Total time elapsed: {:?}ms",
            total_time.elapsed().as_millis()
        );
        println!("Time to lex: {:?}ms", lexing_time.as_millis());
        println!("Time to parse: {:?}ms", parsing_time.as_millis());
        println!("Time to interpret: {:?}ms", interpreting_time.as_millis());
    }

    result.error
}

fn launch_repl(version: &str) {
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
            println!("{e}");

            continue;
        }
    }
}

pub fn new_project(dir_name: &Path, init: bool) {
    if !init {
        fs::create_dir(dir_name).expect("Cannot create directory (invalid name)");
    }

    fs::create_dir(dir_name.join("src")).expect("'src/' directory already exists");

    let _ = fs::write(
        dir_name.join("main.glang"),
        "func main() {\n    bark(\"Hello world!\");\n}\n\nmain();",
    );
    let _ = fs::write(
        dir_name.join("README.md"),
        "# Welcome to GLang!\nTo get started, see our documentation [here](https://sites.google.com/view/george-lang/documentation).",
    );
}
