use std::fmt::Display;

use crate::loc::Loc;

pub struct Token {
    pub kind: TokenKind,
    pub loc: Loc,
}

impl Token {
    pub fn new(kind: TokenKind, loc: Loc) -> Self {
        Self { kind, loc }
    }
}

#[derive(PartialEq, Clone)]
pub enum TokenKind {
    // Keywords
    Cons,  // cons
    Car,   // car
    Cdr,   // cdr
    EqQ,   // eq?
    PairQ, // pair?
    Add,   // add
    If,    // if
    Then,  // then
    Else,  //else

    // Symbols
    LParen,    // (
    RParen,    // )
    Comma,     // ,
    Equals,    // =

    // Literals
    Num(i64),     // [1-9][0-9]*
    Var(String),  // [a-zA-Z][a-zA-Z0-9]*
    Atom(String), // '[a-zA-Z][a-zA-Z0-9]*

    Eof,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Cons => write!(f, "'cons'"),
            TokenKind::Car => write!(f, "'car'"),
            TokenKind::Cdr => write!(f, "'cdr'"),
            TokenKind::EqQ => write!(f, "'eq?'"),
            TokenKind::PairQ => write!(f, "'pair?'"),
            TokenKind::Add => write!(f, "'add'"),
            TokenKind::If => write!(f, "'if'"),
            TokenKind::Then => write!(f, "'then'"),
            TokenKind::Else => write!(f, "'else'"),
            TokenKind::LParen => write!(f, "'('"),
            TokenKind::RParen => write!(f, "')'"),
            TokenKind::Comma => write!(f, "','"),
            TokenKind::Equals => write!(f, "'='"),
            TokenKind::Num(n) => write!(f, "{}", n),
            TokenKind::Var(v) => write!(f, "{}", v),
            TokenKind::Atom(a) => write!(f, "'{}", a),
            TokenKind::Eof => write!(f, "EOF"),
        }
    }
}
