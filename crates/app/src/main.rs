use clap::{Parser as ClapParser, Subcommand};
use glang_attributes::StandardError;
use glang_interpreter::{Context, Interpreter};
use glang_lexer::Lexer;
use glang_logging::log_error;
use glang_parser::Parser;
use std::cell::RefCell;
use std::{
    env, fs,
    io::{Write, stdin, stdout},
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
    #[command(about = "Run a string of glang source code")]
    Run { code: String },
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
            .join("library");

        let pkg_path = dirs::home_dir()
            .expect("Unable to retrieve user home directory")
            .join(".glang")
            .join("kennels");

        // these variables are set so that we can use them inside glang
        // GLANG_STD is the path to the standard library ('library/')
        // GLANG_PKG is the path to the kennels directory ('.glang/kennels/')
        env::set_var(
            "GLANG_STD",
            &std_path
                .to_string_lossy()
                .replace("target", "") // on development, get rid of the target folder in the path
                .replace("debug", "")
                .replace("release", ""),
        );
        env::set_var("GLANG_PKG", &pkg_path.to_string_lossy().to_string());
    }

    // we have to run this everytime the glang executable is ran to double check 'kennels/' always exists
    glang_package_manager::create_package_dir();

    let cli = Cli::parse();

    match (cli.command, cli.file) {
        (Some(Commands::Run { code }), _) => {
            if let Some(err) = run("<stdin>", Some(code)) {
                println!("{err}");
            }
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
            if !file.ends_with(".glang") {
                log_error("failed to read provided file (not a '.glang' file)");

                return;
            }

            // if the file argument is valid, pass it on to the run function
            if let Some(err) = run(&file, None) {
                println!("{err}");
            }
        }
        (None, None) => {
            // 'glang' by itself will just run the REPL, similar to python
            launch_repl();
        }
    }
}

/// Run a '.glang' file or raw glang source code
///
/// ```rust
/// let result = run("example.glang", None);
///
/// if let Some(err) = result {
///     println!("{err}");
/// }
/// ```
///
/// Running raw glang code:
///
/// ```rust
/// let result = run("<stdin>", Some("bark(1 + 1);".to_string()));
///
/// if let Some(err) = result {
///     println!("{err}");
/// }
/// ```
///
/// If the binary is built with the `benchmark` feature enabled, e.g. `cargo build --features benchmark`,
/// this function will automatically time the lexing -> parsing -> interpreting process and display the result
fn run(filename: &str, code: Option<String>) -> Option<StandardError> {
    let contents = if filename == "<stdin>" {
        code.unwrap_or_default()
    } else {
        match fs::read_to_string(filename) {
            Ok(s) => s,
            Err(e) => {
                log_error(&format!("failed to read provided '.glang' file ({e})"));

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

    if !cfg!(feature = "no-std") {
        if let Some(e) = interpreter.evaluate(
            "fetch _env(\"GLANG_STD\") + \"/fundamental/lib.glang\";",
            context.clone(),
        ) {
            return Some(e);
        }
    }

    let result = interpreter.visit(ast.node.unwrap().as_ref(), context.clone());

    let interpreting_time = interpreting_time.elapsed();

    if cfg!(feature = "benchmark") {
        // only display benchmarking if feature is enabled
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

/// Starts the glang REPL using stdio
///
/// ```rust
/// launch_repl();
/// ```
///
/// An infinite loop in the terminal running glang code and evaluating it
fn launch_repl() {
    println!("George Language {VERSION}\nType '/exit' to exit");

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

            continue; // keep evaluating more code
        }
    }
}
