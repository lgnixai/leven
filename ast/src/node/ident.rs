use core::fmt;

#[derive(PartialEq, Hash, Debug, Eq, Clone)]
pub struct Ident(pub String);

impl Ident {
    pub fn new<T: Into<String>>(string: T) -> Ident {
        Ident(string.into())
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}