use core::fmt;
use std::fmt::Display;
use tsr_lexer::globals::Positioned;
use tsr_lexer::token::Modifier;
use crate::node::array_size::ArraySize;
use crate::node::expression::Expression;
use crate::node::ident::Ident;
use crate::node::literal::Literal;
use crate::node::parameter::Parameter;
use crate::node::type_parameter::TypeParameter;

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    UnionOrIntersectionOrPrimaryType(UnionOrIntersectionOrPrimaryType),
    FunctionType(Vec<TypeParameter>, Vec<Parameter>, Box<Type>),
    ConstructorType(Vec<TypeParameter>, Vec<Parameter>, Box<Type>),
}

impl Default for Type {
    fn default() -> Self {
        PredefinedType::Void.into()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum UnionOrIntersectionOrPrimaryType {
    UnionType(Vec<IntersectionOrPrimaryType>),
    IntersectionOrPrimaryType(IntersectionOrPrimaryType),
}

#[derive(PartialEq, Debug, Clone)]
pub enum IntersectionOrPrimaryType {
    IntersectionType(Vec<PrimaryType>),
    PrimaryType(PrimaryType),
}

#[derive(PartialEq, Debug, Clone)]
pub enum PrimaryType {
    ParenthesizedType(Box<Type>),
    PredefinedType(PredefinedType),
    TypeReference(Ident, Vec<Ident>),
    ObjectType(Vec<TypeMember>),
    ArrayType(Box<PrimaryType>, ArraySize),
    TupleType(Vec<Type>),
    TypeQuery,
    ThisType,
}

#[derive(PartialEq, Debug, Clone)]
pub enum PredefinedType {
    Any,
    Number,
    Float,
    Boolean,
    String,
    StringLiteral(String),
    Symbol,
    Null,
    Void,
}

impl From<PredefinedType> for PrimaryType {
    fn from(value: PredefinedType) -> Self {
        PrimaryType::PredefinedType(value)
    }
}

impl From<PredefinedType> for Type {
    fn from(value: PredefinedType) -> Self {
        Self::UnionOrIntersectionOrPrimaryType(
            UnionOrIntersectionOrPrimaryType::IntersectionOrPrimaryType(
                IntersectionOrPrimaryType::PrimaryType(PrimaryType::PredefinedType(value)),
            ),
        )
    }
}

impl From<PrimaryType> for Type {
    fn from(value: PrimaryType) -> Self {
        Self::UnionOrIntersectionOrPrimaryType(
            UnionOrIntersectionOrPrimaryType::IntersectionOrPrimaryType(
                IntersectionOrPrimaryType::PrimaryType(value),
            ),
        )
    }
}

impl From<Vec<PrimaryType>> for Type {
    fn from(value: Vec<PrimaryType>) -> Self {
        Self::UnionOrIntersectionOrPrimaryType(
            UnionOrIntersectionOrPrimaryType::IntersectionOrPrimaryType(
                IntersectionOrPrimaryType::IntersectionType(value),
            ),
        )
    }
}

impl fmt::Display for PredefinedType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PredefinedType::Any => write!(f, "any"),
            PredefinedType::Number => write!(f, "number"),
            PredefinedType::Float => write!(f, "float"),
            PredefinedType::Boolean => write!(f, "boolean"),
            PredefinedType::String => write!(f, "string"),
            PredefinedType::StringLiteral(literal) => write!(f, "\"{literal}\""),
            PredefinedType::Symbol => write!(f, "symbol"),
            PredefinedType::Null => write!(f, "null"),
            PredefinedType::Void => write!(f, "void"),
        }
    }
}

impl fmt::Display for PrimaryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimaryType::ParenthesizedType(ty) => write!(f, "({})", **ty),
            PrimaryType::PredefinedType(ty) => ty.fmt(f),
            PrimaryType::TypeReference(name, generics) => {
                let result = write!(f, "{name}");

                if !generics.is_empty() {
                    write!(f, "<")?;

                    let last = generics.last();

                    for generic in generics {
                        generic.fmt(f)?;

                        if !last.is_some_and(|g| g == generic) {
                            write!(f, ", ")?;
                        }
                    }

                    write!(f, ">")?;
                }

                result
            }
            PrimaryType::ObjectType(properties) => {
                write!(f, "{{ ")?;

                let last = properties.last();

                for member in properties {
                    match member {
                        TypeMember::PropertySignature(signature) => {
                            let PropertySignature {
                                modifiers,
                                name,
                                nullable,
                                ty,
                            } = &signature.value;

                            let modifiers = modifiers
                                .iter()
                                .map(|m| match m.value {
                                    Modifier::Public => "public",
                                    Modifier::Private => "private",
                                    Modifier::Protected => "protected",
                                    Modifier::Async => "async",
                                    Modifier::Static => "static",
                                })
                                .collect::<Vec<_>>();
                            let modifiers = if !modifiers.is_empty() {
                                format!("{} ", modifiers.join(", "))
                            } else {
                                "".into()
                            };

                            let name = &name.value;
                            let nullable = match nullable.value {
                                true => "?",
                                false => "",
                            };
                            let ty = &ty.value;

                            write!(f, "{modifiers}{name}{nullable}: {ty}")?;
                        }
                        TypeMember::CallSignature(_) => todo!(),
                        TypeMember::ConstructSignature(_) => todo!(),
                        TypeMember::IndexSignature(signature) => {
                            let signature = &signature.value;

                            write!(
                                f,
                                "[{}: {}]: {}",
                                signature.0.value, signature.1.value, signature.2.value
                            )?;
                        }
                        TypeMember::MethodSignature(_) => todo!(),
                    }

                    if !last.is_some_and(|p| p == member) {
                        write!(f, ", ")?;
                    }
                }

                write!(f, " }}")
            }
            PrimaryType::ArrayType(ty, size) => {
                ty.fmt(f)?;

                write!(f, "[")?;

                let size = match size {
                    ArraySize::Fixed(size) => size.to_string(),
                    ArraySize::Dynamic => "".into(),
                };

                write!(f, "{size}")?;

                write!(f, "]")
            }
            PrimaryType::TupleType(tuple) => {
                write!(f, "[")?;

                let last = tuple.last();

                for ty in tuple {
                    ty.fmt(f)?;

                    if !last.is_some_and(|t| t == ty) {
                        write!(f, ", ")?;
                    }
                }

                write!(f, "]")
            }
            PrimaryType::TypeQuery => write!(f, "unknown"),
            PrimaryType::ThisType => write!(f, "this"),
        }
    }
}

impl fmt::Display for IntersectionOrPrimaryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntersectionOrPrimaryType::IntersectionType(types) => {
                let last = types.last();

                for ty in types {
                    ty.fmt(f)?;

                    if !last.is_some_and(|t| t == ty) {
                        write!(f, " & ")?;
                    }
                }

                Ok(())
            }
            IntersectionOrPrimaryType::PrimaryType(ty) => ty.fmt(f),
        }
    }
}

impl fmt::Display for TypeParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = write!(f, "{}", self.name.value);

        if let Some(constraint) = &self.constraint {
            return write!(f, " extends {}", constraint.value);
        }

        if let Some(default) = &self.default {
            return write!(f, " = {}", default.value);
        }

        result
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.value)?;

        if self.nullable.value {
            write!(f, "?")?;
        }

        let result = write!(f, ": {}", self.ty.value);

        if let Some(default) = &self.default {
            return write!(f, " = {:?}", default.value);
        }

        result
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::UnionOrIntersectionOrPrimaryType(ty) => match ty {
                UnionOrIntersectionOrPrimaryType::UnionType(types) => {
                    let last = types.last();

                    for ty in types {
                        ty.fmt(f)?;

                        if !last.is_some_and(|t| t == ty) {
                            write!(f, " | ")?;
                        }
                    }

                    Ok(())
                }
                UnionOrIntersectionOrPrimaryType::IntersectionOrPrimaryType(ty) => ty.fmt(f),
            },
            Type::FunctionType(generics, parameters, ty) => {
                if !generics.is_empty() {
                    write!(f, "<")?;

                    let last = generics.last();

                    for generic in generics {
                        generic.fmt(f)?;

                        if !last.is_some_and(|g| g == generic) {
                            write!(f, ", ")?;
                        }
                    }

                    write!(f, ">")?;
                }

                write!(f, "(")?;

                if !parameters.is_empty() {
                    let last = parameters.last();

                    for parameter in parameters {
                        parameter.fmt(f)?;

                        if !last.is_some_and(|p| p == parameter) {
                            write!(f, ", ")?;
                        }
                    }
                }

                write!(f, ") => {ty}")
            }
            Type::ConstructorType(generics, parameters, ty) => {
                write!(f, "new")?;

                if !generics.is_empty() {
                    write!(f, "<")?;

                    let last = generics.last();

                    for generic in generics {
                        generic.fmt(f)?;

                        if !last.is_some_and(|g| g == generic) {
                            write!(f, ", ")?;
                        }
                    }

                    write!(f, ">")?;
                }

                write!(f, "(")?;

                if !parameters.is_empty() {
                    let last = parameters.last();

                    for parameter in parameters {
                        parameter.fmt(f)?;

                        if !last.is_some_and(|p| p == parameter) {
                            write!(f, ", ")?;
                        }
                    }
                }

                write!(f, "): {ty}")
            }
        }
    }
}


#[derive(PartialEq, Debug, Clone)]
pub enum TypeMember {
    PropertySignature(Positioned<PropertySignature>),
    CallSignature(Positioned<CallSignature>),
    ConstructSignature(Positioned<ConstructSignature>),
    IndexSignature(Positioned<IndexSignature>),
    MethodSignature(Positioned<MethodSignature>),
}


#[derive(PartialEq, Debug, Clone)]
pub struct PropertySignature {
    pub modifiers: Vec<Positioned<Modifier>>,
    pub name: Positioned<Ident>,
    pub nullable: Positioned<bool>,
    pub ty: Positioned<Type>,
}


#[derive(PartialEq, Debug, Clone)]
pub struct CallSignature(
    pub Vec<Positioned<TypeParameter>>,
    pub Vec<Positioned<Parameter>>,
    pub Option<Positioned<Type>>,
);

#[derive(PartialEq, Debug, Clone)]
pub struct ConstructSignature(
    pub Vec<Positioned<TypeParameter>>,
    pub Vec<Positioned<Parameter>>,
    pub  Option<Positioned<Type>>,
);

#[derive(PartialEq, Debug, Clone)]
pub struct IndexSignature(
    pub Positioned<Ident>,
    pub Positioned<Type>,
    pub Positioned<Type>,
);

#[derive(PartialEq, Debug, Clone)]
pub struct MethodSignature(
    pub Positioned<PropertyName>,
    pub Positioned<bool>,
    pub Box<Positioned<CallSignature>>,
);


#[derive(PartialEq, Debug, Clone)]
pub enum PropertyName {
    LiteralPropertyName(Positioned<Literal>),
    ComputedPropertyName(Positioned<Expression>),
}
