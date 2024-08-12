use tsr_lexer::globals::Positioned;
use crate::node::data_type::Type;
use crate::node::expression::Expression;
use crate::node::ident::Ident;

#[derive(PartialEq, Debug, Clone)]
pub struct Parameter {
    pub name: Positioned<Ident>,
    pub nullable: Positioned<bool>,
    pub ty: Positioned<Type>,
    pub default: Option<Positioned<Expression>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ArrowParameter {
    pub name: Positioned<Ident>,
    pub nullable: Positioned<bool>,
    pub ty: Option<Positioned<Type>>,
    pub default: Option<Positioned<Expression>>,
}
