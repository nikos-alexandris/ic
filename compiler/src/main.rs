#![feature(iter_intersperse)]

use std::env;
use std::fs::read_to_string;
use std::process::ExitCode;

pub mod fl;
mod ftoh;
pub mod hir;
mod htoi;
pub mod il;
pub mod itoc;
pub mod lexer;
pub mod loc;
pub mod parser;
pub mod token;

fn main() -> ExitCode {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LIB_BACKTRACE", "0");

    let args = env::args().collect::<Box<[String]>>();

    if args.len() != 2 {
        eprintln!("Source file required");
        return ExitCode::FAILURE;
    }

    let ic_home = match env::var("IC_HOME") {
        Ok(ic_home) => ic_home,
        Err(e) => {
            eprintln!("Can't get environment variable IC_HOME: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let source = match read_to_string(&args[1]) {
        Ok(source) => source,
        Err(e) => {
            eprintln!("Failed to read source file {}: {}", &args[1], e);
            return ExitCode::FAILURE;
        }
    };

    let lexer = lexer::Lexer::new(&source);

    let mut parser = parser::Parser::new(lexer);
    let Some(fp) = parser.parse() else {
        return ExitCode::FAILURE;
    };

    let ftoh = ftoh::FtoH::new(fp);
    let Some(hir) = ftoh.convert() else {
        return ExitCode::FAILURE;
    };

    let htoi = htoi::HtoI::new(hir);
    let il = htoi.convert();

    let itoc = itoc::ItoC::new(il, ic_home);
    itoc.generate();

    ExitCode::SUCCESS
}
