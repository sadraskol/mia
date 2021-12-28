use crate::token::Token;

#[derive(Clone, Debug)]
struct Local {
    name: String,
}

#[derive(Clone, Debug)]
pub struct Compiler {
    locals: Vec<Local>,
    debug: bool,
}

impl Compiler {
    pub fn init(debug: bool) -> Self {
        Compiler {
            locals: vec![],
            debug,
        }
    }

    pub fn begin_scope(&mut self) {}

    pub fn end_scope(&mut self) {}

    pub fn add_variable(&mut self, token: &Token) -> usize {
        self.locals.push(Local {
            name: token.lexeme.to_string(),
        });

        if self.debug {
            println!(
                "[Compiler] added local '{}' in position {}",
                token.lexeme.to_string(),
                self.locals.len() - 1
            );
        }

        self.locals.len() - 1
    }

    pub fn resolve_variable(&self, token: &Token) -> Option<usize> {
        for (offset, local) in self.locals.iter().enumerate().rev() {
            if local.name == token.lexeme {
                return Some(offset);
            }
        }
        None
    }
}
