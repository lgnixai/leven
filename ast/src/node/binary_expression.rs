use tsr_lexer::globals::Positioned;
use crate::node::expression::Expression;
use crate::node::operator::Operator;

#[derive(PartialEq, Debug, Clone)]
pub struct BinaryExpression {
    pub left: Positioned<Expression>,
    pub operator: Positioned<Operator>,
    pub right: Positioned<Expression>,
}
