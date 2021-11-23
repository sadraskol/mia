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

    let mut scanner = Scanner::init(&source, true);

    let previous = scanner.scan_token();
    let current = scanner.scan_token();
    let mut parser = Parser::init(scanner, true, previous, current);

    parser.parse();
}
