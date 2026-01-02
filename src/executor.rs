use std::collections::HashMap;

use crate::ast::{Ast, BinaryOp, Expr, Stmt, UnaryOp};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

pub struct Executor {
    ast: Ast,
    variables: HashMap<String, Value>,
}

impl Executor {
    pub fn new(ast: Ast) -> Self {
        Executor {
            ast,
            variables: HashMap::new(),
        }
    }

    pub fn exec(&mut self) {
        let statements = self.ast.statements.clone();
        for statement in &statements {
            self.execute_statement(statement);
        }
    }

    fn execute_statement(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate_expression(expr);
            }
            Stmt::Let { name, value } => {
                let result = self.evaluate_expression(value);
                self.variables.insert(name.clone(), result);
            }
            Stmt::Function { name, params, body: _ } => {
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
                loop {
                    let cond_result = self.evaluate_expression(condition);
                    if !self.is_truthy(&cond_result) {
                        break;
                    }
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

    fn evaluate_expression(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::String(s) => Value::String(s.clone()),
            Expr::Number(n) => Value::Number(*n),
            Expr::Boolean(b) => Value::Boolean(*b),
            Expr::Nil => Value::Nil,
            Expr::Identifier(name) => {
                self.variables.get(name).cloned().unwrap_or_else(|| {
                    eprintln!("Undefined variable: {}", name);
                    Value::Nil
                })
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
            // String concatenation
            (Value::String(l), BinaryOp::Add, Value::String(r)) => {
                Value::String(format!("{}{}", l, r))
            }
            // Number operations
            (Value::Number(l), BinaryOp::Add, Value::Number(r)) => Value::Number(l + r),
            (Value::Number(l), BinaryOp::Subtract, Value::Number(r)) => Value::Number(l - r),
            (Value::Number(l), BinaryOp::Multiply, Value::Number(r)) => Value::Number(l * r),
            (Value::Number(l), BinaryOp::Divide, Value::Number(r)) => Value::Number(l / r),
            (Value::Number(l), BinaryOp::Less, Value::Number(r)) => Value::Boolean(l < r),
            (Value::Number(l), BinaryOp::LessEqual, Value::Number(r)) => Value::Boolean(l <= r),
            (Value::Number(l), BinaryOp::Greater, Value::Number(r)) => Value::Boolean(l > r),
            (Value::Number(l), BinaryOp::GreaterEqual, Value::Number(r)) => Value::Boolean(l >= r),
            // Equality (works for all types)
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
