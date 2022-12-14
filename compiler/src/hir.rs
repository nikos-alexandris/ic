// Differences between fl and hir:
//
// 1: Atoms have been converted to numbers, an atoms array
//    has been added, which holds the original atom's name
//    in the same index as the atom's number.
// 2: Function local argument's names have been uniquified.
// 3: Uses of function local variables and global variables
//    have been separated.

use std::collections::HashMap;

#[derive(Debug)]
pub struct Program<'src> {
    pub definitions: Box<[Definition<'src>]>,
    pub var_indices: HashMap<String, usize>,
    pub atoms: Box<[&'src str]>,
}

impl<'src> Program<'src> {
    pub fn new(
        definitions: Box<[Definition<'src>]>,
        var_indices: HashMap<String, usize>,
        atoms: Box<[&'src str]>,
    ) -> Self {
        Self {
            definitions,
            var_indices,
            atoms,
        }
    }
}

#[derive(Debug)]
pub struct Definition<'src> {
    pub name: &'src str,
    pub args: Box<[String]>,
    pub body: Expr<'src>,
}

impl<'src> Definition<'src> {
    pub fn new(name: &'src str, args: Box<[String]>, body: Expr<'src>) -> Self {
        Self { name, args, body }
    }
}

#[derive(Debug)]
pub enum Expr<'src> {
    Local(String),
    Global(&'src str),
    Atom(usize),
    Num(i64),
    Add(Box<Expr<'src>>, Box<Expr<'src>>),
    Sub(Box<Expr<'src>>, Box<Expr<'src>>),
    Mul(Box<Expr<'src>>, Box<Expr<'src>>),
    Eq(Box<Expr<'src>>, Box<Expr<'src>>),
    Neq(Box<Expr<'src>>, Box<Expr<'src>>),
    Lt(Box<Expr<'src>>, Box<Expr<'src>>),
    Gt(Box<Expr<'src>>, Box<Expr<'src>>),
    Le(Box<Expr<'src>>, Box<Expr<'src>>),
    Ge(Box<Expr<'src>>, Box<Expr<'src>>),
    IsPair(Box<Expr<'src>>),
    If(Box<Expr<'src>>, Box<Expr<'src>>, Box<Expr<'src>>),
    Call(&'src str, Box<[Expr<'src>]>, usize),
    Cons(Box<Expr<'src>>, Box<Expr<'src>>, usize),
    Car(Box<Expr<'src>>),
    Cdr(Box<Expr<'src>>),
}
