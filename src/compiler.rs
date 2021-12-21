use crate::token::Token;

struct Local<'a> {
    name: Token<'a>
}

pub struct Compiler<'a> {
    locals: Vec<Local<'a>>,
}

impl<'a> Compiler<'a> {
    pub fn new() -> Self {
        Compiler {
            locals: vec![],
        }
    }

    pub fn add_local(&mut self, token: Token<'a>) {
        self.locals.push(Local { name: token })
    }

    pub fn resolve_local(&self, token: &Token<'a>) -> Option<usize> {
        for (offset, local) in self.locals.iter().enumerate() {
            if local.name.lexeme == token.lexeme {
                return Some(offset)
            }
        }
        None
    }
}