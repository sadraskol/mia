use crate::scanner::Scanner;
use crate::parser::Parser;
use crate::token::TokenType;
use std::env::args;

mod scanner;
mod token;
mod parser;

fn main() {
    let mut args = args();
    args.next();
    run_file(args.next().expect("Usage: rlox [script]"));
}

fn run_file(f: String) {
    let source = std::fs::read_to_string(f).unwrap();
    let mut scanner = Scanner::init(&source);
    let mut parser = Parser::new();

    loop {
        let t = scanner.scan_token(true);
        parser.parse(t, true);

        if t.kind == TokenType::Eof {
            break;
        }
    }

    parser.ast(true);
}
