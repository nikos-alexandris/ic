use std::collections::HashMap;

use crate::loc::Loc;
use crate::ty::Type;
use crate::{fl, tir};

pub fn typecheck(program: fl::Program) -> Option<tir::Program> {
    let structs = make_structs(&program)?;
    let functions = make_functions(&program)?;
    Some(tir::Program::new(functions, structs))
}

fn make_structs<'src>(
    program: &fl::Program<'src>,
) -> Option<HashMap<&'src str, tir::Struct<'src>>> {
    Some(
        program
            .structs
            .iter()
            .map(|(name, s)| {
                (
                    *name,
                    tir::Struct::new(s.fields.clone(), s.field_names.clone()),
                )
            })
            .collect::<HashMap<_, _>>(),
    )
}

fn make_functions<'src>(
    program: &fl::Program<'src>,
) -> Option<HashMap<&'src str, tir::Function<'src>>> {
    program
        .functions
        .iter()
        .map(|(name, f)| {
            let te = make_expr(program, f, &f.body);
            match te {
                Some(te) => {
                    if te.ty() == f.ret_ty {
                        Some((
                            *name,
                            tir::Function::new(f.args.clone(), f.arg_names.clone(), f.ret_ty, te),
                        ))
                    } else {
                        error(
                            format!(
                                "Function {} has return type {} but returns {}",
                                name,
                                f.ret_ty,
                                te.ty()
                            ),
                            &f.loc,
                        );
                        None
                    }
                }
                None => return None,
            }
        })
        .collect::<Option<HashMap<_, _>>>()
}

fn make_expr<'src>(
    program: &fl::Program<'src>,
    function: &fl::Function<'src>,
    expr: &fl::Expr<'src>,
) -> Option<tir::Expr<'src>> {
    match expr {
        fl::Expr::Var(name, _) => {
            if let Some(&idx) = function.arg_names.get(name) {
                return Some(tir::Expr::Var(name, function.args[idx]));
            }
            Some(tir::Expr::Var(
                name,
                program.functions.get(name).unwrap().ret_ty,
            ))
        }
        fl::Expr::Num(n, ..) => Some(tir::Expr::Num(*n, Type::Int)),
        fl::Expr::Bool(b, ..) => Some(tir::Expr::Bool(*b, Type::Bool)),
        fl::Expr::Add(e1, e2, loc) => {
            let te1 = make_expr(program, function, e1)?;
            let te2 = make_expr(program, function, e2)?;
            if te1.ty() == Type::Int && te2.ty() == Type::Int {
                Some(tir::Expr::Add(Box::new(te1), Box::new(te2), Type::Int))
            } else {
                error(format!("Cannot add {} and {}", te1.ty(), te2.ty()), loc);
                None
            }
        }
        fl::Expr::Sub(e1, e2, loc) => {
            let te1 = make_expr(program, function, e1)?;
            let te2 = make_expr(program, function, e2)?;
            if te1.ty() == Type::Int && te2.ty() == Type::Int {
                Some(tir::Expr::Sub(Box::new(te1), Box::new(te2), Type::Int))
            } else {
                error(
                    format!("Cannot subtract {} and {}", te1.ty(), te2.ty()),
                    loc,
                );
                None
            }
        }
        fl::Expr::Mul(e1, e2, loc) => {
            let te1 = make_expr(program, function, e1)?;
            let te2 = make_expr(program, function, e2)?;
            if te1.ty() == Type::Int && te2.ty() == Type::Int {
                Some(tir::Expr::Mul(Box::new(te1), Box::new(te2), Type::Int))
            } else {
                error(
                    format!("Cannot multiply {} and {}", te1.ty(), te2.ty()),
                    loc,
                );
                None
            }
        }
        fl::Expr::Eq(e1, e2, loc) => {
            let te1 = make_expr(program, function, e1)?;
            let te2 = make_expr(program, function, e2)?;
            if te1.ty() == te2.ty() && te1.ty().is_base_type() {
                Some(tir::Expr::Eq(Box::new(te1), Box::new(te2), Type::Bool))
            } else {
                error(
                    format!(
                        "Cannot compare {} and {} (can only compare same *base* types with '==')",
                        te1.ty(),
                        te2.ty()
                    ),
                    loc,
                );
                None
            }
        }
        fl::Expr::Neq(e1, e2, loc) => {
            let te1 = make_expr(program, function, e1)?;
            let te2 = make_expr(program, function, e2)?;
            if te1.ty() == te2.ty() && te1.ty().is_base_type() {
                Some(tir::Expr::Neq(Box::new(te1), Box::new(te2), Type::Bool))
            } else {
                error(
                    format!(
                        "Cannot compare {} and {} (can only compare same *base* types with '!=')",
                        te1.ty(),
                        te2.ty()
                    ),
                    loc,
                );
                None
            }
        }
        fl::Expr::Lt(e1, e2, loc) => {
            let te1 = make_expr(program, function, e1)?;
            let te2 = make_expr(program, function, e2)?;
            if te1.ty() == Type::Int && te2.ty() == Type::Int {
                Some(tir::Expr::Lt(Box::new(te1), Box::new(te2), Type::Bool))
            } else {
                error(format!("Cannot compare {} and {}", te1.ty(), te2.ty()), loc);
                None
            }
        }
        fl::Expr::Le(e1, e2, loc) => {
            let te1 = make_expr(program, function, e1)?;
            let te2 = make_expr(program, function, e2)?;
            if te1.ty() == Type::Int && te2.ty() == Type::Int {
                Some(tir::Expr::Le(Box::new(te1), Box::new(te2), Type::Bool))
            } else {
                error(format!("Cannot compare {} and {}", te1.ty(), te2.ty()), loc);
                None
            }
        }
        fl::Expr::Gt(e1, e2, loc) => {
            let te1 = make_expr(program, function, e1)?;
            let te2 = make_expr(program, function, e2)?;
            if te1.ty() == Type::Int && te2.ty() == Type::Int {
                Some(tir::Expr::Gt(Box::new(te1), Box::new(te2), Type::Bool))
            } else {
                error(format!("Cannot compare {} and {}", te1.ty(), te2.ty()), loc);
                None
            }
        }
        fl::Expr::Ge(e1, e2, loc) => {
            let te1 = make_expr(program, function, e1)?;
            let te2 = make_expr(program, function, e2)?;
            if te1.ty() == Type::Int && te2.ty() == Type::Int {
                Some(tir::Expr::Ge(Box::new(te1), Box::new(te2), Type::Bool))
            } else {
                error(format!("Cannot compare {} and {}", te1.ty(), te2.ty()), loc);
                None
            }
        }
        fl::Expr::If(e1, e2, e3, loc) => {
            let te1 = make_expr(program, function, e1)?;
            if te1.ty() != Type::Bool {
                error(format!("Cannot use {} as a condition", te1.ty()), loc);
                return None;
            }
            let te2 = make_expr(program, function, e2)?;
            let te3 = make_expr(program, function, e3)?;
            if te2.ty() != te3.ty() {
                error(
                    format!(
                        "Branches of if expression must be of same type (got {} and {})",
                        te2.ty(),
                        te3.ty()
                    ),
                    loc,
                );
                return None;
            } else {
                let ty = te2.ty();
                Some(tir::Expr::If(
                    Box::new(te1),
                    Box::new(te2),
                    Box::new(te3),
                    ty,
                ))
            }
        }
        fl::Expr::Call(name, args, loc) => {
            let f = program.functions.get(name).unwrap();
            let mut typed_args = Vec::new();
            for (i, (ty, arg)) in f.args.iter().zip(args.iter()).enumerate() {
                let typed_arg = make_expr(program, function, arg)?;
                if typed_arg.ty() != *ty {
                    error(
                        format!(
                            "Argument {} to function {} has type {} but should have type {}",
                            i,
                            name,
                            typed_arg.ty(),
                            ty
                        ),
                        loc,
                    );
                    return None;
                }
                typed_args.push(typed_arg);
            }
            Some(tir::Expr::Call(
                name,
                typed_args.into_boxed_slice(),
                f.ret_ty,
            ))
        }
        fl::Expr::Field(e, field, loc) => {
            let te = make_expr(program, function, e)?;

            let Type::Struct(struct_name) = te.ty() else {
                error(format!("'.' operator used on non-struct type {}", te.ty()), loc);
                return None;
            };

            let struct_def = program.structs.get(struct_name).unwrap();
            match struct_def.field_names.get(field) {
                Some(&ty) => Some(tir::Expr::Field(Box::new(te), field, struct_def.fields[ty])),
                None => {
                    error(
                        format!("Struct {} has no field {}", struct_name, field),
                        loc,
                    );
                    None
                }
            }
        }
        fl::Expr::Constructor(name, args, ..) => {
            let mut typed_args = HashMap::new();
            for (field, arg) in args {
                let typed_arg = make_expr(program, function, arg)?;
                typed_args.insert(*field, typed_arg);
            }
            Some(tir::Expr::Constructor(name, typed_args, Type::Struct(name)))
        }
    }
}

fn error<S: AsRef<str>>(message: S, loc: &Loc) {
    eprintln!("[Type error][{}]: {}.", loc, message.as_ref());
}
