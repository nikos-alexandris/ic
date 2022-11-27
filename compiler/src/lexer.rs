use std::str::Chars;

use phf::phf_map;

use crate::{
    loc::Loc,
    token::{Token, TokenKind},
};

pub struct Lexer<'src> {
    chars: Chars<'src>,
    start_location: Loc,
    end_location: Loc,
}

pub const EOF_CHAR: char = '\0';

static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "cons" => TokenKind::Cons,
    "car" => TokenKind::Car,
    "cdr" => TokenKind::Cdr,
    "eq?" => TokenKind::EqQ,
    "pair?" => TokenKind::PairQ,
    "add" => TokenKind::Add,
    "sub" => TokenKind::Sub,
    "if" => TokenKind::If,
    "then" => TokenKind::Then,
    "else" => TokenKind::Else,
};

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            chars: source.chars(),
            start_location: Loc::new(1, 1),
            end_location: Loc::new(1, 1),
        }
    }

    pub fn next(&mut self) -> Option<Token<'src>> {
        self.skip_whitespace();

        self.start_location = self.end_location;

        if self.is_eof() {
            return Some(self.make_tok(TokenKind::Eof));
        }

        let c = self.first();

        match c {
            '1'..='9' => Some(self.lex_num()),
            'a'..='z' | 'A'..='Z' => self.lex_alpha(),
            '\'' => self.lex_atom(),
            '(' => Some(self.make_single(TokenKind::LParen)),
            ')' => Some(self.make_single(TokenKind::RParen)),
            ',' => Some(self.make_single(TokenKind::Comma)),
            '=' => Some(self.make_single(TokenKind::Equals)),
            _ => {
                self.error(&format!("Unexpected character: {}", c));
                None
            }
        }
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn lex_num(&mut self) -> Token<'src> {
        let start = self.chars.clone();

        self.advance();
        let mut len = 1;
        while !self.is_eof() && self.first().is_digit(10) {
            len += 1;
            self.advance();
        }

        let lexeme = &start.as_str()[..len];
        let num = lexeme.parse().unwrap();

        self.make_tok(TokenKind::Num(num))
    }

    fn lex_alpha(&mut self) -> Option<Token<'src>> {
        let start = self.chars.clone();

        self.advance();
        let mut len = 1;
        while !self.is_eof() && self.first().is_alphanumeric() {
            len += 1;
            self.advance();
        }

        let mut has_q = false;
        if self.first() == '?' {
            len += 1;
            self.advance();
            has_q = true;
        }

        let lexeme = &start.as_str()[..len];

        if let Some(kind) = KEYWORDS.get(lexeme) {
            Some(self.make_tok(*kind))
        } else {
            if has_q {
                self.error("Unexpected character: '?'");
                None
            } else {
                Some(self.make_tok(TokenKind::Var(lexeme)))
            }
        }
    }

    fn lex_atom(&mut self) -> Option<Token<'src>> {
        self.advance();

        if self.is_eof() || !self.first().is_alphabetic() {
            self.error("Expected alphabetic character after '");
            return None;
        }

        let start = self.chars.clone();

        self.advance();
        let mut len = 1;
        while !self.is_eof() && self.first().is_alphanumeric() {
            len += 1;
            self.advance();
        }

        let lexeme = &start.as_str()[..len];

        Some(self.make_tok(TokenKind::Atom(lexeme)))
    }

    fn make_single(&mut self, kind: TokenKind<'src>) -> Token<'src> {
        self.advance();
        self.make_tok(kind)
    }

    fn make_tok(&mut self, kind: TokenKind<'src>) -> Token<'src> {
        Token::new(kind, self.start_location)
    }

    fn skip_whitespace(&mut self) {
        while !self.is_eof() && self.first().is_whitespace() {
            self.advance();
        }
    }

    fn first(&mut self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    fn advance(&mut self) {
        match self.chars.next() {
            Some('\n') => {
                self.end_location.line += 1;
                self.end_location.col = 1;
            }
            Some(_) => {
                self.end_location.col += 1;
            }
            None => {}
        }
    }

    pub fn error<S: AsRef<str>>(&self, message: S) {
        eprintln!(
            "[Parse error][{}]: {}.",
            self.end_location,
            message.as_ref()
        );
    }
}
