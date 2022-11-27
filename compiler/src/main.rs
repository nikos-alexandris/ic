use std::fs::read_to_string;

pub mod fl;
mod ftoh;
pub mod ftoi;
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

    let source = read_to_string(&args[1]).unwrap();
    let lexer = lexer::Lexer::new(&source);
    let mut parser = parser::Parser::new(lexer);
    let fp = parser.parse().unwrap();
    let ftoh = ftoh::FtoH::new(fp);
    let hir = ftoh.convert().unwrap();
    let htoi = htoi::HtoI::new(hir);
    let il = htoi.convert();
    let itoc = itoc::ItoC::new(il);
    itoc.generate();
    // println!("{:#?}", il);
}
