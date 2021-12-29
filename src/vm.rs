use crate::bytecode::{Chunk, Opcode};
use crate::parser::{Object, QualifiedName};

pub struct VM {
    frames: Vec<Frame>,
}

struct Frame {
    chunk: Chunk,
    ip: usize,

    locals: Vec<Object>,
    stack: Vec<Object>,

    name: String,
    debug: bool,
}

impl Frame {
    pub fn init(chunk: Chunk, debug: bool) -> Self {
        Frame {
            chunk,
            ip: 0,
            locals: vec![],
            stack: vec![],
            name: "main".to_string(),
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
                println!("[Frame {}] executing {:?}", self.name, op);
            }

            match op {
                Opcode::Pop => {
                    self.pop();
                }
                Opcode::Nil => {
                    self.stack.push(Object::Nil);
                }
                Opcode::Load(i) => {
                    let val = self.locals[i as usize].clone();
                    self.stack.push(val);
                }
                Opcode::Store(i) => {
                    let top = self.pop();
                    if self.locals.len() <= i as usize {
                        self.locals.push(top);
                    } else {
                        self.locals[i as usize] = top;
                    }
                }
                Opcode::Constant(i) => self.stack.push(self.chunk.constants[i as usize].clone()),
                Opcode::Struct(s) => {
                    let mut fields = vec![];
                    for _ in 0..s {
                        let field = self.pop();
                        let val = self.pop();
                        fields.push((QualifiedName(field.as_str()), val));
                    }
                    self.stack.push(Object::Struct(fields))
                }
                Opcode::Array(s) => {
                    let mut arr = vec![];
                    for _ in 0..s {
                        arr.push(self.pop());
                    }
                    self.stack.push(Object::Array(arr))
                }
                Opcode::Call => {
                    let fun = self.pop();
                    if let Object::Function(arity, name, chunk, _) = fun {
                        let mut args = vec![];
                        for _ in 0..arity {
                            args.push(self.pop());
                        }
                        let mut frame = Frame {
                            chunk,
                            ip: 0,
                            locals: args,
                            stack: vec![],
                            name,
                            debug: self.debug,
                        };
                        self.stack.push(frame.run());
                    } else {
                        eprintln!("Could not execute {:?}", fun);
                    }
                }
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

            if self.debug {
                println!("[Frame {}] stack: {:?}", self.name, self.stack);
                println!("[Frame {}] locals: {:?}", self.name, self.locals);
            }
        }
    }
}

impl VM {
    pub fn init(chunk: Chunk, debug: bool) -> Self {
        VM {
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
