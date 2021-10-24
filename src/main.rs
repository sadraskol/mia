use crate::parser::TokenType;
use crate::parser::Scanner;
use std::env::args;

mod parser;

fn main() {
    let mut args = args();
    args.next();
    run_file(args.next().expect("Usage: rlox [script]"));
}

fn run_file(f: String) {
    let source = std::fs::read_to_string(f).unwrap();
    let mut scanner = Scanner::init(&source);

    loop {
        let t = scanner.scan_token();

        if t.kind == TokenType::Eof {
            break;
        }
        println!("{:?}", t);
    }
}
