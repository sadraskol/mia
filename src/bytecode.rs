//! The role of the intermediate is to turn the ast into bytecode.
use crate::bytecode::Opcode::Constant;
use crate::compiler::Compiler;
use crate::parser::{Expr, Object, Statement};
use crate::TokenType;

#[derive(Clone, Debug)]
pub enum Opcode {
    Pop,
    Nil,
    /// From locals to the stack
    Load(u8),
    /// From the stack to the locals
    Store(u8),
    Constant(u32),
    Struct(u32),
    Array(u32),
    Call,
    Add,
    Multiply,
    Return,
}

#[derive(Clone, Debug)]
pub struct Chunk {
    compiler: Compiler, // todo change to the opposite dependency between compiler and chunk
    pub code: Vec<Opcode>,
    pub constants: Vec<Object>,
    debug: bool,
}

impl Chunk {
    pub fn init(debug: bool) -> Self {
        Chunk {
            compiler: Compiler::init(debug),
            code: vec![],
            constants: vec![],
            debug,
        }
    }

    fn enclosed_chunk(&self) -> Chunk {
        Chunk {
            compiler: self.compiler.clone(),
            code: vec![],
            constants: vec![],
            debug: self.debug,
        }
    }

    pub fn compile(&mut self, body: &[Statement]) {
        for stmt in body {
            if self.debug {
                println!("[Chunk] compiling stmt: {:?}", stmt);
            }
            self.statement(stmt);
        }
        if self.debug {
            println!("[Chunk] compiled following bytecode:");
            for line in &self.code {
                println!("[Chunk] {:?}", line);
            }
        }
    }

    fn statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Variable(exported, name, opt_val) => {
                if let Some(val) = opt_val {
                    self.expression(val);
                } else {
                    self.code.push(Opcode::Nil);
                }

                if *exported && "main" == name.lexeme {
                    self.code.push(Opcode::Return);
                } else {
                    let i = self.compiler.add_variable(name);
                    self.code.push(Opcode::Store(i as u8));
                }
            }
            Statement::Return(expr) => {
                self.expression(expr);
                self.code.push(Opcode::Return);
            }
            Statement::Expr(expr) => {
                self.expression(expr);
                self.code.push(Opcode::Pop);
            }
            Statement::Struct(_, _, _) => {}
            Statement::Import(_, _) => {}
            Statement::Fn(_, name, args, ret_ty, body) => {
                let mut fn_chunk = self.enclosed_chunk();
                for (arg, _ty) in args {
                    fn_chunk.compiler.add_variable(arg);
                }
                fn_chunk.compile(body);
                let i = self.compiler.add_variable(name);
                self.constants.push(Object::Function(
                    args.len() as u8,
                    name.lexeme.to_string(),
                    fn_chunk,
                    ret_ty.clone(),
                ));
                self.code.push(Constant(self.constants.len() as u32 - 1));
                self.code.push(Opcode::Store(i as u8))
            }
        }
    }

    fn expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Call(target, args) => {
                for arg in args {
                    self.expression(arg);
                }
                self.expression(target);
                self.code.push(Opcode::Call);
            }
            Expr::Binary(left, op, right) => {
                self.expression(right);
                self.expression(left);
                match op.kind {
                    TokenType::Plus => self.code.push(Opcode::Add),
                    TokenType::Star => self.code.push(Opcode::Multiply),
                    _ => {}
                }
            }
            Expr::Struct(_name, fields) => {
                for f in fields.iter().rev() {
                    self.expression(&f.1);
                    self.constants.push(Object::String(f.0.lexeme.to_string()));
                    self.code
                        .push(Opcode::Constant(self.constants.len() as u32 - 1))
                }
                self.code.push(Opcode::Struct(fields.len() as u32));
            }
            Expr::Grouping(expr) => {
                self.expression(expr);
            }
            Expr::Array(values) => {
                for val in values.iter().rev() {
                    self.expression(val);
                }
                self.code.push(Opcode::Array(values.len() as u32));
            }
            Expr::Literal(lit) => {
                self.constants.push(lit.clone());
                self.code
                    .push(Opcode::Constant(self.constants.len() as u32 - 1));
            }
            Expr::Variable(name) => {
                if let Some(i) = self.compiler.resolve_variable(name) {
                    self.code.push(Opcode::Load(i as u8));
                } else {
                    eprintln!("Could not resolve variable name '{}'.", name.lexeme);
                    std::process::exit(12);
                }
            }
        }
    }
}
