use std::{
    fmt,
    num::{ParseIntError, TryFromIntError},
    rc::Rc,
};

use thiserror::Error;

use crate::token::Token;

#[derive(Debug)]
pub struct Program(pub Vec<Statement>);

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for statement in &self.0 {
            write!(f, "{statement}")?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Statement {
    AssignStatement { name: String, value: Expression },

    ExpressionStatement(Expression),

    PrintStatement(Expression),

    BlockStatement(Vec<Statement>),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::AssignStatement { name, value } => write!(f, "{name} = {value}"),
            Statement::ExpressionStatement(expr) => write!(f, "{expr}"),
            Statement::PrintStatement(expr) => write!(f, "print {expr}"),
            Statement::BlockStatement(statements) => {
                write!(f, "{{")?;
                for statement in statements {
                    write!(f, "{}", statement)?;
                }
                write!(f, "}}")
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Identifier(String),

    IntegerLiteral(isize),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Identifier(s) => write!(f, "{s}"),
            Expression::IntegerLiteral(n) => write!(f, "{n}"),
        }
    }
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unexpected token: {0:#?}")]
    UnexpectedToken(Rc<Token>),

    #[error("Failed to parse to a word-sized integer: {0}")]
    ParseIntError(#[from] ParseIntError),

    #[error("Conversion to int failed: {0}")]
    IntConversionError(#[from] TryFromIntError),
}
