use crate::ast::identifier::Identifier;
use crate::ast::node::Parameter;
use crate::input::Positioned;


#[derive(PartialEq, Debug, Clone)]
pub struct TypeParameter {
    pub name: Positioned<Identifier>,
    pub constraint: Option<Positioned<Type>>,
    pub default: Option<Positioned<Type>>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    FunctionType(Vec<TypeParameter>, Vec<Parameter>, Box<Type>),
}

impl Default for Type {
    fn default() -> Self {
        PredefinedType::Void.into()
    }
}