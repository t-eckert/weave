#![allow(unused)]
#![allow(dead_code)]

use std::fs::{self, read};

fn main() {
    let input_file = "hello.wv";

    let input = fs::read(input_file).expect("File must exist");

    let parser = Parser::new();
    let ast = parser.parse(&input);
    let executor = Executor::new(ast);
    executor.exec();
}

struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse(&self, bytes: &[u8]) -> Ast {
        Ast::new()
    }
}

struct Ast {}

impl Ast {
    pub fn new() -> Self {
        Ast {}
    }
}

struct Executor {
    ast: Ast,
}

impl Executor {
    pub fn new(ast: Ast) -> Self {
        Executor { ast }
    }

    pub fn exec(self) {
        dbg!("exec");
    }
}
