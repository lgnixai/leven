use tsr_lexer::globals::Positioned;

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    String(Positioned<String>),
    Number(Positioned<i64>),
    Float(Positioned<f64>),
    Boolean(Positioned<bool>),
}