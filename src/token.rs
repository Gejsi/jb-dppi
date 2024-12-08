use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Illegal,
    Eof,

    Identifier,
    Integer,

    Assign,

    LeftBrace,
    RightBrace,

    Scope,
    Print,
}

impl TokenKind {
    /// Matches keywords.
    pub fn lookup_identifier(identifier: &str) -> TokenKind {
        match identifier {
            "scope" => TokenKind::Scope,
            "print" => TokenKind::Print,
            _ => TokenKind::Identifier,
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Illegal => write!(f, "illegal"),
            TokenKind::Eof => write!(f, "eof"),

            TokenKind::Identifier => write!(f, "identifier"),
            TokenKind::Integer => write!(f, "integer"),

            TokenKind::Assign => write!(f, "="),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),

            TokenKind::Scope => write!(f, "scope"),
            TokenKind::Print => write!(f, "print"),
        }
    }
}
