use crate::scanner::Scanner;
use crate::token::TokenType;
use std::env::args;

mod scanner;
mod token;

fn main() {
    let mut args = args();
    args.next();
    run_file(args.next().expect("Usage: rlox [script]"));
}

fn run_file(f: String) {
    let source = std::fs::read_to_string(f).unwrap();
    let mut scanner = Scanner::init(&source);

    loop {
        let t = scanner.scan_token(true);

        if t.kind == TokenType::Eof {
            break;
        }
    }
}
