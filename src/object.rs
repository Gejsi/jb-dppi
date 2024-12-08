use std::fmt;

use thiserror::Error;

use crate::ast::ParserError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Object {
    IntegerValue(isize),
    NullValue,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::IntegerValue(value) => write!(f, "{value}"),
            Object::NullValue => write!(f, "null"),
        }
    }
}

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("Parsing error: {0}")]
    ParsingError(#[from] ParserError),
}
