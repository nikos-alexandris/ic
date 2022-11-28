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
            wl!(self, "IC_VALUE {}(IC_WORLD world);", def.name);
        }
        wl!(self, "");
    }

    fn definitions(&mut self) {
        for def in std::mem::take(&mut self.program.definitions).iter() {
            wl!(self, "IC_VALUE {}(IC_WORLD world)", def.name);
            wl!(self, "{{");
            indent!(self);

            let res = self.convert_expr(&def.name, &"world", &def.body);

            wl!(self, "return {};", fmt_tmp!(res));

            dedent!(self);
            wl!(self, "}}");
            wl!(self, "");
        }
    }

    fn convert_expr(&mut self, f: &str, world: &str, expr: &il::Expr<'src>) -> usize {
        match expr {
            il::Expr::Var(name) => {
                let tmp = gen_tmp!(self);
                wl!(self, "IC_VALUE {} = {}({});", fmt_tmp!(tmp), name, world);
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
                let lhs = self.convert_expr(f, world, lhs);
                let rhs = self.convert_expr(f, world, rhs);
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
                let lhs = self.convert_expr(f, world, lhs);
                let rhs = self.convert_expr(f, world, rhs);
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
                let lhs = self.convert_expr(f, world, lhs);
                let rhs = self.convert_expr(f, world, rhs);
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
                let lhs = self.convert_expr(f, world, lhs);
                let rhs = self.convert_expr(f, world, rhs);
                wl!(
                    self,
                    "IC_VALUE {} = IC_eq({}, {});",
                    fmt_tmp!(tmp),
                    fmt_tmp!(lhs),
                    fmt_tmp!(rhs)
                );
                tmp
            }
            il::Expr::IsPair(expr) => {
                let tmp_world = gen_tmp!(self);
                wl!(
                    self,
                    "IC_WORLD {} = IC_world_drop_choices(&{});",
                    fmt_tmp!(tmp_world),
                    world
                );

                let tmp = gen_tmp!(self);
                let expr_res = self.convert_expr(f, &fmt_tmp!(tmp_world), expr);
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

                let cond_res = self.convert_expr(f, world, cond);
                wl!(self, "if ({}.tag != IC_VALUE_ATOM) {{", fmt_tmp!(cond_res));
                indent!(self);
                wl!(self, "IC_runtime_error(\"if condition is not an atom\");");
                dedent!(self);
                wl!(self, "}} else if ({}.as.atom == 1) {{", fmt_tmp!(cond_res));
                indent!(self);
                let then_res = self.convert_expr(f, world, then);
                wl!(self, "{} = {};", fmt_tmp!(tmp), fmt_tmp!(then_res));
                dedent!(self);
                wl!(self, "}} else {{");
                indent!(self);
                let els_res = self.convert_expr(f, world, els);
                wl!(self, "{} = {};", fmt_tmp!(tmp), fmt_tmp!(els_res));
                dedent!(self);
                wl!(self, "}}");
                tmp
            }
            il::Expr::Call(callee, i) => {
                let tmp = gen_tmp!(self);
                wl!(
                    self,
                    "IC_VALUE {} = {}(IC_world_cons_tag(&{}, {}));",
                    fmt_tmp!(tmp),
                    callee,
                    world,
                    i
                );
                tmp
            }
            il::Expr::Cons(lhs, rhs) => {
                let tmp = gen_tmp!(self);
                wl!(self, "IC_VALUE {};", fmt_tmp!(tmp));
                wl!(self, "if (!IC_world_has_choices(&{})) {{", world);
                indent!(self);
                wl!(self, "{} = IC_pair({}, {});", fmt_tmp!(tmp), world, f);
                dedent!(self);
                wl!(self, "}} else {{");
                indent!(self);
                let tmp_choice = gen_tmp!(self);
                wl!(self, "IC_CHOICE {};", fmt_tmp!(tmp_choice),);
                let tmp_world = gen_tmp!(self);
                wl!(
                    self,
                    "IC_WORLD {} = IC_world_uncons_choice(&{}, &{});",
                    fmt_tmp!(tmp_world),
                    world,
                    fmt_tmp!(tmp_choice)
                );
                wl!(self, "if ({} == IC_CAR) {{", fmt_tmp!(tmp_choice));
                indent!(self);
                let lhs_res = self.convert_expr(f, &fmt_tmp!(tmp_world), lhs);
                wl!(self, "{} = {};", fmt_tmp!(tmp), fmt_tmp!(lhs_res));
                dedent!(self);
                wl!(self, "}} else {{");
                indent!(self);
                let rhs_res = self.convert_expr(f, &fmt_tmp!(tmp_world), rhs);
                wl!(self, "{} = {};", fmt_tmp!(tmp), fmt_tmp!(rhs_res));
                dedent!(self);
                wl!(self, "}}");
                dedent!(self);
                wl!(self, "}}");

                tmp
            }
            il::Expr::Car(expr) => {
                let tmp = gen_tmp!(self);
                wl!(self, "IC_VALUE {};", fmt_tmp!(tmp));
                let tmp_world = gen_tmp!(self);
                wl!(
                    self,
                    "IC_WORLD {} = IC_world_cons_choice(&{}, IC_CAR);",
                    fmt_tmp!(tmp_world),
                    world
                );
                let expr_res = self.convert_expr(f, &fmt_tmp!(tmp_world), expr);
                wl!(self, "{} = {};", fmt_tmp!(tmp), fmt_tmp!(expr_res));
                tmp
            }
            il::Expr::Cdr(expr) => {
                let tmp = gen_tmp!(self);
                wl!(self, "IC_VALUE {};", fmt_tmp!(tmp));
                let tmp_world = gen_tmp!(self);
                wl!(
                    self,
                    "IC_WORLD {} = IC_world_cons_choice(&{}, IC_CDR);",
                    fmt_tmp!(tmp_world),
                    world
                );
                let expr_res = self.convert_expr(f, &fmt_tmp!(tmp_world), expr);
                wl!(self, "{} = {};", fmt_tmp!(tmp), fmt_tmp!(expr_res));
                tmp
            }
            il::Expr::Actuals(exprs) => {
                let tmp = gen_tmp!(self);
                wl!(self, "IC_VALUE {};", fmt_tmp!(tmp));
                let tmp_tag = gen_tmp!(self);
                wl!(self, "usize {};", fmt_tmp!(tmp_tag));
                let tmp_world = gen_tmp!(self);
                wl!(
                    self,
                    "IC_WORLD {} = IC_world_uncons_tag(&{}, &{});",
                    fmt_tmp!(tmp_world),
                    world,
                    fmt_tmp!(tmp_tag)
                );
                wl!(self, "switch({}) {{", fmt_tmp!(tmp_tag));
                for (i, expr) in exprs.iter().enumerate() {
                    wl!(self, "case {}: {{", i);
                    indent!(self);
                    let expr_res = self.convert_expr(f, &fmt_tmp!(tmp_world), expr);
                    wl!(self, "{} = {};", fmt_tmp!(tmp), fmt_tmp!(expr_res));
                    wl!(self, "break;");
                    dedent!(self);
                    wl!(self, "}}");
                }
                wl!(self, "default: {{");
                indent!(self);
                wl!(self, "IC_runtime_error(\"invalid tag\");");
                dedent!(self);
                wl!(self, "}}");
                wl!(self, "}}");

                tmp
            }
        }
    }

    fn main(&mut self) {
        wl!(self, "int main(void)");
        wl!(self, "{{");
        indent!(self);

        wl!(self, "IC_WORLD world = IC_world_new();");
        wl!(self, "IC_VALUE res = result(world);");
        wl!(self, "IC_value_show(res, true);");
        wl!(self, "return 0;");

        dedent!(self);
        wl!(self, "}}");
        wl!(self, "");
    }
}
