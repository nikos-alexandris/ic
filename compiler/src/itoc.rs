use std::io::Write;
use std::{fs::File, io::BufWriter};

use crate::il;

pub struct ItoC<'src> {
    pub program: il::Program<'src>,
    ic_home: String,
    out: BufWriter<File>,
    tmp_cnt: usize,
    indentation: usize,
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
        format!("__t{}", $t)
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

impl<'src> ItoC<'src> {
    pub fn new(program: il::Program<'src>, ic_home: String) -> Self {
        std::fs::create_dir("_build").unwrap();
        Self {
            program,
            ic_home,
            out: BufWriter::new(File::create("_build/out.c").unwrap()),
            tmp_cnt: 0,
            indentation: 0,
        }
    }

    pub fn generate(mut self) {
        self.prelude();
        self.atom_names();
        self.prototypes();
        self.main();
        self.definitions();

        self.compile();
    }

    fn compile(&mut self) {
        std::process::Command::new("gcc")
            .arg("-o")
            .arg("_build/out")
            .arg("_build/out.c")
            .arg("-O3")
            .arg("-I")
            .arg(self.ic_home.clone() + "/runtime/include")
            .arg("-L")
            .arg(self.ic_home.clone() + "/runtime/lib")
            .arg("-l")
            .arg("ic")
            .spawn()
            .unwrap();
    }

    fn prelude(&mut self) {
        wl!(self, "#include \"value.h\"");
        wl!(self, "#include <locale.h>");
        wl!(self, "#include <stdio.h>");
        wl!(self, "#include <time.h>");
        wl!(self, "");
    }

    fn atom_names(&mut self) {
        wl!(self, "const char* IC_atom_names[] = {{");

        indent!(self);
        for atom in self.program.atoms.iter() {
            wl!(self, "\"{}\",", atom);
        }
        dedent!(self);

        wl!(self, "}};");
        wl!(self, "");
    }

    fn prototypes(&mut self) {
        for def in self.program.definitions.iter() {
            wl!(self, "static IC_VALUE {}(IC_LAR_PROTO* lar);", def.name);
        }
        wl!(self, "");
    }

    fn definitions(&mut self) {
        let defs = std::mem::take(&mut self.program.definitions);
        for def in defs.iter() {
            wl!(self, "static IC_VALUE {}(IC_LAR_PROTO* lar)", def.name);
            wl!(self, "{{");
            indent!(self);
            if def.is_function {
                wl!(self, "IC_FUNCTION_PUSH(lar);");
            }

            let res = self.convert_expr(&defs, &def.name, &def.body);

            if def.is_function {
                wl!(self, "IC_FUNCTION_POP(lar);");
            }
            wl!(self, "return {};", fmt_tmp!(res));

            dedent!(self);
            wl!(self, "}}");
            wl!(self, "");
        }
    }

    fn convert_expr(
        &mut self,
        defs: &Box<[il::Definition]>,
        f: &str,
        expr: &il::Expr<'src>,
    ) -> usize {
        match expr {
            il::Expr::Var(name) => {
                let tmp = gen_tmp!(self);

                wl!(
                    self,
                    "IC_VALUE {} = IC_lar_get_arg(lar, {});",
                    fmt_tmp!(tmp),
                    self.program.var_indices.get(name).unwrap()
                );
                tmp
            }
            il::Expr::Atom(atom) => {
                let tmp = gen_tmp!(self);
                wl!(self, "IC_VALUE {} = IC_ATOM({});", fmt_tmp!(tmp), atom);
                tmp
            }
            il::Expr::Num(num) => {
                let tmp = gen_tmp!(self);
                wl!(self, "IC_VALUE {} = IC_INTEGER({});", fmt_tmp!(tmp), num);
                tmp
            }
            il::Expr::Add(lhs, rhs) => {
                let tmp = gen_tmp!(self);
                let lhs = self.convert_expr(defs, f, lhs);
                let rhs = self.convert_expr(defs, f, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_add({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::Sub(lhs, rhs) => {
                let tmp = gen_tmp!(self);
                let lhs = self.convert_expr(defs, f, lhs);
                let rhs = self.convert_expr(defs, f, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_sub({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::Mul(lhs, rhs) => {
                let tmp = gen_tmp!(self);
                let lhs = self.convert_expr(defs, f, lhs);
                let rhs = self.convert_expr(defs, f, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_mul({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::Eq(lhs, rhs) => {
                let tmp = gen_tmp!(self);
                let lhs = self.convert_expr(defs, f, lhs);
                let rhs = self.convert_expr(defs, f, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_eq({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::Neq(lhs, rhs) => {
                let tmp = gen_tmp!(self);
                let lhs = self.convert_expr(defs, f, lhs);
                let rhs = self.convert_expr(defs, f, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_neq({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::Lt(lhs, rhs) => {
                let tmp = gen_tmp!(self);
                let lhs = self.convert_expr(defs, f, lhs);
                let rhs = self.convert_expr(defs, f, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_lt({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::Le(lhs, rhs) => {
                let tmp = gen_tmp!(self);
                let lhs = self.convert_expr(defs, f, lhs);
                let rhs = self.convert_expr(defs, f, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_le({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::Gt(lhs, rhs) => {
                let tmp = gen_tmp!(self);
                let lhs = self.convert_expr(defs, f, lhs);
                let rhs = self.convert_expr(defs, f, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_gt({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::Ge(lhs, rhs) => {
                let tmp = gen_tmp!(self);
                let lhs = self.convert_expr(defs, f, lhs);
                let rhs = self.convert_expr(defs, f, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_ge({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::IsPair(expr) => {
                let tmp = gen_tmp!(self);
                let expr_res = self.convert_expr(defs, f, expr);
                wl!(
                    self,
                    "IC_VALUE {} = IC_IS_PAIR({});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(expr_res),
                );
                tmp
            }
            il::Expr::If(cond, then, els) => {
                let tmp = gen_tmp!(self);
                wl!(self, "IC_VALUE {};", fmt_tmp!(tmp));

                let cond_res = self.convert_expr(defs, f, cond);
                wl!(self, "if ({}.tag != IC_VALUE_ATOM) {{", fmt_tmp!(cond_res));
                indent!(self);
                wl!(self, "IC_runtime_error(\"if condition is not an atom\");");
                dedent!(self);
                wl!(self, "}} else if ({}.as.atom == 1) {{", fmt_tmp!(cond_res));
                indent!(self);
                let then_res = self.convert_expr(defs, f, then);
                wl!(self, "{} = {};", fmt_tmp!(tmp), fmt_tmp!(then_res));
                dedent!(self);
                wl!(self, "}} else {{");
                indent!(self);
                let els_res = self.convert_expr(defs, f, els);
                wl!(self, "{} = {};", fmt_tmp!(tmp), fmt_tmp!(els_res));
                dedent!(self);
                wl!(self, "}}");
                tmp
            }
            il::Expr::Call(callee, i) => {
                let tmp = gen_tmp!(self);
                let def = defs.iter().find(|d| d.name == *callee).unwrap();
                w!(
                    self,
                    "IC_VALUE {} = {}(IC_lar_new(lar, {}, (IC_LARF[]){{",
                    fmt_tmp!(tmp),
                    callee,
                    def.args.len()
                );
                for (idx, arg) in def.args.iter().enumerate() {
                    write!(self.out, "{}_{}", arg, i).unwrap();
                    if idx < def.args.len() - 1 {
                        write!(self.out, ", ").unwrap();
                    }
                }
                writeln!(self.out, "}}));").unwrap();
                tmp
            }
            il::Expr::Cons(i) => {
                let tmp = gen_tmp!(self);
                wl!(
                    self,
                    "IC_VALUE {} = IC_PAIR(IC_lar_new(lar, 2, (IC_LARF[]){{__car_{}, __cdr_{}}}));",
                    fmt_tmp!(tmp),
                    i,
                    i
                );
                tmp
            }
            il::Expr::Car(expr) => {
                let res = self.convert_expr(defs, f, expr);
                let tmp = gen_tmp!(self);
                wl!(
                    self,
                    "IC_VALUE {} = IC_car({});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(res)
                );
                tmp
            }
            il::Expr::Cdr(expr) => {
                let res = self.convert_expr(defs, f, expr);
                let tmp = gen_tmp!(self);
                wl!(
                    self,
                    "IC_VALUE {} = IC_cdr({});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(res)
                );
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
        wl!(self, "IC_LAR_VALUE(lar, 0) = IC_ATOM(0);");
        wl!(self, "IC_FUNCTION_PUSH(lar);");
        wl!(
            self,
            "IC_VALUE res = result(IC_lar_new(lar, 0, (IC_LARF[]){{}}));"
        );
        wl!(self, "IC_LAR_VALUE(lar, 0) = res;");
        wl!(self, "IC_value_show(res, true);");
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
