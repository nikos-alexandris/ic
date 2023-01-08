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
    Cons,  // cons
    Car,   // car
    Cdr,   // cdr
    PairQ, // pair?
    If,    // if
    Then,  // then
    Else,  //else

    // Symbols
    LParen, // (
    RParen, // )
    Comma,  // ,
    Equals, // =

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
    Num(i64),        // [1-9][0-9]*
    Var(&'src str),  // [a-zA-Z][a-zA-Z0-9]*
    Atom(&'src str), // '[a-zA-Z][a-zA-Z0-9]*

    Eof,
}

impl<'src> Display for TokenKind<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Car => write!(f, "'car'"),
            TokenKind::Cdr => write!(f, "'cdr'"),
            TokenKind::PairQ => write!(f, "'pair?'"),
            TokenKind::Cons => write!(f, "':'"),
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
