// Parses the source code into an `fl`.
// Checks for duplicate function and struct
// definitions, duplicate parameter and filed
// definitions and for duplicate field assignments
// in constructors.

use std::collections::HashMap;

use crate::ty::Type;
use crate::{
    fl,
    lexer::Lexer,
    loc::Loc,
    token::{Token, TokenKind},
};

pub fn parse<'src>(source: &'src str) -> Option<fl::Program<'src>> {
    Parser::new(Lexer::new(source)).parse()
}

struct Parser<'src> {
    lexer: Lexer<'src>,
    curr: Token<'src>,
}

#[derive(PartialEq)]
enum Assoc {
    Left,
    _Right,
}

enum DefResult<'src> {
    Function(&'src str, fl::Function<'src>),
    Struct(&'src str, fl::Struct<'src>),
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

        let mut functions = HashMap::new();
        let mut structs = HashMap::new();

        while !self.lexer.is_eof() {
            let def = self.parse_def()?;
            match def {
                DefResult::Function(name, f) => {
                    if functions.contains_key(name) {
                        self.error("Duplicate function definition");
                        return None;
                    }
                    if structs.contains_key(name) {
                        self.error("Function name is the same as a struct name");
                        return None;
                    }
                    functions.insert(name, f);
                }
                DefResult::Struct(name, s) => {
                    if structs.contains_key(name) {
                        self.error("Duplicate struct definition");
                        return None;
                    }
                    if functions.contains_key(name) {
                        self.error("Struct name is the same as a function name");
                        return None;
                    }
                    structs.insert(name, s);
                }
            }
        }

        Some(fl::Program::new(functions, structs))
    }

    fn parse_def(&mut self) -> Option<DefResult<'src>> {
        match self.curr.kind {
            TokenKind::Var(name) => {
                let loc = self.curr.loc;
                self.advance()?;
                let f = self.parse_function(loc)?;
                Some(DefResult::Function(name, f))
            }
            TokenKind::Struct => {
                self.advance()?;
                let loc = self.curr.loc;
                let name = self.parse_var()?;
                let s = self.parse_struct(loc)?;
                Some(DefResult::Struct(name, s))
            }
            _ => {
                self.error("expected function or struct definition");
                None
            }
        }
    }

    fn parse_function(&mut self, loc: Loc) -> Option<fl::Function<'src>> {
        match self.curr.kind {
            TokenKind::Colon => {
                self.advance()?;
                let ty = self.parse_ty()?;
                self.expect(TokenKind::Equals)?;
                let expr = self.parse_expr(0)?;
                Some(fl::Function::new(
                    Box::new([]),
                    HashMap::new(),
                    ty,
                    expr,
                    loc,
                ))
            }
            TokenKind::LParen => {
                self.advance()?;
                let mut args = Vec::new();
                let mut arg_names = HashMap::new();
                while !self.lexer.is_eof() {
                    let arg = self.parse_var()?;

                    if arg_names.contains_key(arg) {
                        self.error("Duplicate argument name");
                        return None;
                    }
                    arg_names.insert(arg, args.len());

                    self.expect(TokenKind::Colon)?;
                    let ty = self.parse_ty()?;
                    args.push(ty);

                    if self.curr.kind == TokenKind::RParen {
                        break;
                    } else if self.curr.kind != TokenKind::Comma {
                        self.error(&format!("Expected ',' or ')', got {}", self.curr.kind));
                        return None;
                    }
                    self.advance()?;
                }
                self.expect(TokenKind::RParen)?;
                self.expect(TokenKind::Colon)?;
                let ty = self.parse_ty()?;
                self.expect(TokenKind::Equals)?;

                let expr = self.parse_expr(0)?;

                Some(fl::Function::new(
                    args.into_boxed_slice(),
                    arg_names,
                    ty,
                    expr,
                    loc,
                ))
            }
            _ => {
                self.error(&format!("Expected ':' or '(', got {}", self.curr.kind));
                None
            }
        }
    }

    fn parse_struct(&mut self, loc: Loc) -> Option<fl::Struct<'src>> {
        self.expect(TokenKind::LBrace)?;

        let mut fields = Vec::new();
        let mut field_names = HashMap::new();
        while !self.lexer.is_eof() {
            let field = self.parse_var()?;

            self.expect(TokenKind::Colon)?;
            let ty = self.parse_ty()?;

            if field_names.contains_key(field) {
                self.error("Duplicate field definition");
                return None;
            }
            field_names.insert(field, fields.len());

            fields.push(ty);

            if self.curr.kind == TokenKind::RBrace {
                break;
            } else if self.curr.kind != TokenKind::Comma {
                self.error(&format!("Expected ',' or '}}', got {}", self.curr.kind));
                return None;
            }
            self.advance()?;
        }

        self.expect(TokenKind::RBrace)?;

        Some(fl::Struct::new(fields.into_boxed_slice(), field_names, loc))
    }

    fn parse_expr(&mut self, min_prec: u8) -> Option<fl::Expr<'src>> {
        let loc = self.curr.loc;
        let mut lhs = match self.curr.kind {
            TokenKind::LParen => {
                self.advance()?;
                let expr = self.parse_expr(0)?;
                self.expect(TokenKind::RParen)?;
                expr
            }
            TokenKind::Num(num) => {
                self.advance()?;
                fl::Expr::Num(num, loc)
            }
            TokenKind::True => {
                self.advance()?;
                fl::Expr::Bool(true, loc)
            }
            TokenKind::False => {
                self.advance()?;
                fl::Expr::Bool(false, loc)
            }
            TokenKind::Var(name) => {
                self.advance()?;
                match self.curr.kind {
                    TokenKind::LParen => {
                        let loc = self.curr.loc;
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

                        fl::Expr::Call(name, args.into_boxed_slice(), loc)
                    }
                    TokenKind::LBrace => {
                        let loc = self.curr.loc;
                        self.advance()?;
                        let mut fields = HashMap::new();
                        while !self.lexer.is_eof() {
                            let field = self.parse_var()?;

                            self.expect(TokenKind::Equals)?;
                            let expr = self.parse_expr(0)?;

                            if fields.contains_key(field) {
                                self.error("Duplicate field assignment");
                                return None;
                            }

                            fields.insert(field, expr);

                            if self.curr.kind == TokenKind::RBrace {
                                break;
                            } else if self.curr.kind != TokenKind::Comma {
                                self.error(&format!(
                                    "Expected ',' or '}}', got {}",
                                    self.curr.kind
                                ));
                                return None;
                            }
                            self.advance()?;
                        }

                        self.expect(TokenKind::RBrace)?;

                        fl::Expr::Constructor(name, fields, loc)
                    }
                    _ => fl::Expr::Var(name, loc),
                }
            }
            TokenKind::If => {
                self.advance()?;
                let cond = self.parse_expr(0)?;
                self.expect(TokenKind::Then)?;
                let then = self.parse_expr(0)?;
                self.expect(TokenKind::Else)?;
                let els = self.parse_expr(0)?;
                fl::Expr::If(Box::new(cond), Box::new(then), Box::new(els), loc)
            }
            t => {
                self.error(&format!("Expected expression, got {}", t));
                return None;
            }
        };

        while !self.lexer.is_eof() && Self::is_infix_op(self.curr.kind) {
            let op = self.curr.kind;
            let op_loc = self.curr.loc;
            let (prec, assoc) = Self::infix_prec_assoc(op);
            if prec < min_prec {
                break;
            }
            self.advance()?;

            match op {
                TokenKind::Dot => {
                    let rhs = self.parse_var()?;
                    lhs = fl::Expr::Field(Box::new(lhs), rhs, op_loc);
                }
                _ => {
                    let rhs = match assoc {
                        Assoc::Left => self.parse_expr(prec + 1)?,
                        Assoc::_Right => self.parse_expr(prec)?,
                    };
                    lhs = Self::make_infix(op, lhs, rhs, op_loc);
                }
            }
        }

        Some(lhs)
    }

    fn parse_ty(&mut self) -> Option<Type<'src>> {
        match self.curr.kind {
            TokenKind::Int => {
                self.advance()?;
                Some(Type::Int)
            }
            TokenKind::Bool => {
                self.advance()?;
                Some(Type::Bool)
            }
            TokenKind::Var(name) => {
                self.advance()?;
                Some(Type::Struct(name))
            }
            _ => {
                self.error(&format!("Expected type, got {}", self.curr.kind));
                None
            }
        }
    }

    fn is_infix_op(t: TokenKind) -> bool {
        use TokenKind::*;
        matches!(t, Add | Sub | Mul | Eq | Neq | Lt | Le | Gt | Ge | Dot)
    }

    fn infix_prec_assoc(t: TokenKind) -> (u8, Assoc) {
        use Assoc::*;
        use TokenKind::*;
        match t {
            Eq | Neq => (1, Left),
            Lt | Le | Gt | Ge => (2, Left),
            Add | Sub => (3, Left),
            Mul => (4, Left),
            Dot => (5, _Right),
            _ => unreachable!(),
        }
    }

    fn make_infix(
        op: TokenKind,
        lhs: fl::Expr<'src>,
        rhs: fl::Expr<'src>,
        op_loc: Loc,
    ) -> fl::Expr<'src> {
        let l = Box::new(lhs);
        let r = Box::new(rhs);
        match op {
            TokenKind::Add => fl::Expr::Add(l, r, op_loc),
            TokenKind::Sub => fl::Expr::Sub(l, r, op_loc),
            TokenKind::Mul => fl::Expr::Mul(l, r, op_loc),
            TokenKind::Eq => fl::Expr::Eq(l, r, op_loc),
            TokenKind::Neq => fl::Expr::Neq(l, r, op_loc),
            TokenKind::Lt => fl::Expr::Lt(l, r, op_loc),
            TokenKind::Le => fl::Expr::Le(l, r, op_loc),
            TokenKind::Gt => fl::Expr::Gt(l, r, op_loc),
            TokenKind::Ge => fl::Expr::Ge(l, r, op_loc),
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
