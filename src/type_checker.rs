use crate::parser::{BuiltinType, Expr, Field, FieldDeclaration, Object, Program, Statement, Type};
use crate::token::{Token, TokenType};

#[derive(Debug)]
struct Scope<'a> {
    enclosing: Option<Box<Scope<'a>>>,
    variables: Vec<(Token<'a>, Type)>,
}

impl<'a> Scope<'a> {
    pub fn find(&self, token: &Token) -> Type {
        for var in &self.variables {
            if var.0.lexeme == token.lexeme {
                return var.1.clone();
            }
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.find(token);
        }
        panic!("Variable {:?} not found in {:?}", token, self);
    }
}

pub struct TypeChecker<'a> {
    scope: Scope<'a>,
    debug: bool,
}

impl<'a> TypeChecker<'a> {
    pub fn init(debug: bool) -> Self {
        TypeChecker {
            scope: Scope {
                enclosing: None,
                variables: vec![],
            },
            debug,
        }
    }

    pub fn check(&mut self, program: &Program<'a>) {
        for statement in &program.0 {
            self.check_statement(statement);
        }
    }

    fn check_statement(&mut self, statement: &Statement<'a>) {
        if self.debug {
            println!("[Type Checker] Statement {:?}", statement);
        }
        match statement {
            Statement::Return(_) => {}
            Statement::For(_, _, _) => {}
            Statement::Fn(_, name, _args, ret, _body) => {
                self.scope.variables.push((*name, Type::Fn(Box::new(ret.clone()))))
            }
            Statement::Block(statements) => {
                let new_scope = Scope {
                    enclosing: None,
                    variables: vec![],
                };
                let enclosing = std::mem::replace(&mut self.scope, new_scope);
                self.scope.enclosing = Some(Box::new(enclosing));

                for statement in statements {
                    self.check_statement(statement);
                }

                let enclosing = self.scope.enclosing.take().unwrap();
                self.scope = *enclosing;
            }
            Statement::Variable(_, token, expr) => {
                let ty = expr
                    .as_ref()
                    .map(|e| self.check_expression(e))
                    .expect("Could not type check expression");
                self.scope.variables.push((*token, ty));
            }
            Statement::Expr(expr) => {
                self.check_expression(expr);
            }
            Statement::Struct(_, name, fields) => {
                self.scope
                    .variables
                    .push((*name, Type::Struct(fields.clone())));
            }
            Statement::Import(_, _) => {}
        }
    }

    fn check_expression(&mut self, expr: &Expr<'a>) -> Type {
        if self.debug {
            println!("[Type Checker] Expression {:?}", expr);
        }
        let res = match expr {
            Expr::Call(exp, _) => {
                if let Type::Fn(ret) = self.check_expression(exp) {
                    (&*ret).clone()
                } else {
                    panic!()
                }
            }
            Expr::Assign(_, _) => {
                panic!()
            }
            Expr::Binary(left, op, right) => {
                let left = self.check_expression(left);
                let right = self.check_expression(right);

                if left != right || !op_compatible(op, &left, &right) {
                    panic!("binary op {:?} does not comply", op);
                }
                left
            }
            Expr::Struct(token, fields) => {
                if let Type::Struct(field_declarations) = self.scope.find(token) {
                    for field_declaration in field_declarations {
                        let i = find_in_fields(fields, &field_declaration);
                        let field = &fields[i];
                        let field_ty = self.check_expression(&field.1);
                        if !field_declaration.1.can_be_inferred_from(&field_ty) {
                            eprintln!(
                                "hello.m:{}: Expected '{}', got '{}'",
                                field.0.line,
                                field_declaration.1.print(),
                                field_ty.print()
                            );
                            std::process::exit(324)
                        }
                    }
                    self.scope.find(token)
                } else {
                    panic!("Could not find structure declaration for {:?}", token);
                }
            }
            Expr::Grouping(expr) => self.check_expression(expr),
            Expr::Array(exprs) => {
                let mut ty = Type::Infer;
                for expr in exprs {
                    let item_ty = self.check_expression(expr);
                    if item_ty == Type::Infer {
                        continue;
                    } else if ty == Type::Infer {
                        ty = item_ty;
                    } else if ty != item_ty {
                        eprintln!("Literal array can only have a single type");
                        std::process::exit(231)
                    }
                }
                Type::Nested(Box::new(Type::Builtin(BuiltinType::Array)), Box::new(ty))
            }
            Expr::Literal(object) => object_type(object),
            Expr::Variable(token) => self.scope.find(token),
        };
        if self.debug {
            println!("[Type Checker] type of expression: {:?}", res);
        }
        res
    }
}

fn find_in_fields(fields: &[Field], declaration: &FieldDeclaration) -> usize {
    for (i, field) in fields.iter().enumerate() {
        if declaration.0 .0 == field.0.lexeme {
            return i;
        }
    }
    panic!("declaration {:?} not found...", declaration);
}

fn object_type(object: &Object) -> Type {
    match object {
        Object::Num(_) => Type::Builtin(BuiltinType::Num),
        Object::String(_) => Type::Builtin(BuiltinType::String),
        Object::Struct(_) => {
            panic!("Struct should not be instantiated in the type checker")
        }
        Object::Array(_) => {
            panic!("Struct should not be instantiated in the type checker")
        }
        Object::None => Type::Nullable(Box::new(Type::Infer)),
    }
}

fn op_compatible(op: &Token, left: &Type, right: &Type) -> bool {
    match op.kind {
        TokenType::Plus => {
            (left == &Type::Builtin(BuiltinType::Num) && right == &Type::Builtin(BuiltinType::Num))
                || (left == &Type::Builtin(BuiltinType::String)
                    && right == &Type::Builtin(BuiltinType::String))
        }
        TokenType::Minus => {
            left == &Type::Builtin(BuiltinType::Num) && right == &Type::Builtin(BuiltinType::Num)
        }
        TokenType::Star => {
            left == &Type::Builtin(BuiltinType::Num) && right == &Type::Builtin(BuiltinType::Num)
        }
        TokenType::Slash => {
            left == &Type::Builtin(BuiltinType::Num) && right == &Type::Builtin(BuiltinType::Num)
        }
        _ => panic!("Unsupported op {:?}", op),
    }
}
