use crate::ast::identifier::Identifier;
use crate::ast::enums::EnumDeclaration;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    // 表达式节点
    Expr(Expr),
    // 语句节点
    Stmt(Stmt),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    String(String),
    Variable(String),
    Boolean(bool),
    Integer(i64),
    Double(f64),

    // 其他字面量类型
}

impl From<Atom> for Expr {
    fn from(atom: Atom) -> Self {
        Expr::Atom(atom)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Stmt>,
    pub return_expr: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Body {
    pub stmts: Vec<Stmt>,
}



impl Body {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self { stmts }
    }

    // pub fn with_return_self(&mut self, self_node_id: NodeId) {
    //     let identifier = Identifier::new("self".to_string(), self_node_id);
    //     let operand = Operand::new_identifier(identifier);
    //     // FIXME: should have a valid node_id ?
    //     let primary = PrimaryExpr::new(0, operand, vec![]);
    //     let unary = UnaryExpr::PrimaryExpr(primary);
    //     let expr = Expression::new_unary(unary);
    //     let stmt = Statement::new_expression(expr);
    //
    //     self.stmts.push(stmt);
    // }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Atom(Atom),
    // 字面量，如数字、字符串、布尔值等
    Literal(Literal),
    // 变量
    Variable(String),
    // 二元运算，如加减乘除
    BinaryOp(Box<Expr>, BinaryOperation, Box<Expr>),

    // 一元运算，如负号
    UnaryOp(UnOp, Box<Expr>),
    // 函数调用
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    Na

}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f64),
    Bool(bool),
    Integer(i64),
    Double(f64),
    String(String),
    Variable(String),

    // 其他字面量类型
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperation {
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
    NotEqual,
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
#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub(crate) name: String,                        // 函数名称
    pub(crate) params: Vec<Parameter>,              // 参数列表
    pub body: Body,
}

// 参数定义
#[derive(Debug, PartialEq,Clone)]
pub struct Parameter {
    pub(crate) name: String,                        // 参数名称
    pub(crate) default_value: Option<Expr>,         // 可选的默认值
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub(crate) declaration_mode: Option<DeclarationMode>,
    pub(crate) var_type: Option<Type>,
    pub(crate) identifier: Identifier,
    pub(crate) value: Expr,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Return(Expr),
    Enum(EnumDeclaration),
    Variable(Variable),
    Original(String),
    // 变量声明
    // VariableDeclaration {
    //     declaration_mode: Option<DeclarationMode>,
    //     var_type: Option<Type>,
    //     //identifier: Identifier,
    //     value: Expr,
    // },

    // FunctionDeclaration {
    //     name: String,                        // 函数名称
    //     params: Vec<Stmt::Parameter>,              // 参数列表
    //     body: Vec<Stmt>,                     // 函数体，包含多个语句
    // },
    // Parameter {
    //     name: String,                        // 参数名称
    //     default_value: Option<Expr>,         // 可选的默认值
    // },
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
