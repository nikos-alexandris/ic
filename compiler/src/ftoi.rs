use std::collections::HashMap;

use crate::fl;
use crate::il;

pub struct FtoI<'src> {
    pub program: fl::Program<'src>,
    var_cnt: u64,
}

#[derive(Debug)]
struct FunctionData<'src> {
    params: HashMap<&'src str, (String, usize)>,
    calls: u64,
    args: Vec<Vec<il::Expr>>,
}

macro_rules! new_var {
    ($self:ident) => {{
        let var = format!("__v{}", $self.var_cnt);
        $self.var_cnt += 1;
        var
    }};
}

impl<'src> FtoI<'src> {
    pub fn new(program: fl::Program<'src>) -> Self {
        Self {
            program,
            var_cnt: 0,
        }
    }

    pub fn convert(mut self) -> il::Program {
        let mut map = self.make_function_data();

        let mut definitions = Vec::new();
        for def in self.program.iter() {
            let body = self.convert_body(&mut map, def.name, &def.body);
            definitions.push(il::Definition::new(def.name.to_string(), body));
        }

        for (_, data) in map.iter_mut() {
            for param in data.params.values() {
                let vec = std::mem::take(&mut data.args[param.1]);
                let body = il::Expr::Actuals(vec.into_boxed_slice());
                definitions.push(il::Definition::new(param.0.clone(), body));
            }
        }

        definitions.into_boxed_slice()
    }

    fn convert_body(
        &self,
        map: &mut HashMap<&'src str, FunctionData<'src>>,
        fun: &'src str,
        body: &fl::Expr<'src>,
    ) -> il::Expr {
        match body {
            fl::Expr::Var(name) => {
                if let Some(v) = map.get(fun).unwrap().params.get(name) {
                    il::Expr::Var(v.0.clone())
                } else if map.contains_key(name) {
                    // nullary variable
                    // TODO distringuish between calls to nullary and n-ary.
                    // No need to track calls to nullary functions.
                    il::Expr::Var(name.to_string())
                } else {
                    // TODO check for this before transforming
                    unreachable!("[{}] Variable not found: {}", fun, name);
                }
            }
            fl::Expr::Atom(name) => il::Expr::Atom(name.to_string()),
            fl::Expr::Num(num) => il::Expr::Num(*num),
            fl::Expr::Add(lhs, rhs) => {
                let lhs = self.convert_body(map, fun, lhs);
                let rhs = self.convert_body(map, fun, rhs);
                il::Expr::Add(Box::new(lhs), Box::new(rhs))
            }
            fl::Expr::Eq(lhs, rhs) => {
                let lhs = self.convert_body(map, fun, lhs);
                let rhs = self.convert_body(map, fun, rhs);
                il::Expr::Eq(Box::new(lhs), Box::new(rhs))
            }
            fl::Expr::IsPair(expr) => {
                let expr = self.convert_body(map, fun, expr);
                il::Expr::IsPair(Box::new(expr))
            }
            fl::Expr::If(cond, then, els) => {
                let cond = self.convert_body(map, fun, cond);
                let then = self.convert_body(map, fun, then);
                let els = self.convert_body(map, fun, els);
                il::Expr::If(Box::new(cond), Box::new(then), Box::new(els))
            }
            fl::Expr::Call(name, args) => {
                let args = args
                    .iter()
                    .map(|arg| self.convert_body(map, fun, arg))
                    .collect::<Vec<il::Expr>>();
                let FunctionData {
                    calls,
                    args: actuals,
                    ..
                } = map.get_mut(*name).unwrap();
                let call = *calls;
                *calls += 1;
                for (i, arg) in args.into_iter().enumerate() {
                    actuals.get_mut(i).unwrap().push(arg);
                }

                il::Expr::Call(call, name.to_string())
            }
            fl::Expr::Cons(lhs, rhs) => {
                let lhs = self.convert_body(map, fun, lhs);
                let rhs = self.convert_body(map, fun, rhs);
                il::Expr::Cons(Box::new(lhs), Box::new(rhs))
            }
            fl::Expr::Car(expr) => {
                let expr = self.convert_body(map, fun, expr);
                il::Expr::Car(Box::new(expr))
            }
            fl::Expr::Cdr(expr) => {
                let expr = self.convert_body(map, fun, expr);
                il::Expr::Cdr(Box::new(expr))
            }
        }
    }

    fn make_function_data(&mut self) -> HashMap<&'src str, FunctionData<'src>> {
        let mut map = HashMap::new();
        for def in self.program.iter() {
            let mut params = HashMap::new();
            for (i, param) in def.args.iter().enumerate() {
                params.insert(*param, (new_var!(self), i));
            }
            map.insert(
                def.name,
                FunctionData {
                    params,
                    calls: 0,
                    args: (0..def.args.len())
                        .map(|_| Vec::new())
                        .collect::<Vec<Vec<il::Expr>>>(),
                },
            );
        }
        map
    }
}
