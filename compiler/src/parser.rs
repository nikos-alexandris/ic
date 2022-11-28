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
                let expr = self.parse_expr()?;
                Some(fl::Definition::new(
                    name,
                    Vec::new().into_boxed_slice(),
                    expr,
                ))
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

                let expr = self.parse_expr()?;

                Some(fl::Definition::new(name, args.into_boxed_slice(), expr))
            }
            _ => {
                self.error(&format!("Expected '=', got {}", self.curr.kind));
                None
            }
        }
    }

    fn parse_expr(&mut self) -> Option<fl::Expr<'src>> {
        match self.curr.kind {
            TokenKind::Var(v) => {
                self.advance()?;
                if self.curr.kind == TokenKind::LParen {
                    self.advance()?;
                    let mut args = Vec::new();
                    while !self.lexer.is_eof() {
                        let arg = self.parse_expr()?;
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

                    Some(fl::Expr::Call(v, args.into_boxed_slice()))
                } else {
                    Some(fl::Expr::Var(v))
                }
            }
            TokenKind::Atom(a) => {
                self.advance()?;
                Some(fl::Expr::Atom(a))
            }
            TokenKind::Num(n) => {
                self.advance()?;
                Some(fl::Expr::Num(n))
            }
            TokenKind::Add => {
                self.advance()?;
                self.expect(TokenKind::LParen)?;
                let lhs = self.parse_expr()?;
                self.expect(TokenKind::Comma)?;
                let rhs = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Some(fl::Expr::Add(Box::new(lhs), Box::new(rhs)))
            }
            TokenKind::Sub => {
                self.advance()?;
                self.expect(TokenKind::LParen)?;
                let lhs = self.parse_expr()?;
                self.expect(TokenKind::Comma)?;
                let rhs = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Some(fl::Expr::Sub(Box::new(lhs), Box::new(rhs)))
            }
            TokenKind::Mul => {
                self.advance()?;
                self.expect(TokenKind::LParen)?;
                let lhs = self.parse_expr()?;
                self.expect(TokenKind::Comma)?;
                let rhs = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Some(fl::Expr::Mul(Box::new(lhs), Box::new(rhs)))
            }
            TokenKind::EqQ => {
                self.advance()?;
                self.expect(TokenKind::LParen)?;
                let lhs = self.parse_expr()?;
                self.expect(TokenKind::Comma)?;
                let rhs = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Some(fl::Expr::Eq(Box::new(lhs), Box::new(rhs)))
            }
            TokenKind::LqQ => {
                self.advance()?;
                self.expect(TokenKind::LParen)?;
                let lhs = self.parse_expr()?;
                self.expect(TokenKind::Comma)?;
                let rhs = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Some(fl::Expr::Lq(Box::new(lhs), Box::new(rhs)))
            }
            TokenKind::PairQ => {
                self.advance()?;
                self.expect(TokenKind::LParen)?;
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Some(fl::Expr::IsPair(Box::new(expr)))
            }
            TokenKind::If => {
                self.advance()?;
                let cond = self.parse_expr()?;
                self.expect(TokenKind::Then)?;
                let then = self.parse_expr()?;
                self.expect(TokenKind::Else)?;
                let els = self.parse_expr()?;
                Some(fl::Expr::If(Box::new(cond), Box::new(then), Box::new(els)))
            }
            TokenKind::Cons => {
                self.advance()?;
                self.expect(TokenKind::LParen)?;
                let lhs = self.parse_expr()?;
                self.expect(TokenKind::Comma)?;
                let rhs = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Some(fl::Expr::Cons(Box::new(lhs), Box::new(rhs)))
            }
            TokenKind::Car => {
                self.advance()?;
                self.expect(TokenKind::LParen)?;
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Some(fl::Expr::Car(Box::new(expr)))
            }
            TokenKind::Cdr => {
                self.advance()?;
                self.expect(TokenKind::LParen)?;
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Some(fl::Expr::Cdr(Box::new(expr)))
            }
            TokenKind::LParen => {
                self.advance()?;
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Some(expr)
            }
            _ => {
                self.error(&format!("Expected expression, got {}", self.curr.kind));
                None
            }
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
