use crate::bytecode::Chunk;
use crate::token::Token;
use crate::{Scanner, TokenType};
use std::ops::{Add, Mul};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Object {
    Num(f64),
    String(String),
    Struct(Vec<(QualifiedName, Object)>),
    Array(Vec<Object>),
    Function(u8, String, Chunk, Type),
    Nil,
}

impl Add<Object> for Object {
    type Output = Object;

    fn add(self, rhs: Object) -> Self::Output {
        if let Object::String(lhs) = self {
            if let Object::String(rhs) = rhs {
                Object::String(lhs + &rhs)
            } else {
                panic!()
            }
        } else if let Object::Num(lhs) = self {
            if let Object::Num(rhs) = rhs {
                Object::Num(lhs + rhs)
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }
}

impl Mul<Object> for Object {
    type Output = Object;

    fn mul(self, rhs: Object) -> Self::Output {
        if let Object::Num(lhs) = self {
            if let Object::Num(rhs) = rhs {
                Object::Num(lhs * rhs)
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }
}

impl Object {
    pub fn as_str(&self) -> String {
        if let Object::String(s) = self {
            s.clone()
        } else {
            panic!()
        }
    }
}

#[derive(Debug)]
pub struct Field<'a>(pub Token<'a>, pub Expr<'a>);

#[derive(Debug)]
pub enum Expr<'a> {
    Call(Box<Expr<'a>>, Vec<Expr<'a>>),
    Binary(Box<Expr<'a>>, Token<'a>, Box<Expr<'a>>),
    Struct(Token<'a>, Vec<Field<'a>>),
    Grouping(Box<Expr<'a>>),
    Array(Vec<Expr<'a>>),
    Literal(Object),
    Variable(Token<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuiltinType {
    Unit,
    Num,
    String,
    Array,
}

impl BuiltinType {
    pub fn print(&self) -> String {
        match self {
            BuiltinType::Unit => "Unit".to_string(),
            BuiltinType::Num => "Num".to_string(),
            BuiltinType::String => "String".to_string(),
            BuiltinType::Array => "Array".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QualifiedName(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Struct(Vec<FieldDeclaration>),
    Nullable(Box<Type>),
    Nested(Box<Type>, Box<Type>),
    Explicit(QualifiedName),
    Fn(Box<Type>),
    // will always be a token of KIdentifier kind.
    Builtin(BuiltinType),
    Infer,
}

impl Type {
    pub fn explicit(name: QualifiedName) -> Type {
        match &*name.0 {
            "String" => Type::Builtin(BuiltinType::String),
            "Number" => Type::Builtin(BuiltinType::Num),
            "Array" => Type::Builtin(BuiltinType::Array),
            _ => Type::Explicit(name),
        }
    }

    pub fn print(&self) -> String {
        match self {
            Type::Fn(ty) => {
                format!("(): {}", ty.print())
            }
            Type::Struct(decls) => {
                let mut s = "(".to_string();
                for d in decls {
                    s.push_str(&d.0 .0);
                    s.push(':');
                    s.push_str(&d.1.print());
                    s.push(',')
                }
                s.push(')');
                s
            }
            Type::Nullable(ty) => format!("{}?", ty.print()),
            Type::Nested(ty, nested) => format!("{}<{}>", ty.print(), nested.print()),
            Type::Explicit(name) => name.0.clone(),
            Type::Builtin(builtin) => builtin.print(),
            Type::Infer => "_".to_string(),
        }
    }

    pub fn can_be_inferred_from(&self, other_ty: &Type) -> bool {
        if self != other_ty && other_ty != &Type::Infer {
            match self {
                Type::Infer => true,
                Type::Nullable(t) => {
                    if let Type::Nullable(other_t) = other_ty {
                        t.can_be_inferred_from(other_t)
                    } else {
                        t.can_be_inferred_from(other_ty)
                    }
                }
                Type::Nested(base, n) => {
                    if let Type::Nested(other_base, other_n) = other_ty {
                        base.can_be_inferred_from(other_base) && n.can_be_inferred_from(other_n)
                    } else {
                        false
                    }
                }
                Type::Explicit(_) => false,
                Type::Builtin(_) => false,
                Type::Struct(_) => false,
                Type::Fn(_) => false,
            }
        } else {
            true
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldDeclaration(pub QualifiedName, pub Type); // Todo remove vec<Token> for a field and use anonymous struct instead.

#[derive(Debug)]
pub enum Statement<'a> {
    Variable(bool, Token<'a>, Option<Expr<'a>>),
    Return(Expr<'a>),
    Expr(Expr<'a>),
    Struct(bool, Token<'a>, Vec<FieldDeclaration>),
    Import(Token<'a>, Token<'a>),
    Fn(bool, Token<'a>, Vec<Token<'a>>, Type, Vec<Statement<'a>>),
}

#[derive(Debug)]
pub struct Program<'a>(pub Vec<Statement<'a>>);

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    current: Token<'a>,
    debug: bool,
}

impl<'a> Parser<'a> {
    pub fn init(scanner: Scanner<'a>, debug: bool, current: Token<'a>) -> Self {
        Parser {
            scanner,
            current,
            debug,
        }
    }

    pub fn parse(&mut self) -> Program {
        self.program()
    }

    fn matches(&mut self, kind: TokenType) -> Option<Token<'a>> {
        if self.current.kind == kind {
            Some(self.advance())
        } else {
            None
        }
    }

    fn advance(&mut self) -> Token<'a> {
        std::mem::replace(&mut self.current, self.scanner.scan_token())
    }

    fn program(&mut self) -> Program<'a> {
        let mut stmts = vec![];
        while self.current.kind != TokenType::Eof {
            let statement = self.declaration();
            if self.debug {
                println!("[Parser] parsed {:?}", statement);
            }
            stmts.push(statement);
        }
        Program(stmts)
    }

    fn declaration(&mut self) -> Statement<'a> {
        if self.matches(TokenType::Let).is_some() {
            self.let_declaration(false)
        } else if self.matches(TokenType::Pub).is_some() {
            if self.matches(TokenType::Let).is_some() {
                self.let_declaration(true)
            } else if self.matches(TokenType::Struct).is_some() {
                self.struct_declaration(true)
            } else {
                panic!("the following declaration cannot be public.");
            }
        } else if self.matches(TokenType::Fn).is_some() {
            self.fn_declaration(false)
        } else if self.matches(TokenType::Import).is_some() {
            self.import_declaration()
        } else if self.matches(TokenType::Struct).is_some() {
            self.struct_declaration(false)
        } else if self.matches(TokenType::Return).is_some() {
            self.return_declaration()
        } else {
            self.statement()
        }
    }

    fn return_declaration(&mut self) -> Statement<'a> {
        let value = self.expression();

        self.consume(TokenType::Semicolon, "Expected a ';' after return");
        Statement::Return(value)
    }

    fn fn_declaration(&mut self, public: bool) -> Statement<'a> {
        let name = self.consume(TokenType::Identifier, "Expect function name.");
        self.consume(TokenType::LeftParen, "Expect '(' after function name.");

        let mut args = vec![];
        while self.matches(TokenType::RightParen).is_none() {
            let token = self.consume(TokenType::Identifier, "Expect argument name.");
            args.push(token);
            self.matches(TokenType::Comma);
        }

        let return_type = if self.matches(TokenType::Colon).is_some() {
            self.types()
        } else {
            Type::Builtin(BuiltinType::Unit)
        };

        self.consume(TokenType::LeftBrace, "Expect '{' after function signature.");

        let mut body = vec![];
        while self.current.kind != TokenType::RightBrace {
            body.push(self.declaration());
        }

        self.consume(
            TokenType::RightBrace,
            "Expect '}' at the end of block declaration.",
        );
        Statement::Fn(public, name, args, return_type, body)
    }

    fn struct_declaration(&mut self, public: bool) -> Statement<'a> {
        let struct_name = self.consume(TokenType::KIdentifier, "Expect a name after struct.");

        self.consume(TokenType::LeftBrace, "Expect '{' after struct name.");

        let mut fields = vec![];
        while self.current.kind != TokenType::RightBracket {
            let key_name = QualifiedName(
                self.consume(TokenType::Identifier, "Expect a field declaration.")
                    .lexeme
                    .to_string(),
            );
            while self.matches(TokenType::Dot).is_some() {
                self.consume(TokenType::Identifier, "Expect a nested field declaration.");
                // TODO BETTER WORK BITCH
            }

            self.consume(TokenType::Colon, "Expect ':' after field declaration.");

            let types = self.types();
            fields.push(FieldDeclaration(key_name, types));

            self.matches(TokenType::Comma);
            if self.current.kind == TokenType::RightBrace {
                break;
            }
        }

        self.consume(TokenType::RightBrace, "Expect '}' after struct definition.");

        Statement::Struct(public, struct_name, fields)
    }

    fn types(&mut self) -> Type {
        let mut base_type = Type::explicit(QualifiedName(
            self.consume(
                TokenType::KIdentifier,
                "Expected types to start with a struct identifier.",
            )
            .lexeme
            .to_string(),
        ));

        if let Some(opening) = self.matches(TokenType::LeftCaret) {
            let nested = self.types();
            self.consume(
                TokenType::RightCaret,
                format!(
                    "Expected matching '>' to the '<' at {}:{}",
                    opening.line, opening.col
                ),
            );
            base_type = Type::Nested(Box::new(base_type), Box::new(nested))
        }

        if self.matches(TokenType::Question).is_some() {
            base_type = Type::Nullable(Box::new(base_type));
        }

        base_type
    }

    fn let_declaration(&mut self, public: bool) -> Statement<'a> {
        let iden = self.consume(TokenType::Identifier, "Expected an identifier.");
        let init = if self.matches(TokenType::Equal).is_some() {
            Some(self.expression())
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expected a ';' after variable declaration",
        );
        Statement::Variable(public, iden, init)
    }

    fn import_declaration(&mut self) -> Statement<'a> {
        if let Some(iden) = self
            .matches(TokenType::Identifier)
            .or_else(|| self.matches(TokenType::KIdentifier))
        {
            self.consume(TokenType::From, "Expect 'from' after import");

            let module = self.consume(TokenType::String, "Expect module to import from");

            self.consume(TokenType::Semicolon, "Expected a ';' after import");
            Statement::Import(iden, module)
        } else {
            panic!("Expected a variable or struct definition to export");
        }
    }

    fn consume<S: ToString>(&mut self, kind: TokenType, msg: S) -> Token<'a> {
        if self.current.kind == kind {
            self.advance()
        } else {
            if self.debug {
                println!("Expected {:?}, got {:?}", kind, self.current.kind);
            }
            panic!(
                "Error at hello.m:{}: {}",
                self.current.line,
                msg.to_string()
            );
        }
    }

    fn statement(&mut self) -> Statement<'a> {
        self.expr_statement()
    }

    fn expr_statement(&mut self) -> Statement<'a> {
        Statement::Expr(self.expression())
    }

    fn expression(&mut self) -> Expr<'a> {
        let expr = self.multiply();
        if self.matches(TokenType::LeftParen).is_some() {
            let mut args = vec![];
            while self.current.kind != TokenType::RightParen {
                args.push(self.expression());

                if self.current.kind != TokenType::RightParen {
                    self.consume(TokenType::Comma, "Expect ',' between function arguments.");
                }
            }
            self.consume(
                TokenType::RightParen,
                "Expect ')' after function arguments.",
            );
            Expr::Call(Box::new(expr), args)
        } else {
            expr
        }
    }

    fn multiply(&mut self) -> Expr<'a> {
        let operand = self.add();
        if let Some(op) = self.matches(TokenType::Star) {
            let value = self.expression();
            Expr::Binary(Box::new(operand), op, Box::new(value))
        } else {
            operand
        }
    }

    fn add(&mut self) -> Expr<'a> {
        let operand = self.primary();
        if let Some(op) = self.matches(TokenType::Plus) {
            let value = self.expression();
            Expr::Binary(Box::new(operand), op, Box::new(value))
        } else {
            operand
        }
    }

    fn primary(&mut self) -> Expr<'a> {
        if let Some(identifier) = self.matches(TokenType::Identifier) {
            Expr::Variable(identifier)
        } else if let Some(num) = self.matches(TokenType::Number) {
            Expr::Literal(Object::Num(f64::from_str(num.lexeme).unwrap()))
        } else if self.matches(TokenType::Nil).is_some() {
            Expr::Literal(Object::Nil)
        } else if let Some(str) = self.matches(TokenType::String) {
            Expr::Literal(Object::String(
                str.lexeme[1..str.lexeme.len() - 1].to_string(),
            ))
        } else if let Some(token) = self.matches(TokenType::KIdentifier) {
            self.structure(token)
        } else if self.matches(TokenType::LeftParen).is_some() {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            Expr::Grouping(Box::new(expr))
        } else if self.matches(TokenType::LeftBracket).is_some() {
            self.array()
        } else {
            panic!("Expected expression at hello.m:{}", self.current.line);
        }
    }

    fn array(&mut self) -> Expr<'a> {
        let mut exprs = vec![];
        while self.matches(TokenType::RightBracket).is_none() {
            exprs.push(self.expression());
            if self.matches(TokenType::Comma).is_none() {
                break;
            }
        }
        self.consume(TokenType::RightBracket, "Expect ']' after an array.");
        Expr::Array(exprs)
    }

    fn structure(&mut self, token: Token<'a>) -> Expr<'a> {
        self.consume(TokenType::LeftBrace, "Expect '{' to instantiate a struct");
        let mut fields = vec![];

        while self.current.kind != TokenType::RightBracket {
            let key_name = self.consume(TokenType::Identifier, "Expect a field declaration.");
            while self.matches(TokenType::Dot).is_some() {
                self.consume(TokenType::Identifier, "Expect a nested field declaration.");
                // TODO BE SMART BITCH
            }

            self.consume(TokenType::Colon, "Expect ':' after field declaration.");

            fields.push(Field(key_name, self.expression()));
            self.matches(TokenType::Comma);
            if self.current.kind == TokenType::RightBrace {
                break;
            }
        }

        self.consume(
            TokenType::RightBrace,
            "Expect '}' after struct instantiation",
        );
        Expr::Struct(token, fields)
    }
}
