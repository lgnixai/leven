use tsr_lexer::globals::Positioned;
use crate::node::expression::Expression;
use crate::node::ident::Ident;

#[derive(PartialEq, Debug, Clone)]
pub struct EnumDeclaration {
    pub name: Positioned<Ident>,
   // pub members: Vec<Positioned<EnumMember>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct EnumMember {
    pub name: Positioned<Ident>,
    pub initializer: Option<Positioned<Expression>>,
}
