use std::fmt;

use crate::interning::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Token {
    pub data: TokenData,
    pub span: Span,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenData {
    LParen,
    RParen,
    LBrack,
    RBrack,
    LBrace,
    RBrace,

    Ident(String),

    // TODO: add all tokens
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Span {
    pub start: Position,
    pub end:   Option<Position>,
}

impl Span {
    pub const fn point(point: Position) -> Self {
        Span {
            start: point,
            end:   None,
        }
    }

    pub const fn new(start: Position, end: Position) -> Self {
        Span {
            start,
            end: Some(end),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Position {
    pub lin: u32,
    pub col: u32,
}

impl Position {
    pub const fn new(lin: u32, col: u32) -> Self {
        Position { lin, col }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.lin, self.col)
    }
}
