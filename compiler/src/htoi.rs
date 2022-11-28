use std::collections::HashMap;

use crate::hir;
use crate::il;

pub struct HtoI<'src> {
    pub program: hir::Program<'src>,
    func_idx: HashMap<&'src str, usize>,
}

impl<'src> HtoI<'src> {
    pub fn new(program: hir::Program<'src>) -> Self {
        Self {
            program,
            func_idx: HashMap::new(),
        }
    }

    pub fn convert(mut self) -> il::Program<'src> {
        let mut actuals = self.make_indices();

        let mut definitions = self
            .program
            .definitions
            .iter()
            .map(|def| {
                il::Definition::new(
                    def.name.to_string(),
                    self.convert_expr(&mut actuals, &def.body),
                )
            })
            .collect();

        self.make_actuals(&mut actuals, &mut definitions);

        il::Program::new(definitions.into_boxed_slice(), self.program.atoms)
    }

    fn make_indices(&mut self) -> Vec<Vec<Vec<il::Expr<'src>>>> {
        let mut actuals = Vec::new();
        let mut idx = 0;
        for def in self.program.definitions.iter() {
            if def.args.len() == 0 {
                continue;
            }
            self.func_idx.insert(def.name, idx);
            idx += 1;

            let mut vec = Vec::new();
            for _ in 0..def.args.len() {
                vec.push(Vec::new());
            }
            actuals.push(vec);
        }
        actuals
    }

    fn make_actuals(
        &mut self,
        actuals: &mut Vec<Vec<Vec<il::Expr<'src>>>>,
        definitions: &mut Vec<il::Definition<'src>>,
    ) {
        self.program
            .definitions
            .iter()
            .filter(|def| def.args.len() > 0)
            .zip(actuals.into_iter())
            .for_each(|(def, actuals)| {
                for (j, arg) in def.args.iter().enumerate() {
                    definitions.push(il::Definition::new(
                        arg.to_string(),
                        il::Expr::Actuals(
                            std::mem::replace(&mut actuals[j], Vec::new()).into_boxed_slice(),
                        ),
                    ));
                }
            });
    }

    fn convert_expr(
        &self,
        actuals: &mut Vec<Vec<Vec<il::Expr<'src>>>>,
        expr: &hir::Expr<'src>,
    ) -> il::Expr<'src> {
        match expr {
            hir::Expr::Local(name) => il::Expr::Var(name.clone()),
            hir::Expr::Global(name) => il::Expr::Var(name.to_string()),
            hir::Expr::Atom(index) => il::Expr::Atom(*index),
            hir::Expr::Num(num) => il::Expr::Num(*num),
            hir::Expr::Add(left, right) => il::Expr::Add(
                Box::new(self.convert_expr(actuals, left)),
                Box::new(self.convert_expr(actuals, right)),
            ),
            hir::Expr::Sub(left, right) => il::Expr::Sub(
                Box::new(self.convert_expr(actuals, left)),
                Box::new(self.convert_expr(actuals, right)),
            ),
            hir::Expr::Mul(left, right) => il::Expr::Mul(
                Box::new(self.convert_expr(actuals, left)),
                Box::new(self.convert_expr(actuals, right)),
            ),
            hir::Expr::Eq(left, right) => il::Expr::Eq(
                Box::new(self.convert_expr(actuals, left)),
                Box::new(self.convert_expr(actuals, right)),
            ),
            hir::Expr::Lq(left, right) => il::Expr::Lq(
                Box::new(self.convert_expr(actuals, left)),
                Box::new(self.convert_expr(actuals, right)),
            ),
            hir::Expr::IsPair(expr) => il::Expr::IsPair(Box::new(self.convert_expr(actuals, expr))),
            hir::Expr::If(cond, then, els) => il::Expr::If(
                Box::new(self.convert_expr(actuals, cond)),
                Box::new(self.convert_expr(actuals, then)),
                Box::new(self.convert_expr(actuals, els)),
            ),
            hir::Expr::Call(name, args, i) => {
                for (idx, arg) in args.iter().enumerate() {
                    let expr = self.convert_expr(actuals, arg);
                    actuals[self.func_idx[name]][idx].push(expr);
                }
                il::Expr::Call(name, *i)
            }
            hir::Expr::Cons(left, right) => il::Expr::Cons(
                Box::new(self.convert_expr(actuals, left)),
                Box::new(self.convert_expr(actuals, right)),
            ),
            hir::Expr::Car(expr) => il::Expr::Car(Box::new(self.convert_expr(actuals, expr))),
            hir::Expr::Cdr(expr) => il::Expr::Cdr(Box::new(self.convert_expr(actuals, expr))),
        }
    }
}
