#![feature(let_chains)]

use std::env;
use std::process::ExitCode;

mod fl;
mod flchk;
mod il;
mod itoc;
mod lexer;
mod loc;
mod parser;
mod tc;
mod tir;
mod token;
mod ttoi;
mod ty;

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

    let source = match std::fs::read_to_string(&args[1]) {
        Ok(source) => source,
        Err(e) => {
            eprintln!("Failed to read source file {}: {}", &args[1], e);
            return ExitCode::FAILURE;
        }
    };

    let Some(fp) = parser::parse(&source) else {
        return ExitCode::FAILURE;
    };

    let Ok(()) = flchk::check(&fp) else {
        return ExitCode::FAILURE;
    };

    let Some(tir) = tc::typecheck(fp) else {
        return ExitCode::FAILURE;
    };

    let ip = ttoi::transform(tir);

    itoc::generate(ip, ic_home);

    ExitCode::SUCCESS
}
