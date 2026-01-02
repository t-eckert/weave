use std::collections::HashMap;

use crate::ast::{Ast, BinaryOp, Expr, Stmt, UnaryOp};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone)]
struct Function {
    params: Vec<String>,
    body: Vec<Stmt>,
}

pub struct Executor {
    ast: Ast,
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
}

impl Executor {
    pub fn new(ast: Ast) -> Self {
        Executor {
            ast,
            variables: HashMap::new(),
            functions: HashMap::new(),
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
            Stmt::Function { name, params, body } => {
                let func = Function {
                    params: params.clone(),
                    body: body.clone(),
                };
                self.functions.insert(name.clone(), func);
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
                        let mut output = String::new();
                        for arg in arguments {
                            let value = self.evaluate_expression(arg);
                            output.push_str(&self.value_to_string(&value));
                        }
                        println!("{}", output);
                        return Value::Nil;
                    }

                    // User-defined functions
                    if let Some(func) = self.functions.get(name).cloned() {
                        // Evaluate arguments
                        let mut arg_values = Vec::new();
                        for arg in arguments {
                            arg_values.push(self.evaluate_expression(arg));
                        }

                        // Check parameter count
                        if arg_values.len() != func.params.len() {
                            eprintln!(
                                "Function '{}' expects {} arguments, got {}",
                                name,
                                func.params.len(),
                                arg_values.len()
                            );
                            return Value::Nil;
                        }

                        // Save current variables
                        let saved_vars = self.variables.clone();

                        // Bind parameters to arguments
                        for (param, value) in func.params.iter().zip(arg_values.iter()) {
                            self.variables.insert(param.clone(), value.clone());
                        }

                        // Execute function body
                        for stmt in &func.body {
                            self.execute_statement(stmt);
                        }

                        // Restore variables
                        self.variables = saved_vars;

                        return Value::Nil;
                    }

                    // Unknown function
                    eprintln!("Undefined function: {}", name);
                    return Value::Nil;
                }

                // For non-identifier callees, just print for now
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
