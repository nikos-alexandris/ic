use std::io::Write;
use std::{fs::File, io::BufWriter};

use crate::il;

pub fn generate(program: il::Program, ic_home: String) {
    State::new().generate(&program, ic_home);
}

macro_rules! w {
    ($self:ident, $fmt:expr) => {{
        write!($self.out, "{}", "    ".repeat($self.indentation)).unwrap();
        write!($self.out, $fmt).unwrap();
    }};
    ($self:ident, $fmt:expr, $($arg:tt)*) => {
        write!($self.out, "{}", "    ".repeat($self.indentation)).unwrap();
        write!($self.out, $fmt, $($arg)*).unwrap();
    };
}

macro_rules! wl {
    ($self:ident, $fmt:expr) => {{
        write!($self.out, "{}", "    ".repeat($self.indentation)).unwrap();
        writeln!($self.out, $fmt).unwrap();
    }};
    ($self:ident, $fmt:expr, $($arg:tt)*) => {
        write!($self.out, "{}", "    ".repeat($self.indentation)).unwrap();
        writeln!($self.out, $fmt, $($arg)*).unwrap();
    };
}

macro_rules! fmt_tmp {
    ($t:expr) => {
        format!("tmp__{}", $t)
    };
}

macro_rules! gen_tmp {
    ($self:ident) => {{
        let tmp = $self.tmp_cnt;
        $self.tmp_cnt += 1;
        tmp
    }};
}

macro_rules! indent {
    ($self:ident) => {{
        $self.indentation += 1;
    }};
}

macro_rules! dedent {
    ($self:ident) => {{
        $self.indentation -= 1;
    }};
}

struct State {
    out: BufWriter<File>,
    tmp_cnt: usize,
    indentation: usize,
}

impl State {
    fn new() -> Self {
        std::fs::create_dir("_build").unwrap();
        Self {
            out: BufWriter::new(File::create("_build/out.c").unwrap()),
            tmp_cnt: 0,
            indentation: 0,
        }
    }

    fn generate(&mut self, program: &il::Program, ic_home: String) {
        self.prelude();
        self.prototypes(program);
        self.main();
        self.functions(program);
        self.compile(ic_home);
    }

    fn compile(&mut self, ic_home: String) {
        std::process::Command::new("gcc")
            .arg("-o")
            .arg("_build/out")
            .arg("_build/out.c")
            .arg("-O3")
            .arg("-I")
            .arg(ic_home.clone() + "/runtime/include")
            .arg("-L")
            .arg(ic_home.clone() + "/runtime/lib")
            .arg("-l")
            .arg("icr")
            .spawn()
            .unwrap();
    }

    fn prelude(&mut self) {
        wl!(self, "#include \"lar.h\"");
        wl!(self, "#include <locale.h>");
        wl!(self, "#include <stdio.h>");
        wl!(self, "#include <time.h>");
        wl!(self, "");
    }

    fn prototypes(&mut self, program: &il::Program) {
        for (name, _) in program.functions.iter() {
            wl!(self, "static IC_VALUE {}(IC_LAR_PROTO* lar);", name);
        }
        wl!(self, "");
    }

    fn functions(&mut self, program: &il::Program) {
        for (name, f) in program.functions.iter() {
            wl!(self, "static IC_VALUE {}(IC_LAR_PROTO* lar)", name);
            wl!(self, "{{");
            indent!(self);
            if f.is_function {
                wl!(self, "IC_FUNCTION_PUSH(lar);");
            }

            let res = self.expr(program, &f.body);

            if f.is_function {
                wl!(self, "IC_FUNCTION_POP(lar);");
            }
            wl!(self, "return {};", fmt_tmp!(res));

            dedent!(self);
            wl!(self, "}}");
            wl!(self, "");

            self.tmp_cnt = 0;
        }
    }

    fn expr(&mut self, program: &il::Program, expr: &il::Expr) -> usize {
        match expr {
            il::Expr::Local(_, idx, _) => {
                let tmp = gen_tmp!(self);

                wl!(
                    self,
                    "IC_VALUE {} = IC_lar_get_arg(lar, {});",
                    fmt_tmp!(tmp),
                    idx
                );
                tmp
            }
            il::Expr::Global(name, _) => {
                let tmp = gen_tmp!(self);

                wl!(
                    self,
                    "IC_VALUE {} = {}(IC_lar_new(lar, 0, NULL));",
                    fmt_tmp!(tmp),
                    name
                );
                tmp
            }
            il::Expr::Num(num, _) => {
                let tmp = gen_tmp!(self);
                wl!(self, "IC_VALUE {} = IC_BOX({});", fmt_tmp!(tmp), num);
                tmp
            }
            il::Expr::Bool(b, _) => {
                let tmp = gen_tmp!(self);
                wl!(self, "IC_VALUE {} = IC_BOX({});", fmt_tmp!(tmp), *b as u8);
                tmp
            }
            il::Expr::Add(lhs, rhs, _) => {
                let tmp = gen_tmp!(self);
                let lhs = self.expr(program, lhs);
                let rhs = self.expr(program, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_ADD({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::Sub(lhs, rhs, _) => {
                let tmp = gen_tmp!(self);
                let lhs = self.expr(program, lhs);
                let rhs = self.expr(program, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_SUB({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::Mul(lhs, rhs, _) => {
                let tmp = gen_tmp!(self);
                let lhs = self.expr(program, lhs);
                let rhs = self.expr(program, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_MUL({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::Eq(lhs, rhs, _) => {
                let tmp = gen_tmp!(self);
                let lhs = self.expr(program, lhs);
                let rhs = self.expr(program, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_EQ({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::Neq(lhs, rhs, _) => {
                let tmp = gen_tmp!(self);
                let lhs = self.expr(program, lhs);
                let rhs = self.expr(program, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_NEQ({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::Lt(lhs, rhs, _) => {
                let tmp = gen_tmp!(self);
                let lhs = self.expr(program, lhs);
                let rhs = self.expr(program, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_LT({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::Le(lhs, rhs, _) => {
                let tmp = gen_tmp!(self);
                let lhs = self.expr(program, lhs);
                let rhs = self.expr(program, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_LE({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::Gt(lhs, rhs, _) => {
                let tmp = gen_tmp!(self);
                let lhs = self.expr(program, lhs);
                let rhs = self.expr(program, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_GT({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::Ge(lhs, rhs, _) => {
                let tmp = gen_tmp!(self);
                let lhs = self.expr(program, lhs);
                let rhs = self.expr(program, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_GE({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::If(cond, then, els, _) => {
                let tmp = gen_tmp!(self);
                wl!(self, "IC_VALUE {};", fmt_tmp!(tmp));

                let cond_res = self.expr(program, cond);
                wl!(self, "if (IC_UNBOX({})) {{", fmt_tmp!(cond_res));
                indent!(self);
                let then_res = self.expr(program, then);
                wl!(self, "{} = {};", fmt_tmp!(tmp), fmt_tmp!(then_res));
                dedent!(self);
                wl!(self, "}} else {{");
                indent!(self);
                let else_res = self.expr(program, els);
                wl!(self, "{} = {};", fmt_tmp!(tmp), fmt_tmp!(else_res));
                dedent!(self);
                wl!(self, "}}");

                tmp
            }
            il::Expr::Call(callee, i, _) => {
                let tmp = gen_tmp!(self);
                let f = program.functions.get(*callee).unwrap();
                w!(
                    self,
                    "IC_VALUE {} = {}(IC_lar_new(lar, {}, (IC_LARF[]){{",
                    fmt_tmp!(tmp),
                    callee,
                    f.args.len()
                );
                let mut args = vec![Default::default(); f.args.len()];
                for (arg, idx) in f.arg_names.iter() {
                    args[*idx] = format!("{}__{}", arg, i);
                }
                for (i, arg) in args.iter().enumerate() {
                    write!(self.out, "{}", arg).unwrap();
                    if i < args.len() - 1 {
                        write!(self.out, ", ").unwrap();
                    }
                }
                writeln!(self.out, "}}));").unwrap();
                tmp
            }
            il::Expr::Field(expr, field, _) => {
                let tmp = gen_tmp!(self);
                let res = self.expr(program, expr);
                wl!(
                    self,
                    "IC_VALUE {} = IC_lar_get_arg((IC_LAR_PROTO*){}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(res),
                    *field
                );
                tmp
            }
            il::Expr::Constructor(name, i, _) => {
                let tmp = gen_tmp!(self);
                let s = program.structs.get(*name).unwrap();

                if s.fields.len() == 0 {
                    wl!(
                        self,
                        "IC_VALUE {} = IC_lar_new(lar, 0, NULL);",
                        fmt_tmp!(tmp)
                    );
                    return tmp;
                }

                w!(
                    self,
                    "IC_VALUE {} = (IC_VALUE)IC_lar_new(lar, {}, (IC_LARF[]){{",
                    fmt_tmp!(tmp),
                    s.fields.len()
                );
                let mut fields = vec![Default::default(); s.fields.len()];
                for (arg, idx) in s.field_names.iter() {
                    fields[*idx] = format!("{}__{}", arg, i);
                }
                for (i, arg) in fields.iter().enumerate() {
                    write!(self.out, "{}", arg).unwrap();
                    if i < fields.len() - 1 {
                        write!(self.out, ", ").unwrap();
                    }
                }
                writeln!(self.out, "}});").unwrap();
                tmp
            }
        }
    }

    fn main(&mut self) {
        wl!(self, "int main(void)");
        wl!(self, "{{");
        indent!(self);

        wl!(self, "setlocale(LC_NUMERIC, \"\");");
        wl!(self, "clock_t t1, t2;");
        wl!(self, "t1 = clock();");

        wl!(
            self,
            "IC_LAR_PROTO* lar = IC_lar_new(NULL, 1, (IC_LARF[]){{NULL}});"
        );
        wl!(self, "IC_LAR_VALUE(lar, 0) = 0;");
        wl!(self, "IC_FUNCTION_PUSH(lar);");
        wl!(self, "IC_VALUE res = result(IC_lar_new(lar, 0, NULL));",);
        wl!(self, "IC_LAR_VALUE(lar, 0) = res;");
        wl!(self, "IC_value_show(res);");
        wl!(self, "IC_FUNCTION_POP(lar);");

        wl!(self, "t2 = clock();");
        wl!(
            self,
            "printf(\"c time = %.10f sec (GC: %.10f sec, Alloc: %'lu bytes)\\n\", ((double)(t2 - t1) / CLOCKS_PER_SEC),
            IC_get_gc_time(), IC_get_alloc_size());"
        );
        wl!(self, "IC_mem_cleanup();");
        wl!(self, "return 0;");

        dedent!(self);
        wl!(self, "}}");
        wl!(self, "");
    }
}
