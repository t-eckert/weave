#![allow(unused)]
#![allow(dead_code)]

use std::fs;

mod ast;
mod executor;
mod lexer;
mod parser;

use executor::Executor;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let input_file = "hello.wv";

    let input = fs::read(input_file).expect("File must exist");

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
