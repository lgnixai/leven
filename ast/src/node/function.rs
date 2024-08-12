use tsr_lexer::globals::Positioned;
use crate::node::block::Block;
use crate::node::expression::Expression;

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionCallExpression {
    pub function: Box<Positioned<Expression>>,
    pub arguments: Vec<Positioned<Expression>>,
    pub lambda: Option<Block>,
}