use std::collections::HashMap;

use crate::fl;
use crate::hir;

pub struct FtoH<'src> {
    pub program: fl::Program<'src>,
    // Maps global variables to their index in the program definitions array.
    pub globals: HashMap<&'src str, usize>,
    // Maps function definition indexes to the number of times they have been called.
    pub func_calls: HashMap<usize, usize>,
    pub atoms_map: HashMap<&'src str, usize>,
    pub atom_names: Vec<&'src str>,
}

macro_rules! obf_var {
    ($f:expr, $var:expr) => {
        format!("__{}_{}", $f, $var)
    };
}

impl<'src> FtoH<'src> {
    pub fn new(program: fl::Program<'src>) -> Self {
        Self {
            program,
            atoms_map: HashMap::from([("nil", 0), ("true", 1), ("false", 2)]),
            atom_names: vec!["nil", "true", "false"],
            globals: HashMap::new(),
            func_calls: HashMap::new(),
        }
    }

    pub fn convert(mut self) -> Option<hir::Program<'src>> {
        self.check_duplicate_definitions()?;

        let old_definitions = std::mem::replace(&mut self.program, Box::new([]));

        let mut definitions = Vec::new();
        for def in old_definitions.into_iter() {
            let name = def.name;
            let args = def.args.iter().map(|arg| obf_var!(def.name, arg)).collect();
            let body = self.convert_body(&old_definitions, &def, &def.body)?;
            definitions.push(hir::Definition::new(name, args, body));
        }

        Some(hir::Program::new(
            definitions.into_boxed_slice(),
            self.atom_names.into_boxed_slice(),
        ))
    }

    fn check_duplicate_definitions(&mut self) -> Option<()> {
        for (i, def) in self.program.iter().enumerate() {
            if self.globals.contains_key(def.name) {
                self.error(format!("Duplicate definition: {}", def.name));
                return None;
            }
            self.globals.insert(def.name, i);
            if def.args.len() > 0 {
                self.func_calls.insert(i, 0);
            }
        }
        Some(())
    }

    fn convert_body(
        &mut self,
        definitions: &fl::Program<'src>,
        def: &fl::Definition<'src>,
        body: &fl::Expr<'src>,
    ) -> Option<hir::Expr<'src>> {
        Some(match body {
            fl::Expr::Var(name) => {
                if let Some(_) = def.args.iter().position(|arg| arg == name) {
                    if let Some(idx) = self.globals.get(name) {
                        self.error(format!(
                            "Variable {} shadows global name {}",
                            name, definitions[*idx].name
                        ));
                        return None;
                    }
                    hir::Expr::Local(obf_var!(def.name, name))
                } else if let Some(_) = self.globals.get(name) {
                    hir::Expr::Global(name)
                } else {
                    self.error(format!("Undefined variable: {}", name));
                    return None;
                }
            }
            fl::Expr::Atom(atom_name) => {
                let atom = if let Some(atom) = self.atoms_map.get(atom_name) {
                    *atom
                } else {
                    let atom = self.atoms_map.len();
                    self.atoms_map.insert(atom_name, atom);
                    self.atom_names.push(atom_name);
                    atom
                };
                hir::Expr::Atom(atom)
            }
            fl::Expr::Num(num) => hir::Expr::Num(*num),
            fl::Expr::Add(lhs, rhs) => hir::Expr::Add(
                Box::new(self.convert_body(definitions, def, lhs)?),
                Box::new(self.convert_body(definitions, def, rhs)?),
            ),
            fl::Expr::Eq(lhs, rhs) => hir::Expr::Eq(
                Box::new(self.convert_body(definitions, def, lhs)?),
                Box::new(self.convert_body(definitions, def, rhs)?),
            ),
            fl::Expr::IsPair(expr) => {
                hir::Expr::IsPair(Box::new(self.convert_body(definitions, def, expr)?))
            }
            fl::Expr::If(cond, then, els) => hir::Expr::If(
                Box::new(self.convert_body(definitions, def, cond)?),
                Box::new(self.convert_body(definitions, def, then)?),
                Box::new(self.convert_body(definitions, def, els)?),
            ),
            fl::Expr::Call(name, args) => {
                if let Some(i) = self.globals.get(name) {
                    let callee = &definitions[*i];
                    if callee.args.len() == args.len() {
                        let calls = self.func_calls.get_mut(i).unwrap();
                        let curr = *calls;
                        *calls += 1;

                        hir::Expr::Call(
                            name,
                            args.iter()
                                .map(|arg| self.convert_body(definitions, def, arg))
                                .collect::<Option<Box<[_]>>>()?,
                            curr,
                        )
                    } else {
                        self.error(format!(
                            "Function {} is called with {} arguments, but is of arity {}.",
                            name,
                            args.len(),
                            callee.args.len()
                        ));
                        return None;
                    }
                } else {
                    self.error(format!("Undefined function: {}", name));
                    return None;
                }
            }
            fl::Expr::Cons(lhs, rhs) => hir::Expr::Cons(
                Box::new(self.convert_body(definitions, def, lhs)?),
                Box::new(self.convert_body(definitions, def, rhs)?),
            ),
            fl::Expr::Car(expr) => {
                hir::Expr::Car(Box::new(self.convert_body(definitions, def, expr)?))
            }
            fl::Expr::Cdr(expr) => {
                hir::Expr::Cdr(Box::new(self.convert_body(definitions, def, expr)?))
            }
        })
    }

    fn error<S: AsRef<str>>(&self, message: S) {
        eprintln!("[Error]: {}.", message.as_ref());
    }
}
