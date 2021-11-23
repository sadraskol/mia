use crate::token::Token;
use crate::{Scanner, TokenType};

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
pub struct Program<'a>(Vec<Statement<'a>>);

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    previous: Token<'a>,
    current: Token<'a>,
    debug: bool,
}

impl<'a> Parser<'a> {
    pub fn init(scanner: Scanner<'a>, debug: bool, previous: Token<'a>, current: Token<'a>) -> Self {
        Parser {
            scanner,
            previous,
            current,
            debug,
        }
    }

    pub fn parse(&mut self) -> Program {
        self.program()
    }

    fn matches(&mut self, kind: TokenType) -> bool {
        if self.current.kind == kind {
            self.advance();
            true
        } else {
            false
        }
    }

    fn advance(&mut self) {
        self.previous = self.current;
        self.current = self.scanner.scan_token();
    }

    fn program(&mut self) -> Program {
        let mut stmts = vec![];
        while self.current.kind != TokenType::Eof {
            stmts.push(self.declaration())
        }
        Program(stmts)
    }

    fn declaration(&mut self) -> Statement {
        if self.matches(TokenType::Let) {
            self.let_declaration()
        } else if self.matches(TokenType::Export) {
            self.export_declaration()
        } else if self.matches(TokenType::Import) {
            self.import_declaration()
        } else {
            self.statement()
        }
    }

    fn statement(&mut self) -> Statement {
        if self.matches(TokenType::For) {
            self.for_statement()
        } else {
            self.expr_statement()
        }
    }
}