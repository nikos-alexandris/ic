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
    pub name: String,
    pub args: Box<[String]>, // We need this field for generating the lars
    pub body: Expr<'src>,
    pub is_function: bool,
}

impl<'src> Definition<'src> {
    pub fn new(name: String, args: Box<[String]>, body: Expr<'src>, is_function: bool) -> Self {
        Self {
            name,
            args,
            body,
            is_function,
        }
    }
}

#[derive(Debug)]
pub enum Expr<'src> {
    Var(String),
    Atom(usize),
    Num(i64),
    Add(Box<Expr<'src>>, Box<Expr<'src>>),
    Sub(Box<Expr<'src>>, Box<Expr<'src>>),
    Mul(Box<Expr<'src>>, Box<Expr<'src>>),
    Eq(Box<Expr<'src>>, Box<Expr<'src>>),
    Lq(Box<Expr<'src>>, Box<Expr<'src>>),
    IsPair(Box<Expr<'src>>),
    If(Box<Expr<'src>>, Box<Expr<'src>>, Box<Expr<'src>>),
    Call(&'src str, usize),
    Cons(usize),
    Car(Box<Expr<'src>>),
    Cdr(Box<Expr<'src>>),
}
