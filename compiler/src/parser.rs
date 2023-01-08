use std::collections::HashSet;

use crate::{
    fl,
    lexer::Lexer,
    loc::Loc,
    token::{Token, TokenKind},
};

pub struct Parser<'src> {
    lexer: Lexer<'src>,
    curr: Token<'src>,
}

#[derive(PartialEq)]
enum Assoc {
    Left,
    _Right,
}

impl<'src> Parser<'src> {
    pub fn new(lexer: Lexer<'src>) -> Self {
        Self {
            lexer,
            curr: Token::new(TokenKind::Eof, Loc::new(0, 0)),
        }
    }

    pub fn parse(&mut self) -> Option<fl::Program<'src>> {
        self.advance()?;

        let mut defs = Vec::new();
        while !self.lexer.is_eof() {
            let def = self.parse_def()?;
            defs.push(def);
        }
        Some(defs.into_boxed_slice())
    }

    fn parse_def(&mut self) -> Option<fl::Definition<'src>> {
        let name = self.parse_var()?;

        match self.curr.kind {
            TokenKind::Equals => {
                self.advance()?;
                let expr = self.parse_expr(0)?;
                Some(fl::Definition::new(name, Box::new([]), expr))
            }
            TokenKind::LParen => {
                self.advance()?;
                let mut args = Vec::new();
                let mut arg_set = HashSet::new();
                while !self.lexer.is_eof() {
                    let arg = self.parse_var()?;
                    args.push(arg);

                    if arg_set.contains(&arg) {
                        self.error(format!("duplicate argument '{}'", arg));
                    }
                    arg_set.insert(arg);

                    if self.curr.kind == TokenKind::RParen {
                        break;
                    } else if self.curr.kind != TokenKind::Comma {
                        self.error(&format!("Expected ',' or ')', got {}", self.curr.kind));
                        return None;
                    }
                    self.advance()?;
                }

                self.expect(TokenKind::RParen)?;

                self.expect(TokenKind::Equals)?;

                let expr = self.parse_expr(0)?;

                Some(fl::Definition::new(name, args.into_boxed_slice(), expr))
            }
            _ => {
                self.error(&format!("Expected '=', got {}", self.curr.kind));
                None
            }
        }
    }

    fn parse_expr(&mut self, min_prec: u8) -> Option<fl::Expr<'src>> {
        let mut lhs = match self.curr.kind {
            TokenKind::LParen => {
                self.advance()?;
                let expr = self.parse_expr(0)?;
                self.expect(TokenKind::RParen)?;
                expr
            }
            TokenKind::Atom(name) => {
                self.advance()?;
                fl::Expr::Atom(name)
            }
            TokenKind::Num(num) => {
                self.advance()?;
                fl::Expr::Num(num)
            }
            TokenKind::Var(name) => {
                self.advance()?;
                if self.curr.kind == TokenKind::LParen {
                    self.advance()?;
                    let mut args = Vec::new();
                    while !self.lexer.is_eof() {
                        let arg = self.parse_expr(0)?;
                        args.push(arg);

                        if self.curr.kind == TokenKind::RParen {
                            break;
                        } else if self.curr.kind != TokenKind::Comma {
                            self.error(&format!("Expected ',' or ')', got {}", self.curr.kind));
                            return None;
                        }
                        self.advance()?;
                    }

                    self.expect(TokenKind::RParen)?;

                    fl::Expr::Call(name, args.into_boxed_slice())
                } else {
                    fl::Expr::Var(name)
                }
            }
            TokenKind::Cons => {
                self.advance()?;
                self.expect(TokenKind::LParen)?;
                let head = self.parse_expr(0)?;
                self.expect(TokenKind::Comma)?;
                let tail = self.parse_expr(0)?;
                self.expect(TokenKind::RParen)?;
                fl::Expr::Cons(Box::new(head), Box::new(tail))
            }
            TokenKind::Car => {
                self.advance()?;
                self.expect(TokenKind::LParen)?;
                let expr = self.parse_expr(0)?;
                self.expect(TokenKind::RParen)?;
                fl::Expr::Car(Box::new(expr))
            }
            TokenKind::Cdr => {
                self.advance()?;
                self.expect(TokenKind::LParen)?;
                let expr = self.parse_expr(0)?;
                self.expect(TokenKind::RParen)?;
                fl::Expr::Cdr(Box::new(expr))
            }
            TokenKind::PairQ => {
                self.advance()?;
                self.expect(TokenKind::LParen)?;
                let expr = self.parse_expr(0)?;
                self.expect(TokenKind::RParen)?;
                fl::Expr::IsPair(Box::new(expr))
            }
            TokenKind::If => {
                self.advance()?;
                let cond = self.parse_expr(0)?;
                self.expect(TokenKind::Then)?;
                let then = self.parse_expr(0)?;
                self.expect(TokenKind::Else)?;
                let els = self.parse_expr(0)?;
                fl::Expr::If(Box::new(cond), Box::new(then), Box::new(els))
            }
            t => {
                self.error(&format!("Expected expression, got {}", t));
                return None;
            }
        };

        while !self.lexer.is_eof() && Self::is_infix_op(self.curr.kind) {
            let op = self.curr.kind;
            let (prec, assoc) = Self::infix_prec_assoc(op);
            if prec < min_prec {
                break;
            }
            self.advance()?;
            let rhs = match assoc {
                Assoc::Left => self.parse_expr(prec + 1)?,
                Assoc::_Right => self.parse_expr(prec)?,
            };
            lhs = Self::make_infix(op, lhs, rhs);
        }

        Some(lhs)
    }

    fn is_infix_op(t: TokenKind) -> bool {
        matches!(
            t,
            TokenKind::Add
                | TokenKind::Sub
                | TokenKind::Mul
                | TokenKind::Eq
                | TokenKind::Neq
                | TokenKind::Lt
                | TokenKind::Le
                | TokenKind::Gt
                | TokenKind::Ge
        )
    }

    fn infix_prec_assoc(t: TokenKind) -> (u8, Assoc) {
        match t {
            TokenKind::Add | TokenKind::Sub => (1, Assoc::Left),
            TokenKind::Mul => (2, Assoc::Left),
            TokenKind::Eq
            | TokenKind::Neq
            | TokenKind::Lt
            | TokenKind::Le
            | TokenKind::Gt
            | TokenKind::Ge => (3, Assoc::Left),
            _ => unreachable!(),
        }
    }

    fn make_infix(op: TokenKind, lhs: fl::Expr<'src>, rhs: fl::Expr<'src>) -> fl::Expr<'src> {
        let l = Box::new(lhs);
        let r = Box::new(rhs);
        match op {
            TokenKind::Add => fl::Expr::Add(l, r),
            TokenKind::Sub => fl::Expr::Sub(l, r),
            TokenKind::Mul => fl::Expr::Mul(l, r),
            TokenKind::Eq => fl::Expr::Eq(l, r),
            TokenKind::Neq => fl::Expr::Neq(l, r),
            TokenKind::Lt => fl::Expr::Lt(l, r),
            TokenKind::Le => fl::Expr::Le(l, r),
            TokenKind::Gt => fl::Expr::Gt(l, r),
            TokenKind::Ge => fl::Expr::Ge(l, r),
            _ => unreachable!(),
        }
    }

    fn parse_var(&mut self) -> Option<&'src str> {
        match self.curr.kind.clone() {
            TokenKind::Var(name) => {
                self.advance()?;
                Some(name.clone())
            }
            _ => {
                self.error("Expected variable");
                None
            }
        }
    }

    fn expect(&mut self, kind: TokenKind<'src>) -> Option<()> {
        if self.curr.kind == kind {
            self.advance()?;
            Some(())
        } else {
            self.error(&format!("Expected {}, got {}", kind, self.curr.kind));
            None
        }
    }

    fn advance(&mut self) -> Option<()> {
        self.curr = self.lexer.next()?;
        Some(())
    }

    fn error<S: AsRef<str>>(&self, message: S) {
        self.lexer.error(message);
    }
}
