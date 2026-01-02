#![allow(unused)]
#![allow(dead_code)]

use std::fs::{self, read};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    // Literals
    Identifier(String),
    String(String),
    Number(f64),

    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Semicolon,
    Colon,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Keywords
    Let,
    Fn,
    If,
    Else,
    While,
    For,
    Return,
    True,
    False,
    Nil,

    // Special
    Eof,
}

struct Lexer {
    input: Vec<u8>,
    position: usize,
    current: Option<u8>,
}

impl Lexer {
    pub fn new(input: Vec<u8>) -> Self {
        let current = if input.is_empty() { None } else { Some(input[0]) };
        Lexer {
            input,
            position: 0,
            current,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.current.is_some() {
            self.skip_whitespace();

            if self.current.is_none() {
                break;
            }

            let token = self.next_token();
            tokens.push(token);
        }

        tokens.push(Token::Eof);
        tokens
    }

    fn next_token(&mut self) -> Token {
        let ch = self.current.unwrap();

        match ch {
            b'(' => {
                self.advance();
                Token::LeftParen
            }
            b')' => {
                self.advance();
                Token::RightParen
            }
            b'{' => {
                self.advance();
                Token::LeftBrace
            }
            b'}' => {
                self.advance();
                Token::RightBrace
            }
            b'[' => {
                self.advance();
                Token::LeftBracket
            }
            b']' => {
                self.advance();
                Token::RightBracket
            }
            b',' => {
                self.advance();
                Token::Comma
            }
            b'.' => {
                self.advance();
                Token::Dot
            }
            b';' => {
                self.advance();
                Token::Semicolon
            }
            b':' => {
                self.advance();
                Token::Colon
            }
            b'+' => {
                self.advance();
                Token::Plus
            }
            b'-' => {
                self.advance();
                Token::Minus
            }
            b'*' => {
                self.advance();
                Token::Star
            }
            b'/' => {
                self.advance();
                Token::Slash
            }
            b'=' => {
                self.advance();
                if self.current == Some(b'=') {
                    self.advance();
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            }
            b'!' => {
                self.advance();
                if self.current == Some(b'=') {
                    self.advance();
                    Token::BangEqual
                } else {
                    Token::Bang
                }
            }
            b'<' => {
                self.advance();
                if self.current == Some(b'=') {
                    self.advance();
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            b'>' => {
                self.advance();
                if self.current == Some(b'=') {
                    self.advance();
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            b'"' => self.read_string(),
            b'0'..=b'9' => self.read_number(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.read_identifier(),
            _ => {
                self.advance();
                // For unknown characters, create an identifier (or could error)
                Token::Identifier(String::from("UNKNOWN"))
            }
        }
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // Skip opening quote
        let mut value = String::new();

        while let Some(ch) = self.current {
            if ch == b'"' {
                self.advance(); // Skip closing quote
                break;
            }
            value.push(ch as char);
            self.advance();
        }

        Token::String(value)
    }

    fn read_number(&mut self) -> Token {
        let mut value = String::new();

        while let Some(ch) = self.current {
            if ch.is_ascii_digit() || ch == b'.' {
                value.push(ch as char);
                self.advance();
            } else {
                break;
            }
        }

        let num = value.parse::<f64>().unwrap_or(0.0);
        Token::Number(num)
    }

    fn read_identifier(&mut self) -> Token {
        let mut value = String::new();

        while let Some(ch) = self.current {
            if ch.is_ascii_alphanumeric() || ch == b'_' {
                value.push(ch as char);
                self.advance();
            } else {
                break;
            }
        }

        // Check if identifier is a keyword
        match value.as_str() {
            "let" => Token::Let,
            "fn" => Token::Fn,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            "nil" => Token::Nil,
            _ => Token::Identifier(value),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current {
            if ch.is_ascii_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current = if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        };
    }
}

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

    // Executor: execute the AST
    let executor = Executor::new(ast);
    executor.exec();
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Ast {
        Ast::new()
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn peek(&self, offset: usize) -> &Token {
        self.tokens
            .get(self.position + offset)
            .unwrap_or(&Token::Eof)
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
