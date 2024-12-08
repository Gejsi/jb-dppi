use std::rc::Rc;

use crate::{
    ast::{Expression, ParserError, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub cur: Rc<Token>,
    pub next: Rc<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input);

        let mut parser = Self {
            lexer,
            cur: Rc::new(Token {
                kind: TokenKind::Eof,
                literal: "".to_owned(),
            }),
            next: Rc::new(Token {
                kind: TokenKind::Eof,
                literal: "".to_owned(),
            }),
        };

        // consume two tokens to set `cur` and `next` correctly
        parser.eat_token();
        parser.eat_token();

        parser
    }

    pub fn eat_token(&mut self) {
        /*
            This is like doing...
            ```
            self.cur = self.next;
            self.next = self.lexer.next_token().into();
            ```
            ...but respecting the borrow checker.
        */
        self.cur = std::mem::replace(&mut self.next, self.lexer.next_token().into());
    }

    pub fn expect_token(&mut self, token_kind: TokenKind) -> Result<Rc<Token>, ParserError> {
        if self.next.kind != token_kind {
            return Err(ParserError::UnexpectedToken(self.next.clone()));
        }

        self.eat_token();
        Ok(self.cur.clone())
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut statements: Vec<Statement> = vec![];

        while self.cur.kind != TokenKind::Eof {
            statements.push(self.parse_statement()?);
            self.eat_token();
        }

        Ok(Program(statements))
    }

    pub fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.cur.kind {
            TokenKind::Scope => self.parse_block_statement(),

            TokenKind::Print => self.parse_print_statement(),

            TokenKind::Identifier => {
                if self.next.kind == TokenKind::Assign {
                    self.parse_assign_statement()
                } else {
                    self.parse_expression_statement()
                }
            }
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_assign_statement(&mut self) -> Result<Statement, ParserError> {
        let name = self.cur.literal.clone();
        self.expect_token(TokenKind::Assign)?;
        let expr = self.parse_expression(false)?;

        Ok(Statement::AssignStatement { name, value: expr })
    }

    pub fn parse_block_statement(&mut self) -> Result<Statement, ParserError> {
        // consume keyword
        self.eat_token();

        if self.cur.kind != TokenKind::LeftBrace {
            return Err(ParserError::UnexpectedToken(self.cur.clone()));
        }

        // consume left brace
        self.eat_token();
        let mut statements: Vec<Statement> = vec![];

        while self.cur.kind != TokenKind::RightBrace {
            let statement = self.parse_statement()?;
            statements.push(statement);
            self.eat_token();
        }

        Ok(Statement::BlockStatement(statements))
    }

    pub fn parse_print_statement(&mut self) -> Result<Statement, ParserError> {
        // consume keyword
        self.eat_token();
        let expr = self.parse_expression(true)?;

        Ok(Statement::PrintStatement(expr))
    }

    pub fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expr = self.parse_expression(true)?;

        Ok(Statement::ExpressionStatement(expr))
    }

    /// `skip_eating` - skip the initial token eating. Useful for parsing *expression statements*.
    pub fn parse_expression(&mut self, skip_eating: bool) -> Result<Expression, ParserError> {
        if !skip_eating {
            self.eat_token();
        }

        let expr = match self.cur.kind {
            TokenKind::Integer => Expression::IntegerLiteral(self.cur.literal.parse::<isize>()?),
            TokenKind::Identifier => Expression::Identifier(self.cur.literal.clone()),

            _ => {
                return Err(ParserError::UnexpectedToken(self.cur.clone()));
            }
        };

        Ok(expr)
    }
}
