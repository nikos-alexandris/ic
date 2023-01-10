use std::collections::HashMap;

use crate::ty::Type;
use crate::{il, tir};

pub fn transform(program: tir::Program) -> il::Program {
    let structs = transform_structs(&program);
    let functions = transform_functions(&program);
    il::Program::new(functions, structs)
}

fn transform_structs<'src>(program: &tir::Program<'src>) -> HashMap<&'src str, il::Struct<'src>> {
    program
        .structs
        .iter()
        .map(|(name, s)| {
            (
                *name,
                il::Struct::new(
                    s.fields.clone(),
                    s.field_names
                        .iter()
                        .map(|(fl, idx)| (format!("{}__{}", name, fl), *idx))
                        .collect(),
                ),
            )
        })
        .collect()
}

fn transform_functions<'src>(program: &tir::Program<'src>) -> HashMap<String, il::Function<'src>> {
    let mut functions = HashMap::new();
    let mut new_functions = HashMap::new();
    let mut calls = HashMap::new();
    for (name, f) in program.functions.iter() {
        let body = transform_expr(&mut new_functions, &mut calls, program, f, &f.body);
        functions.insert(
            name.to_string(),
            il::Function::new(
                f.args.clone(),
                f.arg_names
                    .iter()
                    .map(|(ar, idx)| (format!("{}__{}", name, ar), *idx))
                    .collect(),
                f.ret_ty,
                body,
                true,
            ),
        );
    }
    functions.extend(
        new_functions
            .into_iter()
            .map(|(name, (ty, expr))| {
                (
                    name,
                    il::Function::new(Box::new([]), HashMap::new(), ty, expr, false),
                )
            })
            .collect::<HashMap<_, _>>(),
    );
    functions
}

fn transform_expr<'src>(
    map: &mut HashMap<String, (Type<'src>, il::Expr<'src>)>,
    calls: &mut HashMap<&'src str, usize>,
    program: &tir::Program<'src>,
    function: &tir::Function<'src>,
    expr: &tir::Expr<'src>,
) -> il::Expr<'src> {
    match expr {
        tir::Expr::Var(name, _) => {
            if let Some(&idx) = function.arg_names.get(name) {
                return il::Expr::Local(name, idx, function.args[idx]);
            }
            il::Expr::Global(name, program.functions.get(name).unwrap().ret_ty)
        }
        tir::Expr::Num(n, t) => il::Expr::Num(*n, *t),
        tir::Expr::Bool(b, t) => il::Expr::Bool(*b, *t),
        tir::Expr::Add(lhs, rhs, t) => il::Expr::Add(
            Box::new(transform_expr(map, calls, program, function, lhs)),
            Box::new(transform_expr(map, calls, program, function, rhs)),
            *t,
        ),
        tir::Expr::Sub(lhs, rhs, t) => il::Expr::Sub(
            Box::new(transform_expr(map, calls, program, function, lhs)),
            Box::new(transform_expr(map, calls, program, function, rhs)),
            *t,
        ),
        tir::Expr::Mul(lhs, rhs, t) => il::Expr::Mul(
            Box::new(transform_expr(map, calls, program, function, lhs)),
            Box::new(transform_expr(map, calls, program, function, rhs)),
            *t,
        ),
        tir::Expr::Eq(lhs, rhs, t) => il::Expr::Eq(
            Box::new(transform_expr(map, calls, program, function, lhs)),
            Box::new(transform_expr(map, calls, program, function, rhs)),
            *t,
        ),
        tir::Expr::Neq(lhs, rhs, t) => il::Expr::Neq(
            Box::new(transform_expr(map, calls, program, function, lhs)),
            Box::new(transform_expr(map, calls, program, function, rhs)),
            *t,
        ),
        tir::Expr::Lt(lhs, rhs, t) => il::Expr::Lt(
            Box::new(transform_expr(map, calls, program, function, lhs)),
            Box::new(transform_expr(map, calls, program, function, rhs)),
            *t,
        ),
        tir::Expr::Gt(lhs, rhs, t) => il::Expr::Gt(
            Box::new(transform_expr(map, calls, program, function, lhs)),
            Box::new(transform_expr(map, calls, program, function, rhs)),
            *t,
        ),
        tir::Expr::Le(lhs, rhs, t) => il::Expr::Le(
            Box::new(transform_expr(map, calls, program, function, lhs)),
            Box::new(transform_expr(map, calls, program, function, rhs)),
            *t,
        ),
        tir::Expr::Ge(lhs, rhs, t) => il::Expr::Ge(
            Box::new(transform_expr(map, calls, program, function, lhs)),
            Box::new(transform_expr(map, calls, program, function, rhs)),
            *t,
        ),
        tir::Expr::If(cond, then, els, t) => il::Expr::If(
            Box::new(transform_expr(map, calls, program, function, cond)),
            Box::new(transform_expr(map, calls, program, function, then)),
            Box::new(transform_expr(map, calls, program, function, els)),
            *t,
        ),
        tir::Expr::Call(name, args, ty) => {
            let f = program.functions.get(name).unwrap();
            let entry = calls.entry(name).or_insert(0);
            let curr_call = *entry;
            *entry += 1;
            let mut new_args = Vec::new();
            for arg in args.iter() {
                new_args.push(transform_expr(map, calls, program, function, arg));
            }
            for (arg_name, idx) in f.arg_names.iter() {
                map.insert(
                    format!("{}__{}__{}", name, arg_name, curr_call),
                    (
                        f.args[*idx], // TODO this might not be needed
                        std::mem::replace(&mut new_args[*idx], il::Expr::Num(0, Type::Int)),
                    ),
                );
            }
            il::Expr::Call(name, curr_call, *ty)
        }
        tir::Expr::Field(expr, name, ty) => {
            let struct_ = match expr.ty() {
                Type::Struct(name) => program.structs.get(name).unwrap(),
                _ => unreachable!(),
            };
            il::Expr::Field(
                Box::new(transform_expr(map, calls, program, function, expr)),
                *struct_.field_names.get(name).unwrap(),
                *ty,
            )
        }
        tir::Expr::Constructor(name, fields, ty) => {
            let s = program.structs.get(name).unwrap();
            let entry = calls.entry(name).or_insert(0);
            let curr_constr = *entry;
            *entry += 1;
            let mut new_fields = HashMap::new();
            for (name, e) in fields.iter() {
                new_fields.insert(*name, transform_expr(map, calls, program, function, e));
            }
            for (field_name, idx) in s.field_names.iter() {
                map.insert(
                    format!("{}__{}__{}", name, field_name, curr_constr),
                    (
                        s.fields[*idx], // TODO this might not be needed
                        std::mem::replace(
                            &mut new_fields.get_mut(field_name).unwrap(),
                            il::Expr::Num(0, Type::Int),
                        ),
                    ),
                );
            }
            il::Expr::Constructor(name, curr_constr, *ty)
        }
    }
}
