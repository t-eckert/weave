// Type annotations
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Str,
    Number,
    Bool,
}

// AST Node types
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
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

    // Struct literal
    StructLiteral {
        name: String,
        fields: Vec<(String, Expr)>,
    },

    // Field access
    FieldAccess {
        object: Box<Expr>,
        field: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
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
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
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

    // Struct definition
    Struct {
        name: String,
        fields: Vec<(String, Type)>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub statements: Vec<Stmt>,
}

impl Ast {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Ast { statements }
    }
}
