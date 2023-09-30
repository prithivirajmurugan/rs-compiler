use crate::compilation_unit::FunctionIdx;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Bool,
    Void,
    Function(FunctionIdx),
    Unresolved,
    Error,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let type_name = match self {
            Type::Int => "int",
            Type::Bool => "bool",
            Type::Unresolved => "unresolved",
            Type::Void => "void",
            Type::Function(_) => "function",
            Type::Error => "?",
        };
        write!(f, "{}", type_name)
    }
}

impl Type {
    pub fn is_assignable_to(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Int, Type::Int) => true,
            (Type::Bool, Type::Bool) => true,
            (Type::Error, _) => true,
            (_, Type::Error) => true,
            _ => false,
        }
    }
    pub fn from_str(s: &str) -> Option<Type> {
        match s {
            "int" => Some(Type::Int),
            "bool" => Some(Type::Bool),
            "void" => Some(Type::Void),
            _ => None,
        }
    }
}
