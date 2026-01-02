use crate::ast::{Ast, BinaryOp, Expr, Stmt, Type, UnaryOp};
use crate::lexer::Token;

pub struct Parser {
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
            Token::Struct => self.parse_struct(),
            Token::Type => self.parse_type_alias(),
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
                let param_name = param.clone();
                self.advance();

                // Check for type annotation
                let param_type = if matches!(self.current_token(), Token::Colon) {
                    self.advance(); // consume ':'
                    Some(self.parse_type())
                } else {
                    None
                };

                params.push((param_name, param_type));

                if matches!(self.current_token(), Token::Comma) {
                    self.advance();
                }
            } else {
                panic!("Expected parameter name");
            }
        }
        self.advance(); // consume ')'

        // Parse optional return type
        let return_type = if matches!(self.current_token(), Token::Arrow) {
            self.advance(); // consume '->'
            Some(self.parse_type())
        } else {
            None
        };

        // Parse body
        let body = if matches!(self.current_token(), Token::LeftBrace) {
            match self.parse_block() {
                Stmt::Block(stmts) => stmts,
                _ => panic!("Expected block"),
            }
        } else {
            panic!("Expected function body");
        };

        Stmt::Function {
            name,
            params,
            return_type,
            body,
        }
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

    fn parse_struct(&mut self) -> Stmt {
        self.advance(); // consume 'struct'

        let name = match self.current_token() {
            Token::Identifier(n) => n.clone(),
            _ => panic!("Expected struct name"),
        };
        self.advance();

        // Expect '{'
        if !matches!(self.current_token(), Token::LeftBrace) {
            panic!("Expected '{{' after struct name");
        }
        self.advance();

        // Parse fields
        let mut fields = Vec::new();
        while !matches!(self.current_token(), Token::RightBrace | Token::Eof) {
            // Field name
            let field_name = match self.current_token() {
                Token::Identifier(n) => n.clone(),
                _ => panic!("Expected field name"),
            };
            self.advance();

            // Expect ':'
            if !matches!(self.current_token(), Token::Colon) {
                panic!("Expected ':' after field name");
            }
            self.advance();

            // Parse type
            let field_type = self.parse_type();

            fields.push((field_name, field_type));

            // Optional comma or newline (we just skip to next field)
            if matches!(self.current_token(), Token::Comma) {
                self.advance();
            }
        }

        if !matches!(self.current_token(), Token::RightBrace) {
            panic!("Expected '}}' at end of struct");
        }
        self.advance();

        Stmt::Struct { name, fields }
    }

    fn parse_type_alias(&mut self) -> Stmt {
        self.advance(); // consume 'type'

        let name = match self.current_token() {
            Token::Identifier(n) => n.clone(),
            _ => panic!("Expected type alias name"),
        };
        self.advance();

        // Expect '='
        if !matches!(self.current_token(), Token::Equal) {
            panic!("Expected '=' in type alias");
        }
        self.advance();

        // Parse union variants (string literals separated by |)
        let mut variants = Vec::new();

        loop {
            match self.current_token() {
                Token::String(s) => {
                    variants.push(s.clone());
                    self.advance();
                }
                _ => panic!("Expected string literal in type union"),
            }

            if matches!(self.current_token(), Token::Pipe) {
                self.advance(); // consume '|'
            } else {
                break;
            }
        }

        if variants.is_empty() {
            panic!("Type alias must have at least one variant");
        }

        Stmt::TypeAlias { name, variants }
    }

    fn parse_struct_literal(&mut self, name: String) -> Expr {
        // Expect '{'
        if !matches!(self.current_token(), Token::LeftBrace) {
            panic!("Expected '{{' for struct literal");
        }
        self.advance();

        // Parse fields
        let mut fields = Vec::new();
        while !matches!(self.current_token(), Token::RightBrace | Token::Eof) {
            // Field name
            let field_name = match self.current_token() {
                Token::Identifier(n) => n.clone(),
                _ => panic!("Expected field name"),
            };
            self.advance();

            // Expect ':'
            if !matches!(self.current_token(), Token::Colon) {
                panic!("Expected ':' after field name in struct literal");
            }
            self.advance();

            // Parse value expression
            let value = self.parse_expression();

            fields.push((field_name, value));

            // Optional comma
            if matches!(self.current_token(), Token::Comma) {
                self.advance();
            }
        }

        if !matches!(self.current_token(), Token::RightBrace) {
            panic!("Expected '}}' at end of struct literal");
        }
        self.advance();

        Expr::StructLiteral { name, fields }
    }

    fn parse_type(&mut self) -> Type {
        let typ = match self.current_token() {
            Token::TypeStr => Type::Str,
            Token::TypeNumber => Type::Number,
            Token::TypeBool => Type::Bool,
            Token::Identifier(name) => {
                // Custom type (either struct or type alias)
                Type::Custom(name.clone())
            }
            _ => panic!("Expected type annotation, got {:?}", self.current_token()),
        };
        self.advance();
        typ
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

        // Special case: if we just parsed an identifier and the current token is LeftBrace,
        // check if it's actually a struct literal by peeking inside
        if let Expr::Identifier(name) = &expr {
            if matches!(self.current_token(), Token::LeftBrace) {
                // Peek ahead to see if this looks like a struct literal
                // Struct literals have the pattern: { identifier: ...
                // If we see anything else after {, it's not a struct literal
                let next_token = self.peek(1);
                let looks_like_struct = matches!(next_token, Token::Identifier(_));

                if looks_like_struct {
                    // Check if there's a colon after the identifier
                    let after_id = self.peek(2);
                    if matches!(after_id, Token::Colon) {
                        return self.parse_struct_literal(name.clone());
                    }
                }
            }
        }

        loop {
            match self.current_token() {
                Token::LeftParen => {
                    // Function call
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
                }
                Token::Dot => {
                    // Field access or method call
                    self.advance();
                    let field = match self.current_token() {
                        Token::Identifier(name) => name.clone(),
                        _ => panic!("Expected field name after '.'"),
                    };
                    self.advance();

                    // Check if this is a method call (followed by '(')
                    if matches!(self.current_token(), Token::LeftParen) {
                        // Method call: transform to function call with receiver as first arg
                        self.advance(); // consume '('

                        let mut arguments = vec![expr]; // receiver is first argument

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

                        // Create a function call with the method name
                        expr = Expr::Call {
                            callee: Box::new(Expr::Identifier(field)),
                            arguments,
                        };
                    } else {
                        // Regular field access
                        expr = Expr::FieldAccess {
                            object: Box::new(expr),
                            field,
                        };
                    }
                }
                _ => break,
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
            Token::Identifier(name) => {
                // Check if this might be a struct literal
                // We peek ahead to see if there's a LeftBrace after this identifier
                // But we need to be smarter - only treat as struct if we're at statement level
                Expr::Identifier(name)
            }
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

    #[allow(dead_code)]
    fn peek(&self, offset: usize) -> &Token {
        self.tokens
            .get(self.position + offset)
            .unwrap_or(&Token::Eof)
    }
}
