use std::fmt::Display;

use crate::loc::Loc;

pub struct Token<'src> {
    pub kind: TokenKind<'src>,
    pub loc: Loc,
}

impl<'src> Token<'src> {
    pub fn new(kind: TokenKind<'src>, loc: Loc) -> Self {
        Self { kind, loc }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum TokenKind<'src> {
    // Keywords
    If,     // if
    Then,   // then
    Else,   //else
    Struct, // struct

    // Symbols
    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }
    Comma,  // ,
    Dot,    // .
    Equals, // =
    Colon,  // :

    // Infix Operators
    Add, // +
    Sub, // -
    Mul, // *
    Eq,  // ==
    Neq, // !=
    Lt,  // <
    Gt,  // >
    Le,  // <=
    Ge,  // >=

    // Literals
    Num(i64), // [1-9][0-9]*
    True,
    False,
    Var(&'src str), // [a-zA-Z][a-zA-Z0-9]*

    // Types
    Int,  // int
    Bool, // bool

    Eof,
}

impl<'src> Display for TokenKind<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Add => write!(f, "'+'"),
            TokenKind::Sub => write!(f, "'-'"),
            TokenKind::Mul => write!(f, "'*'"),
            TokenKind::Eq => write!(f, "'=='"),
            TokenKind::Neq => write!(f, "'!='"),
            TokenKind::Lt => write!(f, "'<'"),
            TokenKind::Gt => write!(f, "'>'"),
            TokenKind::Le => write!(f, "'<='"),
            TokenKind::Ge => write!(f, "'>='"),
            TokenKind::If => write!(f, "'if'"),
            TokenKind::Then => write!(f, "'then'"),
            TokenKind::Else => write!(f, "'else'"),
            TokenKind::Struct => write!(f, "'struct'"),
            TokenKind::LParen => write!(f, "'('"),
            TokenKind::RParen => write!(f, "')'"),
            TokenKind::LBrace => write!(f, "'{{'"),
            TokenKind::RBrace => write!(f, "'}}'"),
            TokenKind::Comma => write!(f, "','"),
            TokenKind::Dot => write!(f, "'.'"),
            TokenKind::Equals => write!(f, "'='"),
            TokenKind::Colon => write!(f, "':'"),
            TokenKind::Num(n) => write!(f, "{}", n),
            TokenKind::True => write!(f, "'true'"),
            TokenKind::False => write!(f, "'false'"),
            TokenKind::Var(v) => write!(f, "{}", v),
            TokenKind::Int => write!(f, "'int'"),
            TokenKind::Bool => write!(f, "'bool'"),
            TokenKind::Eof => write!(f, "EOF"),
        }
    }
}
