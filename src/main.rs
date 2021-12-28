use crate::bytecode::Chunk;
use crate::formatter::JsonFmt;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::TokenType;
use crate::type_checker::TypeChecker;
use crate::vm::VM;
use std::env::args;

mod bytecode;
mod compiler;
mod formatter;
mod parser;
mod scanner;
mod token;
mod type_checker;
mod vm;

fn main() {
    let mut args = args();
    args.next();
    run_file(args.next().expect("Usage: rlox [script]"), true);
}

fn run_file(f: String, debug: bool) {
    let source = std::fs::read_to_string(f).unwrap();

    let mut scanner = Scanner::init(&source, debug);

    let current = scanner.scan_token();
    let mut parser = Parser::init(scanner, debug, current);

    let ast = parser.parse();

    let mut checker = TypeChecker::init(debug);
    checker.check(&ast);

    let mut main = Chunk::init(debug);
    main.compile(&ast.0);

    let mut vm = VM::init(main, debug);
    let result = vm.run();

    let formatter = JsonFmt::new();
    let str = formatter.format(&result);

    println!("{}", str);
}
