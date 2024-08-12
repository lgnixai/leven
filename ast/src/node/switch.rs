use tsr_lexer::globals::Positioned;
use crate::node::expression::Expression;
use crate::node::statement::Statement;

#[derive(PartialEq, Debug, Clone)]
pub struct SwitchVariant {
    pub value: Positioned<Expression>,
    pub callback: Positioned<Statement>,
}
