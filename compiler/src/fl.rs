pub type Program<'src> = Box<[Definition<'src>]>;

#[derive(Debug)]
pub struct Definition<'src> {
    pub name: &'src str,
    pub args: Box<[&'src str]>,
    pub body: Expr<'src>,
}

impl<'src> Definition<'src> {
    pub fn new(name: &'src str, args: Box<[&'src str]>, body: Expr<'src>) -> Self {
        Self { name, args, body }
    }
}

#[derive(Debug)]
pub enum Expr<'src> {
    Var(&'src str),
    Atom(&'src str),
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
    Call(&'src str, Box<[Expr<'src>]>),
    Cons(Box<Expr<'src>>, Box<Expr<'src>>),
    Car(Box<Expr<'src>>),
    Cdr(Box<Expr<'src>>),
}
