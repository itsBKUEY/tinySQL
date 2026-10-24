use crate::error::{Result, Span};

#[derive(Debug, Clone, PartialEq)]

pub enum TokenKind {
//keyword for tokens
    Select, From, Where, Insert, Into, Values, Create, Table,
    // literals and names
    Ident(String), Int(i64), Str(String),
    // symbols
    Comma, Semicolon, LParen, RParen, Star,
    Plus, Minus, Slash,
    Eq, NotEq, Lt, LtEq, Gt, GtEq, Concat,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token { pub kind: TokenKind, pub span: Span }

pub struct Lexer<'a> { /* chars, position, line, col */ }

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self { todo!() }
    pub fn tokenize(&mut self) -> Result<Vec<Token>> { todo!() }
}