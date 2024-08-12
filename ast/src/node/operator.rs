
#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    And,
    AndAnd,
    Plus,
    Star,
    Slash,
    Or,
    OrOr,
    PlusPlus,
    Minus,
    MinusMinus,
    EqEq,
    Eq,
    Ne,
    Le,
    Ge,
    Lt,
    Gt,
    Not,
}