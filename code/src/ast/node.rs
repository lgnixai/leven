#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    // 表达式节点
    Expr(Expr),
    // 语句节点
    Stmt(Stmt),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // 字面量，如数字、字符串、布尔值等
    Literal(Literal),
    // 变量
    Variable(String),
    // 二元运算，如加减乘除
    BinaryOp(Box<Expr>, BinOp, Box<Expr>),
    // 一元运算，如负号
    UnaryOp(UnOp, Box<Expr>),
    // 函数调用
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },


}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f64),
    Bool(bool),
    String(String),
    // 其他字面量类型
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    And,    // &&
    Or,     // ||
    Eq,     // ==
    Neq,    // !=
    Lt,     // <
    Gt,     // >
    Leq,    // <=
    Geq,    // >=
    // 其他运算符
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOp {
    Neg,    // 负号
    Not,    // 逻辑非
}
#[derive(Debug, Clone, PartialEq)]
pub enum DeclarationMode {
    Var,
    Varip,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Color,
    String,
    Line,
    LineFill,
    Label,
    Box,
    Table,
    Array(Box<Type>),
    Matrix(Box<Type>),
    UDF,
    // 其他类型
}
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // 变量声明
    VariableDeclaration {
        declaration_mode: Option<DeclarationMode>,
        var_type: Option<Type>,
        identifier: String,
        value: Expr,
    },
    // 元组声明
    TupleDeclaration {
        identifiers: Vec<String>,
        value: Expr,
    },
    // 表达式语句（如函数调用）
    ExprStmt(Expr),
    // 条件语句
    IfStmt {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    // 循环语句
    WhileStmt {
        condition: Expr,
        body: Vec<Stmt>,
    },
    ForLoop {
        declaration_mode: Option<DeclarationMode>,
        var_type: Option<Type>,
        identifier: String,
        start_expr: Expr,
        end_expr: Expr,
        step_expr: Option<Expr>,
        body: Vec<Stmt>,
        return_value: Option<Expr>,
    },
    // 其他语句类型
}
