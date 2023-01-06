#![feature(iter_intersperse)]

use std::env;
use std::fs::read_to_string;

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

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        panic!("Usage: ic <source file>");
    }

    let ic_home = match env::var("IC_HOME") {
        Ok(val) => val,
        Err(_) => panic!("IC_HOME environment variable not set"),
    };

    let source = read_to_string(&args[1]).unwrap();
    let lexer = lexer::Lexer::new(&source);
    let mut parser = parser::Parser::new(lexer);
    let fp = parser.parse().unwrap();
    let ftoh = ftoh::FtoH::new(fp);
    let hir = ftoh.convert().unwrap();
    let htoi = htoi::HtoI::new(hir);
    let il = htoi.convert();

    let itoc = itoc::ItoC::new(il, ic_home);
    itoc.generate();
}
