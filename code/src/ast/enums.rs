
#[derive(Debug, Clone, PartialEq)]

pub struct EnumDeclaration {
    pub(crate) name: String,
    //pub(crate) fields: Vec<EnumField>,
}

#[derive(Debug, Clone, PartialEq)]

pub struct EnumField {
    pub(crate) name: String,
    pub(crate) title: Option<String>,
}