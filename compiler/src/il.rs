#[derive(Debug)]
pub struct Program<'src> {
    pub definitions: Box<[Definition<'src>]>,
    pub atoms: Box<[&'src str]>,
}

impl<'src> Program<'src> {
    pub fn new(definitions: Box<[Definition<'src>]>, atoms: Box<[&'src str]>) -> Self {
        Self { definitions, atoms }
    }
}

#[derive(Debug)]
pub struct Definition<'src> {
    pub name: String,
    pub body: Expr<'src>,
}

impl<'src> Definition<'src> {
    pub fn new(name: String, body: Expr<'src>) -> Self {
        Self { name, body }
    }
}

#[derive(Debug)]
pub enum Expr<'src> {
    Var(String),
    Atom(usize),
    Num(i64),
    Add(Box<Expr<'src>>, Box<Expr<'src>>),
    Sub(Box<Expr<'src>>, Box<Expr<'src>>),
    Eq(Box<Expr<'src>>, Box<Expr<'src>>),
    IsPair(Box<Expr<'src>>),
    If(Box<Expr<'src>>, Box<Expr<'src>>, Box<Expr<'src>>),
    Call(&'src str, usize),
    Cons(Box<Expr<'src>>, Box<Expr<'src>>),
    Car(Box<Expr<'src>>),
    Cdr(Box<Expr<'src>>),
    Actuals(Box<[Expr<'src>]>),
}
