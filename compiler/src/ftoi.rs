use std::collections::HashMap;

use crate::fl;
use crate::il;

pub struct FtoI {
    pub program: fl::Program,
    var_cnt: u64,
}

macro_rules! new_var {
    ($self:ident) => {{
        let var = format!("__v{}", $self.var_cnt);
        $self.var_cnt += 1;
        var
    }};
}

impl FtoI {
    pub fn new(program: fl::Program) -> Self {
        Self {
            program,
            var_cnt: 0,
        }
    }

    pub fn convert(mut self) -> il::Program {
        let mut actuals_map = self.make_actuals_map();

        let mut definitions = Vec::new();
        for def in self.program.iter() {
            let body = Self::convert_body(&mut actuals_map, &def.name, &def.body);
            definitions.push(il::Definition::new(def.name.clone(), body));
        }

        for (_, (_, actuals)) in actuals_map {
            for (name, exprs) in actuals {
                definitions.push(il::Definition::new(
                    name,
                    il::Expr::Actuals(exprs.into_boxed_slice()),
                ));
            }
        }

        definitions.into_boxed_slice()
    }

    fn convert_body(
        actuals_map: &mut HashMap<String, (u64, Vec<(String, Vec<il::Expr>)>)>,
        fun: &String,
        body: &fl::Expr,
    ) -> il::Expr {
        match body {
            fl::Expr::Var(name) => il::Expr::Var(name.clone()),
            fl::Expr::Atom(name) => il::Expr::Atom(name.clone()),
            fl::Expr::Num(num) => il::Expr::Num(*num),
            fl::Expr::Add(lhs, rhs) => {
                let lhs = Self::convert_body(actuals_map, fun, lhs);
                let rhs = Self::convert_body(actuals_map, fun, rhs);
                il::Expr::Add(Box::new(lhs), Box::new(rhs))
            }
            fl::Expr::Eq(lhs, rhs) => {
                let lhs = Self::convert_body(actuals_map, fun, lhs);
                let rhs = Self::convert_body(actuals_map, fun, rhs);
                il::Expr::Eq(Box::new(lhs), Box::new(rhs))
            }
            fl::Expr::IsPair(expr) => {
                let expr = Self::convert_body(actuals_map, fun, expr);
                il::Expr::IsPair(Box::new(expr))
            }
            fl::Expr::If(cond, then, els) => {
                let cond = Self::convert_body(actuals_map, fun, cond);
                let then = Self::convert_body(actuals_map, fun, then);
                let els = Self::convert_body(actuals_map, fun, els);
                il::Expr::If(Box::new(cond), Box::new(then), Box::new(els))
            }
            fl::Expr::Call(name, args) => {
                let args = args
                    .iter()
                    .map(|arg| Self::convert_body(actuals_map, fun, arg))
                    .collect::<Vec<il::Expr>>();
                let (cnt, actuals) = actuals_map.get_mut(name).unwrap();
                let call = *cnt;
                *cnt += 1;
                for (i, arg) in args.into_iter().enumerate() {
                    actuals.get_mut(i).unwrap().1.push(arg);
                }

                il::Expr::Call(call, name.clone())
            }
            fl::Expr::Cons(lhs, rhs) => {
                let lhs = Self::convert_body(actuals_map, fun, lhs);
                let rhs = Self::convert_body(actuals_map, fun, rhs);
                il::Expr::Cons(Box::new(lhs), Box::new(rhs))
            }
            fl::Expr::Car(expr) => {
                let expr = Self::convert_body(actuals_map, fun, expr);
                il::Expr::Car(Box::new(expr))
            }
            fl::Expr::Cdr(expr) => {
                let expr = Self::convert_body(actuals_map, fun, expr);
                il::Expr::Cdr(Box::new(expr))
            }
        }
    }

    fn make_actuals_map(&mut self) -> HashMap<String, (u64, Vec<(String, Vec<il::Expr>)>)> {
        let mut actuals_map = HashMap::new();
        for def in self.program.iter() {
            let actuals = def
                .args
                .iter()
                .map(|_| (new_var!(self), Vec::new()))
                .collect();
            actuals_map.insert(def.name.clone(), (0, actuals));
        }
        actuals_map
    }
}
