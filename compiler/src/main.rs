use std::fs::read_to_string;

pub mod fl;
pub mod ftoi;
pub mod il;
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
    let ftoi = ftoi::FtoI::new(fp);
    let ip = ftoi.convert();
    println!("{:#?}", ip);
}
