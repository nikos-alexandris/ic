use std::fmt::Display;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Type<'src> {
    Int,
    Bool,
    Struct(&'src str),
}

impl<'src> Type<'src> {
    pub fn is_base_type(&self) -> bool {
        match self {
            Self::Int | Self::Bool => true,
            _ => false,
        }
    }
}

impl Display for Type<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Bool => write!(f, "bool"),
            Type::Struct(name) => write!(f, "struct {}", name),
        }
    }
}
