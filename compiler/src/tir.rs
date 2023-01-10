use std::collections::HashMap;

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
}

impl<'src> Function<'src> {
    pub fn new(
        args: Box<[Type<'src>]>,
        arg_names: HashMap<&'src str, usize>,
        ret_ty: Type<'src>,
        body: Expr<'src>,
    ) -> Self {
        Self {
            args,
            arg_names,
            ret_ty,
            body,
        }
    }
}

#[derive(Debug)]
pub struct Struct<'src> {
    pub fields: Box<[Type<'src>]>,
    pub field_names: HashMap<&'src str, usize>,
}

impl<'src> Struct<'src> {
    pub fn new(fields: Box<[Type<'src>]>, field_names: HashMap<&'src str, usize>) -> Self {
        Self {
            fields,
            field_names,
        }
    }
}

#[derive(Debug)]
pub enum Expr<'src> {
    Var(&'src str, Type<'src>),
    Num(i64, Type<'src>),
    Bool(bool, Type<'src>),
    Add(Box<Expr<'src>>, Box<Expr<'src>>, Type<'src>),
    Sub(Box<Expr<'src>>, Box<Expr<'src>>, Type<'src>),
    Mul(Box<Expr<'src>>, Box<Expr<'src>>, Type<'src>),
    Eq(Box<Expr<'src>>, Box<Expr<'src>>, Type<'src>),
    Neq(Box<Expr<'src>>, Box<Expr<'src>>, Type<'src>),
    Lt(Box<Expr<'src>>, Box<Expr<'src>>, Type<'src>),
    Gt(Box<Expr<'src>>, Box<Expr<'src>>, Type<'src>),
    Le(Box<Expr<'src>>, Box<Expr<'src>>, Type<'src>),
    Ge(Box<Expr<'src>>, Box<Expr<'src>>, Type<'src>),
    If(
        Box<Expr<'src>>,
        Box<Expr<'src>>,
        Box<Expr<'src>>,
        Type<'src>,
    ),
    Call(&'src str, Box<[Expr<'src>]>, Type<'src>),
    Field(Box<Expr<'src>>, &'src str, Type<'src>),
    Constructor(&'src str, HashMap<&'src str, Expr<'src>>, Type<'src>),
}

impl<'src> Expr<'src> {
    pub fn ty(&self) -> Type<'src> {
        match self {
            Expr::Var(_, ty) => *ty,
            Expr::Num(_, ty) => *ty,
            Expr::Bool(_, ty) => *ty,
            Expr::Add(_, _, ty) => *ty,
            Expr::Sub(_, _, ty) => *ty,
            Expr::Mul(_, _, ty) => *ty,
            Expr::Eq(_, _, ty) => *ty,
            Expr::Neq(_, _, ty) => *ty,
            Expr::Lt(_, _, ty) => *ty,
            Expr::Gt(_, _, ty) => *ty,
            Expr::Le(_, _, ty) => *ty,
            Expr::Ge(_, _, ty) => *ty,
            Expr::If(_, _, _, ty) => *ty,
            Expr::Call(_, _, ty) => *ty,
            Expr::Field(_, _, ty) => *ty,
            Expr::Constructor(_, _, ty) => *ty,
        }
    }
}
