use tsr_lexer::globals::Positioned;
use crate::node::binary_expression::BinaryExpression;
use crate::node::block::Block;
use crate::node::function::FunctionCallExpression;
use crate::node::ident::Ident;
use crate::node::literal::Literal;
use crate::node::match_case::MatchExpression;
use crate::node::type_parameter::TypeParameter;

#[derive(PartialEq, Debug, Clone)]
pub struct NewExpression {
    pub expression: Box<Positioned<Expression>>,
    pub type_parameters: Vec<Positioned<TypeParameter>>,
    pub arguments: Vec<Positioned<Expression>>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    BinaryExpression(Box<Positioned<BinaryExpression>>),
    IndexExpression(Box<Positioned<IndexExpression>>),
    MatchExpression(Box<Positioned<MatchExpression>>),
    FunctionCallExpression(Box<Positioned<FunctionCallExpression>>),
    NewExpression(Positioned<NewExpression>),
    Block(Block),
    Literal(Positioned<Literal>),
    Ident(Positioned<Ident>),
    Array {
        elements: Vec<Positioned<Expression>>,
        is_dynamic: Positioned<bool>,
    },

    This,
    Null,
}

#[derive(PartialEq, Debug, Clone)]
pub struct IndexExpression {
    pub target: Positioned<Expression>,
    pub index: Positioned<Expression>,
}