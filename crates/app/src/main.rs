use clap::{Parser as ClapParser, Subcommand};
use glang_attributes::StandardError;
use glang_interpreter::interpret;
use glang_lexer::lex;
use glang_parser::parse;
use glang_tooling::log_error;
use std::{
    env, fs,
    io::{Write, stdin, stdout},
    panic,
    path::Path,
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
    #[command(name = "self", about = "Manage the glang binary and components")]
    GlangSelf {
        #[command(subcommand)]
        action: SelfCommands,
    },
    #[command(about = "Create a new project folder")]
    New {
        #[command(subcommand)]
        action: NewCommands,
    },
    #[command(about = "Run a string of glang source code")]
    Run { code: String },
    #[command(about = "Install a '.kennel' file")]
    Install {
        name: String,
        #[arg(long, help = "Force overwriting of the installed '.kennel'")]
        force: bool,
    },
    #[command(about = "Remove an installed kennel")]
    Remove {
        name: String,
        #[arg(long, help = "Force removal of the kennel")]
        force: bool,
    },
    #[command(about = "Package a project into an installable '.kennel' file")]
    Package,
}

#[derive(Subcommand)]
enum SelfCommands {
    #[command(about = "Update glang to the latest version")]
    Update,
    #[command(about = "Uninstall glang from the system")]
    Uninstall,
}

#[derive(Subcommand)]
enum NewCommands {
    #[command(about = "Create a new project configuration")]
    Project,
    #[command(about = "Create a new kennel configuration")]
    Kennel,
}

fn main() {
    panic::set_hook(Box::new(|info| {
        if let Some(location) = info.location() {
            log_error(&format!(
                "Error occured in {}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            ));
        }

        if let Some(msg) = info.payload().downcast_ref::<&str>() {
            log_error(msg);
        } else if let Some(msg) = info.payload().downcast_ref::<String>() {
            log_error(&msg);
        }
    }));

    let registry = glang_tooling::read_registry();

    // if 'glang-lib' isn't installed, retreive it
    if let Some(_) = registry.packages.get("lib") {
    } else {
        glang_tooling::install_library()
    }

    let cli = Cli::parse();

    match (cli.command, cli.file) {
        (Some(Commands::GlangSelf { action }), _) => match action {
            SelfCommands::Update => {
                glang_tooling::update_self();
            }
            SelfCommands::Uninstall => {
                glang_tooling::uninstall_self();
            }
        },
        (Some(Commands::New { action }), _) => match action {
            NewCommands::Kennel => {
                glang_tooling::create_package_folder();
            }
            NewCommands::Project => {
                glang_tooling::create_project_folder();
            }
        },
        (Some(Commands::Run { code }), _) => {
            if let Some(err) = run("<stdin>", Some(code)) {
                println!("{err}");
            }
        }
        (Some(Commands::Install { name, force }), _) => {
            glang_tooling::add_package(&name, force);
        }
        (Some(Commands::Remove { name, force }), _) => {
            glang_tooling::remove_package(&name, force);
        }
        (Some(Commands::Package), _) => {
            glang_tooling::write_package_file(None);
        }
        (None, Some(file)) => {
            if !file.ends_with(".glang") {
                println!("Unable to read provided file (not a '.glang' file)");

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
/// If the binary is built with the `benchmark` feature enabled, e.g. `cargo build --features benchmark`,
/// this function will automatically time the lexing -> parsing -> interpreting process and display the result
fn run(filename: &str, code: Option<String>) -> Option<StandardError> {
    let contents = if let Some(c) = code {
        c
    } else {
        fs::read_to_string(filename).expect("Unable to read provided '.glang' file")
    };

    let filename = Path::new(filename);
    let total_time = Instant::now();

    let error = match lex(filename, &contents) {
        Ok(tokens) => match parse(&tokens, &contents) {
            Ok(ast_node) => match interpret(ast_node, &contents) {
                Some(e) => Some(e),
                None => None,
            },
            Err(e) => Some(e),
        },
        Err(e) => Some(e),
    };

    if cfg!(feature = "benchmark") {
        println!(
            "Total time elapsed: {:?}ms",
            total_time.elapsed().as_millis()
        );
    }

    error
}

/// Starts the glang read evaluate print loop (REPL) using stdio
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

        if let Some(mut e) = run("<stdin>", Some(code.clone())) {
            e.contents = Some(code);

            println!("{e}");

            continue; // keep evaluating more code
        }
    }
}
