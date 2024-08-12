use tsr_lexer::globals::Positioned;
use crate::node::expression::Expression;
use crate::node::switch::SwitchVariant;

#[derive(PartialEq, Debug, Clone)]
pub struct MatchExpression {
    pub target: Positioned<Expression>,
    pub variants: Vec<Positioned<SwitchVariant>>,
}
