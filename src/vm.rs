use crate::bytecode::{Chunk, Opcode};
use crate::parser::{Expr, Object, QualifiedName, Statement};
use crate::token::Token;
use crate::TokenType;
use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::ops::Deref;

pub struct VM {
    debug: bool,
    frames: Vec<Frame>,
}

struct Frame {
    chunk: Chunk,
    ip: usize,

    locals: [MaybeUninit<Object>; 256],
    stack: Vec<Object>,
    fns: HashMap<String, Object>,

    debug: bool,
}

impl Frame {
    pub fn init(chunk: Chunk, debug: bool) -> Self {
        Frame {
            chunk,
            ip: 0,
            locals: unsafe { MaybeUninit::uninit().assume_init() },
            stack: vec![],
            fns: HashMap::new(),
            debug,
        }
    }

    fn pop(&mut self) -> Object {
        self.stack.pop().unwrap()
    }

    pub fn run(&mut self) -> Object {
        loop {
            let op = self.chunk.code[self.ip].clone();
            self.ip += 1;

            if self.debug {
                println!("[Frame] executing {:?}", op);
                println!("[Frame] stack: {:?}", self.stack);
            }

            match op {
                Opcode::Pop => {
                    self.pop();
                }
                Opcode::Nil => {
                    self.stack.push(Object::Nil);
                }
                Opcode::Load(i) => {
                    let val = unsafe { self.locals[i as usize].assume_init_ref() }.clone();
                    self.stack.push(val);
                }
                Opcode::Store(i) => {
                    let top = self.pop();
                    self.locals[i as usize] = MaybeUninit::new(top);
                }
                Opcode::Constant(i) => self.stack.push(self.chunk.constants[i as usize].clone()),
                Opcode::Struct(_) => {}
                Opcode::Array(_) => {}
                Opcode::Call => {}
                Opcode::Add => {
                    let left = self.pop();
                    let right = self.pop();
                    self.stack.push(left + right);
                }
                Opcode::Multiply => {
                    let left = self.pop();
                    let right = self.pop();
                    self.stack.push(left * right);
                }
                Opcode::Return => {
                    return self.stack[0].clone();
                }
            }
        }
    }
}

impl VM {
    pub fn init(chunk: Chunk, debug: bool) -> Self {
        VM {
            debug,
            frames: vec![Frame::init(chunk, debug)],
        }
    }

    fn frame(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    pub fn run(&mut self) -> Object {
        self.frame().run()
    }
}
