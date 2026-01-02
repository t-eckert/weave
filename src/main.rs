#![allow(unused)]
#![allow(dead_code)]

use clap::Parser as ClapParser;

mod ast;
mod cli;
mod executor;
mod lexer;
mod parser;

use cli::Cli;

fn main() {
    let cli = Cli::parse();
    cli.command.execute();
}
