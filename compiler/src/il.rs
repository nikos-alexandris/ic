// It is worth noting that at this stage
// we have obfuscated local variables, so
// we can be sure that when we pull them
// to the global scope we will have no
// conflicts. For some local variable
// v in some function f, the variable
// is renamed to f__v. We can be sure
// that there will be no conflicts
// because
// 1. we don't allow '_' in names
// 2. we have checked for duplicate
//    function definitions
// 3. we have checked for duplicate
//    local definitions
// The same thing is true for structs.

use std::collections::HashMap;

use crate::ty::Type;

#[derive(Debug)]
pub struct Program<'src> {
    pub functions: HashMap<String, Function<'src>>,
    pub structs: HashMap<&'src str, Struct<'src>>,
}

impl<'src> Program<'src> {
    pub fn new(
        functions: HashMap<String, Function<'src>>,
        structs: HashMap<&'src str, Struct<'src>>,
    ) -> Self {
        Self { functions, structs }
    }
}

#[derive(Debug)]
pub struct Function<'src> {
    pub args: Box<[Type<'src>]>, // TODO: maybe don't need this field
    pub arg_names: HashMap<String, usize>, // We need these for the transformation
    pub ret_ty: Type<'src>,
    pub body: Expr<'src>,
    pub is_function: bool,
}

impl<'src> Function<'src> {
    pub fn new(
        args: Box<[Type<'src>]>,
        arg_names: HashMap<String, usize>,
        ret_ty: Type<'src>,
        body: Expr<'src>,
        is_function: bool,
    ) -> Self {
        Self {
            args,
            arg_names,
            ret_ty,
            body,
            is_function,
        }
    }
}

#[derive(Debug)]
pub struct Struct<'src> {
    pub fields: Box<[Type<'src>]>,
    pub field_names: HashMap<String, usize>,
}

impl<'src> Struct<'src> {
    pub fn new(fields: Box<[Type<'src>]>, field_names: HashMap<String, usize>) -> Self {
        Self {
            fields,
            field_names,
        }
    }
}

#[derive(Debug)]
pub enum Expr<'src> {
    // A local variable is, at the time, a
    // parameter. The second field is its
    // position in its function's parameter
    // list
    // TODO maybe we don't need the first field
    Local(&'src str, usize, Type<'src>),
    // A global variable is just a call to
    // a nullary function in the source
    // program.
    Global(&'src str, Type<'src>),
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
    // The second field is the call's index.
    Call(&'src str, usize, Type<'src>),
    // The second field is the position of
    // the field in the struct's field list.
    Field(Box<Expr<'src>>, usize, Type<'src>),
    Constructor(&'src str, usize, Type<'src>),
}

impl<'src> Expr<'src> {
    pub fn ty(&self) -> Type<'src> {
        match self {
            Expr::Local(_, _, ty) => *ty,
            Expr::Global(_, ty) => *ty,
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
