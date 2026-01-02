use std::fs;
use std::path::PathBuf;

use clap::{Parser as ClapParser, Subcommand};

use crate::executor::Executor;
use crate::lexer::Lexer;
use crate::parser::Parser;

/// Weave programming language interpreter
#[derive(ClapParser)]
#[command(name = "weave")]
#[command(version = "0.1.0")]
#[command(about = "Scripting, batteries included", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a Weave program
    Run {
        /// Path to the .wv file to run
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}

impl Commands {
    pub fn execute(&self) {
        match self {
            Commands::Run { file } => run(file),
        }
    }
}

fn run(file: &PathBuf) {
    let input = fs::read(file).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", file.display(), err);
        std::process::exit(1);
    });

    // Lexer: tokenize the input bytes
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    // Debug: print tokens
    dbg!(&tokens);

    // Parser: parse tokens into AST
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    // Debug: print AST
    dbg!(&ast);

    // Executor: execute the AST
    let executor = Executor::new(ast);
    executor.exec();
}
