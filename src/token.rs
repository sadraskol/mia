#[derive(Clone, Copy, Debug)]
pub struct Token<'a> {
    pub kind: TokenType,
    pub lexeme: &'a str,
    pub line: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    RightBracket,
    LeftBracket,
    RightParen,
    LeftParen,
    Identifier,
    Var,
    If,
    For,
    In,
    Question,
    Dot,
    Number,
    String,
    Process,
    Equal,
    NewLine,
    Eof,
    Error,
}
