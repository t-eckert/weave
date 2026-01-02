use std::collections::HashMap;

use crate::ast::{Ast, BinaryOp, Expr, Stmt, Type, UnaryOp};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
    Struct {
        type_name: String,
        fields: HashMap<String, Value>,
    },
}

#[derive(Debug, Clone)]
struct StructDef {
    fields: Vec<(String, Type)>,
}

#[derive(Debug, Clone)]
struct TypeAlias {
    variants: Vec<String>,
}

#[derive(Debug, Clone)]
struct Function {
    params: Vec<(String, Option<Type>)>,
    return_type: Option<Type>,
    body: Vec<Stmt>,
}

pub struct Executor {
    ast: Ast,
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
    structs: HashMap<String, StructDef>,
    type_aliases: HashMap<String, TypeAlias>,
}

impl Executor {
    pub fn new(ast: Ast) -> Self {
        Executor {
            ast,
            variables: HashMap::new(),
            functions: HashMap::new(),
            structs: HashMap::new(),
            type_aliases: HashMap::new(),
        }
    }

    pub fn exec(&mut self) {
        let statements = self.ast.statements.clone();
        for statement in &statements {
            self.execute_statement(statement);
        }
    }

    fn execute_statement(&mut self, stmt: &Stmt) -> Option<Value> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate_expression(expr);
                None
            }
            Stmt::Let { name, value } => {
                let result = self.evaluate_expression(value);
                self.variables.insert(name.clone(), result);
                None
            }
            Stmt::Function {
                name,
                params,
                return_type,
                body,
            } => {
                let func = Function {
                    params: params.clone(),
                    return_type: return_type.clone(),
                    body: body.clone(),
                };
                self.functions.insert(name.clone(), func);
                None
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_result = self.evaluate_expression(condition);
                if self.is_truthy(&cond_result) {
                    for stmt in then_branch {
                        if let Some(return_val) = self.execute_statement(stmt) {
                            return Some(return_val);
                        }
                    }
                } else if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        if let Some(return_val) = self.execute_statement(stmt) {
                            return Some(return_val);
                        }
                    }
                }
                None
            }
            Stmt::While { condition, body } => {
                loop {
                    let cond_result = self.evaluate_expression(condition);
                    if !self.is_truthy(&cond_result) {
                        break;
                    }
                    for stmt in body {
                        if let Some(return_val) = self.execute_statement(stmt) {
                            return Some(return_val);
                        }
                    }
                }
                None
            }
            Stmt::Return(value) => {
                if let Some(expr) = value {
                    let result = self.evaluate_expression(expr);
                    Some(result)
                } else {
                    Some(Value::Nil)
                }
            }
            Stmt::Block(statements) => {
                for stmt in statements {
                    if let Some(return_val) = self.execute_statement(stmt) {
                        return Some(return_val);
                    }
                }
                None
            }
            Stmt::Struct { name, fields } => {
                let struct_def = StructDef {
                    fields: fields.clone(),
                };
                self.structs.insert(name.clone(), struct_def);
                None
            }
            Stmt::TypeAlias { name, variants } => {
                let type_alias = TypeAlias {
                    variants: variants.clone(),
                };
                self.type_aliases.insert(name.clone(), type_alias);
                None
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

                        // Bind parameters to arguments with type checking
                        for ((param_name, param_type), value) in
                            func.params.iter().zip(arg_values.iter())
                        {
                            // Type check if type annotation exists
                            if let Some(expected_type) = param_type {
                                if !self.type_matches(value, expected_type) {
                                    eprintln!(
                                        "Type mismatch for parameter '{}' in function '{}': expected {:?}, got {:?}",
                                        param_name, name, expected_type, value
                                    );
                                    return Value::Nil;
                                }
                            }
                            self.variables.insert(param_name.clone(), value.clone());
                        }

                        // Execute function body and capture return value
                        let mut return_value = Value::Nil;
                        for stmt in &func.body {
                            if let Some(ret_val) = self.execute_statement(stmt) {
                                return_value = ret_val;
                                break;
                            }
                        }

                        // Restore variables
                        self.variables = saved_vars;

                        return return_value;
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
            Expr::StructLiteral { name, fields } => {
                // Get struct definition
                let struct_def = self.structs.get(name).cloned().unwrap_or_else(|| {
                    eprintln!("Undefined struct: {}", name);
                    std::process::exit(1);
                });

                // Create a HashMap for field values
                let mut field_values = HashMap::new();

                // Check that all defined fields are provided and type-check them
                for (field_name, field_type) in &struct_def.fields {
                    // Find the field in the provided fields
                    let field_value = fields
                        .iter()
                        .find(|(name, _)| name == field_name)
                        .map(|(_, expr)| self.evaluate_expression(expr));

                    match field_value {
                        Some(value) => {
                            // Type check
                            if !self.type_matches(&value, field_type) {
                                eprintln!(
                                    "Type mismatch for field '{}': expected {:?}, got {:?}",
                                    field_name, field_type, value
                                );
                                std::process::exit(1);
                            }
                            field_values.insert(field_name.clone(), value);
                        }
                        None => {
                            eprintln!("Missing field '{}' in struct {}", field_name, name);
                            std::process::exit(1);
                        }
                    }
                }

                // Check for extra fields
                for (provided_field, _) in fields {
                    if !struct_def
                        .fields
                        .iter()
                        .any(|(name, _)| name == provided_field)
                    {
                        eprintln!(
                            "Unknown field '{}' in struct {}",
                            provided_field, name
                        );
                        std::process::exit(1);
                    }
                }

                Value::Struct {
                    type_name: name.clone(),
                    fields: field_values,
                }
            }
            Expr::FieldAccess { object, field } => {
                let obj_value = self.evaluate_expression(object);
                match obj_value {
                    Value::Struct {
                        type_name: _,
                        fields,
                    } => fields.get(field).cloned().unwrap_or_else(|| {
                        eprintln!("Field '{}' not found on struct", field);
                        std::process::exit(1);
                    }),
                    _ => {
                        eprintln!("Cannot access field on non-struct value");
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    fn type_matches(&self, value: &Value, expected_type: &Type) -> bool {
        match (value, expected_type) {
            (Value::String(_), Type::Str) => true,
            (Value::Number(_), Type::Number) => true,
            (Value::Boolean(_), Type::Bool) => true,
            (Value::String(s), Type::Custom(type_name)) => {
                // Check if it's a type alias (union type)
                if let Some(type_alias) = self.type_aliases.get(type_name) {
                    // For string literal unions, check if value is in variants
                    type_alias.variants.contains(s)
                } else {
                    // Not a type alias, might be trying to use a struct type for a string
                    false
                }
            }
            (
                Value::Struct {
                    type_name,
                    fields: _,
                },
                Type::Custom(expected_type),
            ) => {
                // Check that the struct's type matches the expected type
                type_name == expected_type
            }
            (Value::String(s), Type::Union(variants)) => {
                // Direct union type check
                variants.contains(s)
            }
            _ => false,
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
            Value::Struct {
                type_name: _,
                fields,
            } => {
                let field_strs: Vec<String> = fields
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, self.value_to_string(v)))
                    .collect();
                format!("{{ {} }}", field_strs.join(", "))
            }
        }
    }
}
