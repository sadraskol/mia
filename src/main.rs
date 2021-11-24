use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::TokenType;
use std::env::args;

mod parser;
mod scanner;
mod token;

fn main() {
    let mut args = args();
    args.next();
    run_file(args.next().expect("Usage: rlox [script]"));
}

fn run_file(f: String) {
    let source = std::fs::read_to_string(f).unwrap();

    let mut scanner = Scanner::init(&source, true);

    let current = scanner.scan_token();
    let mut parser = Parser::init(scanner, true, current);

    parser.parse();
}
