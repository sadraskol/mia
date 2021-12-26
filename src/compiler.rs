use crate::token::Token;

struct Local<'a> {
    name: Token<'a>,
    depth: usize,
}

pub struct Compiler<'a> {
    locals: Vec<Local<'a>>,
    scope_depth: usize,
}

impl<'a> Compiler<'a> {
    pub fn new() -> Self {
        Compiler {
            locals: vec![],
            scope_depth: 0,
        }
    }

    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    pub fn end_scope(&mut self) {
        self.scope_depth -= 1;
    }

    pub fn add_local(&mut self, token: Token<'a>) {
        self.locals.push(Local { name: token, depth: self.scope_depth })
    }

    pub fn resolve_local(&self, token: &Token<'a>) -> Option<usize> {
        for (offset, local) in self.locals.iter().enumerate().rev() {
            if local.name.lexeme == token.lexeme {
                return Some(offset)
            }
        }
        None
    }
}