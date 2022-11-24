use std::fs::read_to_string;

pub mod fl;
pub mod ftoi;
pub mod il;
pub mod lexer;
pub mod loc;
pub mod parser;
pub mod token;

/*
result = f(10) + f(10)
f(x) = x + x
*/

fn main() {
    let source = read_to_string("test.fl").unwrap();
    let lexer = lexer::Lexer::new(&source);
    let mut parser = parser::Parser::new(lexer);
    let fp = parser.parse().unwrap();
    let ftoi = ftoi::FtoI::new(fp);
    let ip = ftoi.convert();
    println!("{:#?}", ip);
}
