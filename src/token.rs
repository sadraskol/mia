#[derive(Clone, Copy, Debug)]
pub struct Token<'a> {
    pub kind: TokenType,
    pub lexeme: &'a str,
    pub line: usize,
    pub col: usize,
    pub stack_offset: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType {
    RightBracket,
    LeftBracket,
    RightBrace,
    LeftBrace,
    RightParen,
    LeftParen,
    RightCaret,
    LeftCaret,
    Identifier,
    KIdentifier,
    Let,
    Ampersand,
    Nil,
    Plus,
    Minus,
    Star,
    Slash,
    If,
    For,
    Fn,
    Return,
    In,
    Question,
    Dot,
    Semicolon,
    Colon,
    Comma,
    Struct,
    Number,
    String,
    Equal,
    Import,
    From,
    Pub,
    Eof,
    Error,
}
