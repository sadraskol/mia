use crate::token::Token;

#[derive(Debug)]
enum Object {
    None
}

#[derive(Debug)]
enum Expr<'a> {
    Assign(Token<'a>, Box<Expr<'a>>),
    Binary(Box<Expr<'a>>, Token<'a>, Box<Expr<'a>>),
    Grouping(Box<Expr<'a>>),
    Literal(Object),
    Variable(Token<'a>),
}

#[derive(Debug)]
enum Type<'a> {
    AnonymousStruct(Vec<FieldDeclaration<'a>>),
    Nullable(Box<Type<'a>>),
    Array(Box<Type<'a>>),
    Explicit(Token<'a>), // will always be a token of KIdentifier kind.
    Infer,
}

#[derive(Debug)]
struct FieldDeclaration<'a>(Token<'a>, Type<'a>);

#[derive(Debug)]
enum Statement<'a> {
    For(Token<'a>, Expr<'a>, Box<Statement<'a>>),
    Block(Vec<Statement<'a>>),
    Variable(Token<'a>, Option<Expr<'a>>),
    Expr(Expr<'a>),
    Struct(Token<'a>, FieldDeclaration<'a>),
    Export,
    Import,
}

#[derive(Debug)]
struct Program<'a> {
    declarations: Vec<Statement<'a>>
}

pub struct Parser<'a> {
    program: Program<'a>,
    current: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Parser {
            program: Program { declarations: vec![] },
            current: None,
        }
    }

    pub fn current(&self) -> Token<'a> {
        self.current.unwrap()
    }

    pub fn parse(&mut self, tok: Token<'a>, debug: bool) {
        self.current = Some(tok);
    }

    pub fn ast(&self, debug: bool) {
        if debug {
            println!("[Parser]: {:?}", self.program);
        }
    }
}