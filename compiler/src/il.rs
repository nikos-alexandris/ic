pub type Program = Box<[Definition]>;

#[derive(Debug)]
pub struct Definition {
    pub name: String,
    pub body: Expr,
}

impl Definition {
    pub fn new(name: String, body: Expr) -> Self {
        Self { name, body }
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
    Call(u64, String),
    Cons(Box<Expr>, Box<Expr>),
    Car(Box<Expr>),
    Cdr(Box<Expr>),
    Actuals(Box<[Expr]>),
}
