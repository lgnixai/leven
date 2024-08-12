use tsr_lexer::globals::Positioned;
use crate::node::ident::Ident;
use crate::node::data_type::Type;

#[derive(PartialEq, Debug, Clone)]
pub struct TypeParameter {
    pub name: Positioned<Ident>,
    pub constraint: Option<Positioned<Type>>,
    pub default: Option<Positioned<Type>>,
}
