// Checks the ast for simple errors, namely:
// 1. 'result' function is defined
// 2. Shadowing of global functions and structs by parameters
// 3. Calls to undefined functions
// 4. Construction of undefined structs
// 5. Arity mismatches in function calls
// 6. Missing fields in constructors
// 7. Use of undefined variables
// 8. Undefined types in type annotations

use std::collections::HashSet;

use crate::fl;
use crate::loc::Loc;
use crate::ty::Type;

pub fn check(program: &fl::Program) -> Result<(), ()> {
    check_for_result(program)?;
    check_for_shadowing(program)?;
    check_for_undefined_functions(program)?;
    check_for_undefined_structs(program)?;
    check_for_arity_mismatches(program)?;
    check_for_missing_fields(program)?;
    check_for_undefined_variables(program)?;
    check_for_undefined_types(program)?;
    Ok(())
}

fn check_for_result(program: &fl::Program) -> Result<(), ()> {
    if program.functions.contains_key("result") {
        Ok(())
    } else {
        error(
            "Exactly one 'result' nullary function must be defined",
            &Loc::new(1, 1),
        );
        Err(())
    }
}

fn check_for_shadowing(program: &fl::Program) -> Result<(), ()> {
    for (name, func) in program.functions.iter() {
        for (arg, _) in func.arg_names.iter() {
            if program.functions.contains_key(arg) {
                error(
                    format!(
                        "Parameter {} in function {} shadows global name {}",
                        arg, name, arg
                    ),
                    &func.loc,
                );
                return Err(());
            }
        }
    }
    Ok(())
}

fn check_for_undefined_functions(program: &fl::Program) -> Result<(), ()> {
    for (_, func) in program.functions.iter() {
        check_expr(program, &func.body)?;
    }
    return Ok(());

    fn check_expr(program: &fl::Program, expr: &fl::Expr) -> Result<(), ()> {
        use fl::Expr::*;
        match expr {
            Var(..) | Num(..) | Bool(..) => Ok(()),
            Add(l, r, ..)
            | Sub(l, r, ..)
            | Mul(l, r, ..)
            | Eq(l, r, ..)
            | Neq(l, r, ..)
            | Lt(l, r, ..)
            | Gt(l, r, ..)
            | Le(l, r, ..)
            | Ge(l, r, ..) => {
                check_expr(program, l)?;
                check_expr(program, r)
            }
            If(c, t, e, ..) => {
                check_expr(program, c)?;
                check_expr(program, t)?;
                check_expr(program, e)
            }
            Call(name, args, loc) => {
                if !program.functions.contains_key(name) {
                    error(format!("Function {} is undefined", name), loc);
                    return Err(());
                }
                for arg in args.iter() {
                    check_expr(program, arg)?;
                }
                Ok(())
            }
            Field(e, ..) => check_expr(program, e),
            Constructor(_, args, ..) => {
                for (_, e) in args.iter() {
                    check_expr(program, e)?;
                }
                Ok(())
            }
        }
    }
}

fn check_for_undefined_structs(program: &fl::Program) -> Result<(), ()> {
    for (_, func) in program.functions.iter() {
        check_expr(program, &func.body)?;
    }
    return Ok(());

    fn check_expr(program: &fl::Program, expr: &fl::Expr) -> Result<(), ()> {
        use fl::Expr::*;
        match expr {
            Var(..) | Num(..) | Bool(..) => Ok(()),
            Add(l, r, ..)
            | Sub(l, r, ..)
            | Mul(l, r, ..)
            | Eq(l, r, ..)
            | Neq(l, r, ..)
            | Lt(l, r, ..)
            | Gt(l, r, ..)
            | Le(l, r, ..)
            | Ge(l, r, ..) => {
                check_expr(program, l)?;
                check_expr(program, r)
            }
            If(c, t, e, ..) => {
                check_expr(program, c)?;
                check_expr(program, t)?;
                check_expr(program, e)
            }
            Call(_, args, ..) => {
                for arg in args.iter() {
                    check_expr(program, arg)?;
                }
                Ok(())
            }
            Field(e, ..) => check_expr(program, e),
            Constructor(name, args, loc) => {
                if !program.structs.contains_key(name) {
                    error(format!("Function {} is undefined", name), loc);
                    return Err(());
                }
                for (_, e) in args.iter() {
                    check_expr(program, e)?;
                }
                Ok(())
            }
        }
    }
}

fn check_for_arity_mismatches(program: &fl::Program) -> Result<(), ()> {
    for (_, func) in program.functions.iter() {
        check_expr(program, &func.body)?;
    }
    return Ok(());

    fn check_expr(program: &fl::Program, expr: &fl::Expr) -> Result<(), ()> {
        use fl::Expr::*;
        match expr {
            Var(..) | Num(..) | Bool(..) => Ok(()),
            Add(l, r, ..)
            | Sub(l, r, ..)
            | Mul(l, r, ..)
            | Eq(l, r, ..)
            | Neq(l, r, ..)
            | Lt(l, r, ..)
            | Gt(l, r, ..)
            | Le(l, r, ..)
            | Ge(l, r, ..) => {
                check_expr(program, l)?;
                check_expr(program, r)
            }
            If(c, t, e, ..) => {
                check_expr(program, c)?;
                check_expr(program, t)?;
                check_expr(program, e)
            }
            Call(name, args, loc) => {
                let func = program.functions.get(name).unwrap();
                if func.args.len() != args.len() {
                    error(
                        format!(
                            "Function {} called with {} arguments, but expects {}",
                            name,
                            args.len(),
                            func.args.len()
                        ),
                        loc,
                    );
                    return Err(());
                }
                for arg in args.iter() {
                    check_expr(program, arg)?;
                }
                Ok(())
            }
            Field(e, ..) => check_expr(program, e),
            Constructor(_, args, ..) => {
                for (_, e) in args.iter() {
                    check_expr(program, e)?;
                }
                Ok(())
            }
        }
    }
}

fn check_for_missing_fields(program: &fl::Program) -> Result<(), ()> {
    for (_, func) in program.functions.iter() {
        check_expr(program, &func.body)?;
    }
    return Ok(());

    fn check_expr(program: &fl::Program, expr: &fl::Expr) -> Result<(), ()> {
        use fl::Expr::*;
        match expr {
            Var(..) | Num(..) | Bool(..) => Ok(()),
            Add(l, r, ..)
            | Sub(l, r, ..)
            | Mul(l, r, ..)
            | Eq(l, r, ..)
            | Neq(l, r, ..)
            | Lt(l, r, ..)
            | Gt(l, r, ..)
            | Le(l, r, ..)
            | Ge(l, r, ..) => {
                check_expr(program, l)?;
                check_expr(program, r)
            }
            If(c, t, e, ..) => {
                check_expr(program, c)?;
                check_expr(program, t)?;
                check_expr(program, e)
            }
            Call(_, args, ..) => {
                for arg in args.iter() {
                    check_expr(program, arg)?;
                }
                Ok(())
            }
            Field(e, ..) => check_expr(program, e),
            Constructor(name, args, loc) => {
                let struct_ = program.structs.get(name).unwrap();
                for (field, _) in struct_.field_names.iter() {
                    if !args.contains_key(field) {
                        error(format!("Constructor {} missing field {}", name, field), loc);
                        return Err(());
                    }
                }
                for (arg, _) in args.iter() {
                    if !struct_.field_names.contains_key(arg) {
                        error(format!("Constructor {} has no field {}", name, arg), loc);
                        return Err(());
                    }
                }
                for (_, e) in args.iter() {
                    check_expr(program, e)?;
                }
                Ok(())
            }
        }
    }
}

fn check_for_undefined_variables(program: &fl::Program) -> Result<(), ()> {
    for (_, func) in program.functions.iter() {
        check_expr(
            program,
            &func
                .arg_names
                .iter()
                .map(|(n, _)| *n)
                .collect::<HashSet<_>>(),
            &func.body,
        )?;
    }
    return Ok(());

    fn check_expr(program: &fl::Program, args: &HashSet<&str>, expr: &fl::Expr) -> Result<(), ()> {
        use fl::Expr::*;
        match expr {
            Var(name, loc) => {
                if args.contains(name) {
                    return Ok(());
                }
                if let Some(n) = program.functions.get(name) && n.args.len() == 0 {
                    return Ok(());
                }
                error(format!("Variable {} is undefined", name), loc);
                Err(())
            }
            Num(..) | Bool(..) => Ok(()),
            Add(l, r, ..)
            | Sub(l, r, ..)
            | Mul(l, r, ..)
            | Eq(l, r, ..)
            | Neq(l, r, ..)
            | Lt(l, r, ..)
            | Gt(l, r, ..)
            | Le(l, r, ..)
            | Ge(l, r, ..) => {
                check_expr(program, args, l)?;
                check_expr(program, args, r)
            }
            If(c, t, e, ..) => {
                check_expr(program, args, c)?;
                check_expr(program, args, t)?;
                check_expr(program, args, e)
            }
            Call(_, es, ..) => {
                for e in es.iter() {
                    check_expr(program, args, e)?;
                }
                Ok(())
            }
            Field(e, ..) => check_expr(program, args, e),
            Constructor(_, es, ..) => {
                for (_, e) in es.iter() {
                    check_expr(program, args, e)?;
                }
                Ok(())
            }
        }
    }
}

fn check_for_undefined_types(program: &fl::Program) -> Result<(), ()> {
    for (_, f) in program.functions.iter() {
        for t in f.args.iter() {
            check_type(program, &f.loc, t)?;
        }
        check_type(program, &f.loc, &f.ret_ty)?;
    }
    for (_, s) in program.structs.iter() {
        for t in s.fields.iter() {
            check_type(program, &s.loc, t)?;
        }
    }
    return Ok(());

    fn check_type(program: &fl::Program, loc: &Loc, ty: &Type) -> Result<(), ()> {
        use Type::*;
        match ty {
            Int | Bool => Ok(()),
            Struct(name) => {
                if program.structs.contains_key(name) {
                    Ok(())
                } else {
                    error(format!("Type {} is undefined", name), loc);
                    Err(())
                }
            }
        }
    }
}

fn error<S: AsRef<str>>(message: S, loc: &Loc) {
    eprintln!("[Semantic error][{}]: {}.", loc, message.as_ref());
}
