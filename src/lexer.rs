use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    /// Current position in input (points to current char)
    cur: usize,
    /// Next position in input (after current char)
    next: usize,
    /// Current char under examination
    ch: char,
}

const EOF_CHAR: char = '\0';

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input,
            cur: 0,
            next: 0,
            ch: EOF_CHAR,
        };

        lexer.eat_char();

        lexer
    }

    /// Give the next character.
    pub fn peek_char(&mut self) -> char {
        if self.next >= self.input.chars().count() {
            // reached EOF
            EOF_CHAR
        } else {
            self.input.chars().nth(self.next).unwrap_or(EOF_CHAR)
        }
    }

    /// Retrieve the next character and advance position in the input string.
    pub fn eat_char(&mut self) {
        self.ch = self.peek_char();
        self.cur = self.next;
        self.next += 1;
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.eat_char();
        }
    }

    pub fn eat_identifier(&mut self) -> &str {
        let start = self.cur;

        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.eat_char();
        }

        &self.input[start..self.cur]
    }

    pub fn eat_number(&mut self) -> &str {
        let start = self.cur;

        while self.ch.is_ascii_digit() {
            self.eat_char();
        }

        &self.input[start..self.cur]
    }

    /// Retrieve the current token and advance position in the input string.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => Token {
                kind: TokenKind::Assign,
                literal: "=".to_owned(),
            },
            '{' => Token {
                kind: TokenKind::LeftBrace,
                literal: "{".to_owned(),
            },
            '}' => Token {
                kind: TokenKind::RightBrace,
                literal: "}".to_owned(),
            },
            EOF_CHAR => Token {
                kind: TokenKind::Eof,
                literal: "".to_owned(),
            },
            _ => {
                if self.ch.is_alphabetic() || self.ch == '_' {
                    let literal = self.eat_identifier();
                    let kind = TokenKind::lookup_identifier(literal);

                    return Token {
                        kind,
                        literal: literal.to_owned(),
                    };
                } else if self.ch.is_ascii_digit() {
                    let literal = self.eat_number().to_owned();

                    return Token {
                        kind: TokenKind::Integer,
                        literal,
                    };
                } else {
                    Token {
                        kind: TokenKind::Illegal,
                        literal: self.ch.to_string(),
                    }
                }
            }
        };

        self.eat_char();

        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    fn test_tokenization_iter(input: &str, tests: Vec<(TokenKind, &str)>) {
        let mut lexer = Lexer::new(input);

        for (i, (expected_token, expected_literal)) in tests.iter().enumerate() {
            let tok = lexer.next_token();

            assert_eq!(
                &tok.kind, expected_token,
                "Test {} - wrong 'kind'. Expected={:#?}, Got={:#?}",
                i, expected_token, tok.kind
            );

            assert_eq!(
                &tok.literal, expected_literal,
                "Test {} - wrong 'literal'. Expected={}, Got={}",
                i, expected_literal, tok.literal
            );
        }
    }

    #[test]
    fn punct() {
        let input = "={}";

        let tests = vec![
            (TokenKind::Assign, "="),
            (TokenKind::LeftBrace, "{"),
            (TokenKind::RightBrace, "}"),
            (TokenKind::Eof, ""),
        ];

        test_tokenization_iter(input, tests)
    }

    #[test]
    fn kwd() {
        let input = r#"
            scope
            print
        "#;

        let tests = vec![
            (TokenKind::Scope, "scope"),
            (TokenKind::Print, "print"),
            (TokenKind::Eof, ""),
        ];

        test_tokenization_iter(input, tests)
    }
}
