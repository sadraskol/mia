use crate::formatter::JsonFmt;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::TokenType;
use crate::type_checker::TypeChecker;
use crate::vm::VM;
use std::env::args;

mod formatter;
mod parser;
mod scanner;
mod token;
mod type_checker;
mod vm;
mod compiler;

fn main() {
    let mut args = args();
    args.next();
    run_file(args.next().expect("Usage: rlox [script]"), false);
}

fn run_file(f: String, debug: bool) {
    let source = std::fs::read_to_string(f).unwrap();

    let mut scanner = Scanner::init(&source, debug);

    let current = scanner.scan_token();
    let mut parser = Parser::init(scanner, debug, current);

    let ast = parser.parse();

    let mut checker = TypeChecker::init(debug);
    checker.check(&ast);

    let mut vm = VM::init(debug);
    if let Some(result) = vm.run(ast) {
        let formatter = JsonFmt::new();
        let str = formatter.format(&result);
        println!("{}", str);
    }
}
