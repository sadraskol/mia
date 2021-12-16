use crate::parser::{Expr, Object, Program, QualifiedName, Statement};
use crate::token::Token;
use crate::TokenType;

pub struct VM {
    debug: bool,
    stack: Vec<Object>,
}

impl VM {
    pub fn init(debug: bool) -> Self {
        VM {
            debug,
            stack: vec![],
        }
    }
    pub fn run(&mut self, program: Program) -> Option<Object> {
        for statement in program.0 {
            if self.debug {
                println!("[VM] running statement {:?}", statement);
            }
            match statement {
                Statement::For(_, _, _) => {}
                Statement::Block(_) => {}
                Statement::Variable(true, Token { lexeme: "main", .. }, None) => {
                    return Some(Object::None);
                }
                Statement::Variable(true, Token { lexeme: "main", .. }, Some(expr)) => {
                    let value = self.run_expression(&expr);
                    return Some(value);
                }
                Statement::Variable(_, _, maybe_expr) => {
                    let value = if let Some(expr) = maybe_expr {
                        self.run_expression(&expr)
                    } else {
                        Object::None
                    };
                    self.stack.push(value)
                }
                Statement::Expr(_) => {}
                Statement::Struct(_, _, _) => {}
                Statement::Import(_, _) => {}
            }
        }
        None
    }

    fn run_expression(&mut self, expr: &Expr) -> Object {
        match expr {
            Expr::Assign(_, _) => Object::None,
            Expr::Binary(
                left,
                Token {
                    kind: TokenType::Star,
                    ..
                },
                right,
            ) => {
                let left = self.run_expression(left);
                let right = self.run_expression(right);

                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        Object::Num(left * right)
                    } else {
                        panic!("both values are not numbers");
                    }
                } else {
                    panic!("both values are not numbers");
                }
            }
            Expr::Binary(_, _, _) => Object::None,
            Expr::Struct(_, fields) => {
                let mut pairs = vec![];
                for field in fields {
                    pairs.push((
                        QualifiedName(field.0.lexeme.to_string()),
                        self.run_expression(&field.1),
                    ))
                }
                Object::Struct(pairs)
            }
            Expr::Grouping(expr) => self.run_expression(expr),
            Expr::Array(exprs) => {
                let values = exprs.iter().map(|e| self.run_expression(e)).collect();
                Object::Array(values)
            }
            Expr::Literal(o) => o.clone(),
            Expr::Variable(_) => self.stack[0].clone(),
        }
    }
}
