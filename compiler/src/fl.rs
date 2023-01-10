// This is the basic ast. Some analyses happens
// during parsing, but most of the simple errors
// are caught later in `flchk.rs`.

use std::collections::HashMap;

use crate::loc::Loc;
use crate::ty::Type;

#[derive(Debug)]
pub struct Program<'src> {
    pub functions: HashMap<&'src str, Function<'src>>,
    pub structs: HashMap<&'src str, Struct<'src>>,
}

impl<'src> Program<'src> {
    pub fn new(
        functions: HashMap<&'src str, Function<'src>>,
        structs: HashMap<&'src str, Struct<'src>>,
    ) -> Self {
        Self { functions, structs }
    }
}

#[derive(Debug)]
pub struct Function<'src> {
    pub args: Box<[Type<'src>]>,
    pub arg_names: HashMap<&'src str, usize>,
    pub ret_ty: Type<'src>,
    pub body: Expr<'src>,
    pub loc: Loc,
}

impl<'src> Function<'src> {
    pub fn new(
        args: Box<[Type<'src>]>,
        arg_names: HashMap<&'src str, usize>,
        ret_ty: Type<'src>,
        body: Expr<'src>,
        loc: Loc,
    ) -> Self {
        Self {
            args,
            arg_names,
            ret_ty,
            body,
            loc,
        }
    }
}

#[derive(Debug)]
pub struct Struct<'src> {
    pub fields: Box<[Type<'src>]>,
    pub field_names: HashMap<&'src str, usize>,
    pub loc: Loc,
}

impl<'src> Struct<'src> {
    pub fn new(
        fields: Box<[Type<'src>]>,
        field_names: HashMap<&'src str, usize>,
        loc: Loc,
    ) -> Self {
        Self {
            fields,
            field_names,
            loc,
        }
    }
}

#[derive(Debug)]
pub enum Expr<'src> {
    Var(&'src str, Loc),
    Num(i64, Loc),
    Bool(bool, Loc),
    Add(Box<Expr<'src>>, Box<Expr<'src>>, Loc),
    Sub(Box<Expr<'src>>, Box<Expr<'src>>, Loc),
    Mul(Box<Expr<'src>>, Box<Expr<'src>>, Loc),
    Eq(Box<Expr<'src>>, Box<Expr<'src>>, Loc),
    Neq(Box<Expr<'src>>, Box<Expr<'src>>, Loc),
    Lt(Box<Expr<'src>>, Box<Expr<'src>>, Loc),
    Gt(Box<Expr<'src>>, Box<Expr<'src>>, Loc),
    Le(Box<Expr<'src>>, Box<Expr<'src>>, Loc),
    Ge(Box<Expr<'src>>, Box<Expr<'src>>, Loc),
    If(Box<Expr<'src>>, Box<Expr<'src>>, Box<Expr<'src>>, Loc),
    Call(&'src str, Box<[Expr<'src>]>, Loc),
    Field(Box<Expr<'src>>, &'src str, Loc),
    Constructor(&'src str, HashMap<&'src str, Expr<'src>>, Loc),
}
