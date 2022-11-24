pub type Program = Box<[Definition]>;

#[derive(Debug)]
pub struct Definition {
    pub name: String,
    pub args: Box<[String]>,
    pub body: Expr,
}

impl Definition {
    pub fn new(name: String, args: Box<[String]>, body: Expr) -> Self {
        Self { name, args, body }
    }
}

#[derive(Debug)]
pub enum Expr {
    Var(String),
    Atom(String),
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    IsPair(Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Call(String, Box<[Expr]>),
    Cons(Box<Expr>, Box<Expr>),
    Car(Box<Expr>),
    Cdr(Box<Expr>),
}
