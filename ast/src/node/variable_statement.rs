use tsr_lexer::globals::Positioned;
use crate::node::expression::Expression;
use crate::node::ident::Ident;
use crate::node::data_type::Type;

#[derive(PartialEq, Debug, Clone)]
pub struct VariableStatement {
    pub mutable: Positioned<bool>,
    pub declarations: Vec<Positioned<VariableDeclaration>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct VariableDeclaration {
    pub name: Positioned<Ident>,
    pub ty: Option<Positioned<Type>>,
    pub nullable: Positioned<bool>,
    pub initializer: Option<Positioned<Expression>>,
}