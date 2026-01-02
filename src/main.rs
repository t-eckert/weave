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

    // Debug: print AST
    dbg!(&ast);

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
        let mut statements = Vec::new();

        while !matches!(self.current_token(), Token::Eof) {
            statements.push(self.parse_statement());
        }

        Ast::new(statements)
    }

    // Statement parsing
    fn parse_statement(&mut self) -> Stmt {
        match self.current_token() {
            Token::Let => self.parse_let(),
            Token::Fn => self.parse_function(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::Return => self.parse_return(),
            Token::LeftBrace => self.parse_block(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let(&mut self) -> Stmt {
        self.advance(); // consume 'let'

        let name = match self.current_token() {
            Token::Identifier(n) => n.clone(),
            _ => panic!("Expected identifier after 'let'"),
        };
        self.advance();

        // Expect '='
        if !matches!(self.current_token(), Token::Equal) {
            panic!("Expected '=' in let statement");
        }
        self.advance();

        let value = self.parse_expression();

        // Optional semicolon
        if matches!(self.current_token(), Token::Semicolon) {
            self.advance();
        }

        Stmt::Let { name, value }
    }

    fn parse_function(&mut self) -> Stmt {
        self.advance(); // consume 'fn'

        let name = match self.current_token() {
            Token::Identifier(n) => n.clone(),
            _ => panic!("Expected function name"),
        };
        self.advance();

        // Parse parameters
        if !matches!(self.current_token(), Token::LeftParen) {
            panic!("Expected '(' after function name");
        }
        self.advance();

        let mut params = Vec::new();
        while !matches!(self.current_token(), Token::RightParen) {
            if let Token::Identifier(param) = self.current_token() {
                params.push(param.clone());
                self.advance();

                if matches!(self.current_token(), Token::Comma) {
                    self.advance();
                }
            } else {
                panic!("Expected parameter name");
            }
        }
        self.advance(); // consume ')'

        // Parse body
        let body = if matches!(self.current_token(), Token::LeftBrace) {
            match self.parse_block() {
                Stmt::Block(stmts) => stmts,
                _ => panic!("Expected block"),
            }
        } else {
            panic!("Expected function body");
        };

        Stmt::Function { name, params, body }
    }

    fn parse_if(&mut self) -> Stmt {
        self.advance(); // consume 'if'

        let condition = self.parse_expression();

        let then_branch = if matches!(self.current_token(), Token::LeftBrace) {
            match self.parse_block() {
                Stmt::Block(stmts) => stmts,
                _ => panic!("Expected block"),
            }
        } else {
            panic!("Expected '{{' after if condition");
        };

        let else_branch = if matches!(self.current_token(), Token::Else) {
            self.advance();
            Some(match self.parse_block() {
                Stmt::Block(stmts) => stmts,
                _ => panic!("Expected block"),
            })
        } else {
            None
        };

        Stmt::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    fn parse_while(&mut self) -> Stmt {
        self.advance(); // consume 'while'

        let condition = self.parse_expression();

        let body = if matches!(self.current_token(), Token::LeftBrace) {
            match self.parse_block() {
                Stmt::Block(stmts) => stmts,
                _ => panic!("Expected block"),
            }
        } else {
            panic!("Expected '{{' after while condition");
        };

        Stmt::While { condition, body }
    }

    fn parse_return(&mut self) -> Stmt {
        self.advance(); // consume 'return'

        let value = if matches!(self.current_token(), Token::Semicolon | Token::RightBrace) {
            None
        } else {
            Some(self.parse_expression())
        };

        if matches!(self.current_token(), Token::Semicolon) {
            self.advance();
        }

        Stmt::Return(value)
    }

    fn parse_block(&mut self) -> Stmt {
        self.advance(); // consume '{'

        let mut statements = Vec::new();

        while !matches!(self.current_token(), Token::RightBrace | Token::Eof) {
            statements.push(self.parse_statement());
        }

        if !matches!(self.current_token(), Token::RightBrace) {
            panic!("Expected '}}' at end of block");
        }
        self.advance(); // consume '}'

        Stmt::Block(statements)
    }

    fn parse_expression_statement(&mut self) -> Stmt {
        let expr = self.parse_expression();

        // Optional semicolon
        if matches!(self.current_token(), Token::Semicolon) {
            self.advance();
        }

        Stmt::Expression(expr)
    }

    // Expression parsing (with precedence)
    fn parse_expression(&mut self) -> Expr {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Expr {
        let mut expr = self.parse_comparison();

        while matches!(
            self.current_token(),
            Token::EqualEqual | Token::BangEqual
        ) {
            let operator = match self.current_token() {
                Token::EqualEqual => BinaryOp::Equal,
                Token::BangEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_term();

        while matches!(
            self.current_token(),
            Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual
        ) {
            let operator = match self.current_token() {
                Token::Greater => BinaryOp::Greater,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                Token::Less => BinaryOp::Less,
                Token::LessEqual => BinaryOp::LessEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();

        while matches!(self.current_token(), Token::Plus | Token::Minus) {
            let operator = match self.current_token() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_unary();

        while matches!(self.current_token(), Token::Star | Token::Slash) {
            let operator = match self.current_token() {
                Token::Star => BinaryOp::Multiply,
                Token::Slash => BinaryOp::Divide,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn parse_unary(&mut self) -> Expr {
        if matches!(self.current_token(), Token::Bang | Token::Minus) {
            let operator = match self.current_token() {
                Token::Bang => UnaryOp::Not,
                Token::Minus => UnaryOp::Negate,
                _ => unreachable!(),
            };
            self.advance();
            let operand = self.parse_unary();
            return Expr::Unary {
                operator,
                operand: Box::new(operand),
            };
        }

        self.parse_call()
    }

    fn parse_call(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        loop {
            if matches!(self.current_token(), Token::LeftParen) {
                self.advance();
                let mut arguments = Vec::new();

                if !matches!(self.current_token(), Token::RightParen) {
                    loop {
                        arguments.push(self.parse_expression());

                        if matches!(self.current_token(), Token::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }

                if !matches!(self.current_token(), Token::RightParen) {
                    panic!("Expected ')' after arguments");
                }
                self.advance();

                expr = Expr::Call {
                    callee: Box::new(expr),
                    arguments,
                };
            } else {
                break;
            }
        }

        expr
    }

    fn parse_primary(&mut self) -> Expr {
        let expr = match self.current_token().clone() {
            Token::Number(n) => Expr::Number(n),
            Token::String(s) => Expr::String(s),
            Token::True => Expr::Boolean(true),
            Token::False => Expr::Boolean(false),
            Token::Nil => Expr::Nil,
            Token::Identifier(name) => Expr::Identifier(name),
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression();
                if !matches!(self.current_token(), Token::RightParen) {
                    panic!("Expected ')' after expression");
                }
                self.advance();
                return Expr::Grouping(Box::new(expr));
            }
            _ => panic!("Unexpected token: {:?}", self.current_token()),
        };

        self.advance();
        expr
    }

    // Helper methods
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

// AST Node types
#[derive(Debug, Clone, PartialEq)]
enum Expr {
    // Literals
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,

    // Identifier
    Identifier(String),

    // Binary operations
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },

    // Unary operations
    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
    },

    // Function call
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },

    // Grouping
    Grouping(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Debug, Clone, PartialEq)]
enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
enum Stmt {
    // Expression statement
    Expression(Expr),

    // Let binding
    Let { name: String, value: Expr },

    // Function declaration
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },

    // If statement
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },

    // While loop
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },

    // Return statement
    Return(Option<Expr>),

    // Block
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq)]
struct Ast {
    statements: Vec<Stmt>,
}

impl Ast {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Ast { statements }
    }
}

struct Executor {
    ast: Ast,
}

impl Executor {
    pub fn new(ast: Ast) -> Self {
        Executor { ast }
    }

    pub fn exec(&self) {
        for statement in &self.ast.statements {
            self.execute_statement(statement);
        }
    }

    fn execute_statement(&self, stmt: &Stmt) {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate_expression(expr);
            }
            Stmt::Let { name, value } => {
                let _result = self.evaluate_expression(value);
                // For now, just evaluate - no variable storage yet
                println!("Let binding: {} = {:?}", name, _result);
            }
            Stmt::Function { name, params, body } => {
                println!("Function declaration: {}({:?})", name, params);
                // Store function for later - not implemented yet
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_result = self.evaluate_expression(condition);
                if self.is_truthy(&cond_result) {
                    for stmt in then_branch {
                        self.execute_statement(stmt);
                    }
                } else if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        self.execute_statement(stmt);
                    }
                }
            }
            Stmt::While { condition, body } => {
                while self.is_truthy(&self.evaluate_expression(condition)) {
                    for stmt in body {
                        self.execute_statement(stmt);
                    }
                }
            }
            Stmt::Return(value) => {
                if let Some(expr) = value {
                    let _result = self.evaluate_expression(expr);
                    println!("Return: {:?}", _result);
                } else {
                    println!("Return: nil");
                }
            }
            Stmt::Block(statements) => {
                for stmt in statements {
                    self.execute_statement(stmt);
                }
            }
        }
    }

    fn evaluate_expression(&self, expr: &Expr) -> Value {
        match expr {
            Expr::String(s) => Value::String(s.clone()),
            Expr::Number(n) => Value::Number(*n),
            Expr::Boolean(b) => Value::Boolean(*b),
            Expr::Nil => Value::Nil,
            Expr::Identifier(name) => {
                // For now, just return nil - no variable lookup yet
                println!("Identifier lookup: {}", name);
                Value::Nil
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate_expression(left);
                let right_val = self.evaluate_expression(right);
                self.evaluate_binary_op(&left_val, operator, &right_val)
            }
            Expr::Unary { operator, operand } => {
                let operand_val = self.evaluate_expression(operand);
                self.evaluate_unary_op(operator, &operand_val)
            }
            Expr::Call { callee, arguments } => {
                // Evaluate callee
                if let Expr::Identifier(name) = callee.as_ref() {
                    // Built-in functions
                    if name == "print" {
                        for arg in arguments {
                            let value = self.evaluate_expression(arg);
                            println!("{}", self.value_to_string(&value));
                        }
                        return Value::Nil;
                    }
                }

                // For other functions, just print for now
                println!("Function call: {:?}", callee);
                Value::Nil
            }
            Expr::Grouping(inner) => self.evaluate_expression(inner),
        }
    }

    fn evaluate_binary_op(&self, left: &Value, op: &BinaryOp, right: &Value) -> Value {
        match (left, op, right) {
            (Value::Number(l), BinaryOp::Add, Value::Number(r)) => Value::Number(l + r),
            (Value::Number(l), BinaryOp::Subtract, Value::Number(r)) => Value::Number(l - r),
            (Value::Number(l), BinaryOp::Multiply, Value::Number(r)) => Value::Number(l * r),
            (Value::Number(l), BinaryOp::Divide, Value::Number(r)) => Value::Number(l / r),
            (Value::Number(l), BinaryOp::Less, Value::Number(r)) => Value::Boolean(l < r),
            (Value::Number(l), BinaryOp::LessEqual, Value::Number(r)) => Value::Boolean(l <= r),
            (Value::Number(l), BinaryOp::Greater, Value::Number(r)) => Value::Boolean(l > r),
            (Value::Number(l), BinaryOp::GreaterEqual, Value::Number(r)) => Value::Boolean(l >= r),
            (l, BinaryOp::Equal, r) => Value::Boolean(l == r),
            (l, BinaryOp::NotEqual, r) => Value::Boolean(l != r),
            _ => {
                println!("Invalid binary operation: {:?} {:?} {:?}", left, op, right);
                Value::Nil
            }
        }
    }

    fn evaluate_unary_op(&self, op: &UnaryOp, operand: &Value) -> Value {
        match (op, operand) {
            (UnaryOp::Negate, Value::Number(n)) => Value::Number(-n),
            (UnaryOp::Not, val) => Value::Boolean(!self.is_truthy(val)),
            _ => {
                println!("Invalid unary operation: {:?} {:?}", op, operand);
                Value::Nil
            }
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Boolean(b) => *b,
            Value::Nil => false,
            _ => true,
        }
    }

    fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Nil => "nil".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}
