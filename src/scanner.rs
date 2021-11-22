use crate::token::Token;
use crate::token::TokenType;
use std::str::Chars;

pub struct Scanner<'a> {
    iter: Chars<'a>,
    current: &'a str,
    offset: usize,
    line: usize,
    col: usize,
    col_offset: usize,
}

impl<'a> Scanner<'a> {
    pub fn init(source: &'a str) -> Self {
        Scanner {
            iter: source.chars(),
            current: source,
            offset: 0,
            line: 1,
            col: 1,
            col_offset: 1,
        }
    }

    pub fn scan_token(&mut self, debug: bool) -> Token<'a> {
        self.skip_whitespace();
        self.current = self.iter.as_str();
        self.offset = 0;
        let t = if let Some(c) = self.advance() {
            if c.is_alphabetic() || c == '_' {
                self.identifier()
            } else if c.is_numeric() {
                self.number()
            } else {
                match c {
                    '[' => self.make_token(TokenType::LeftBracket),
                    ']' => self.make_token(TokenType::RightBracket),
                    '(' => self.make_token(TokenType::LeftParen),
                    ')' => self.make_token(TokenType::RightParen),
                    '{' => self.make_token(TokenType::LeftBrace),
                    '}' => self.make_token(TokenType::RightBrace),
                    '<' => self.make_token(TokenType::LeftCaret),
                    '>' => self.make_token(TokenType::RightCaret),
                    '=' => self.make_token(TokenType::Equal),
                    '&' => self.make_token(TokenType::Ampersand),
                    '+' => self.make_token(TokenType::Plus),
                    '-' => self.make_token(TokenType::Minus),
                    '*' => self.make_token(TokenType::Star),
                    '/' => self.make_token(TokenType::Slash),
                    '.' => self.make_token(TokenType::Dot),
                    ';' => self.make_token(TokenType::Semicolon),
                    ',' => self.make_token(TokenType::Comma),
                    ':' => self.make_token(TokenType::Colon),
                    '?' => self.make_token(TokenType::Question),
                    '\'' => self.string(),
                    _ => {
                        eprintln!("Unexpected character at line {}: {}", self.line, c);
                        self.error_token("Unexpected character.")
                    },
                }
            }
        } else {
            self.make_token(TokenType::Eof)
        };

        if debug {
            println!("[Scanner] {:?}", t);
        }

        t
    }

    fn skip_whitespace(&mut self) {
        loop {
            if let Some(c) = self.peek() {
                if c.is_whitespace() {
                    if c == '\n' {
                        self.line += 1;
                        self.col_offset = 1;
                    }
                    self.advance();
                } else if c == '#' {
                    while self.peek() != Some('\n') {
                        self.advance();
                    }
                } else {
                    return;
                }
            } else {
                return;
            }
        }
    }

    fn string(&mut self) -> Token<'a> {
        while self.peek() != Some('\'') {
            if self.peek() == Some('\n') {
                self.line += 1;
                self.col_offset = 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_token("Unterminated string.")
        } else {
            self.advance();
            self.make_token(TokenType::String)
        }
    }

    fn identifier(&mut self) -> Token<'a> {
        while self
            .peek()
            .map(|c| c.is_alphanumeric() || c == '_' || c.is_numeric())
            .unwrap_or(false)
        {
            self.advance();
        }
        self.make_token(self.token_type())
    }

    fn token_type(&self) -> TokenType {
        if self.current.starts_with("export") {
            TokenType::Export
        } else {
            let mut local_iter = self.current.chars();
            match local_iter.next() {
                Some('f') => {
                    if Self::remaining(local_iter.as_str(), "or") {
                        TokenType::For
                    } else if Self::remaining(local_iter.as_str(), "rom") {
                        TokenType::From
                    } else {
                        TokenType::Identifier
                    }
                }
                Some('i') => {
                    match local_iter.next() {
                        Some('f') => TokenType::If,
                        Some('n') => TokenType::In,
                        Some('m') => if Self::remaining(local_iter.as_str(), "port") {
                            TokenType::Import
                        } else {
                            TokenType::Identifier
                        }
                        _ => TokenType::Identifier
                    }
                }
                Some('l') => {
                    match local_iter.next() {
                        Some('e') => {
                            match local_iter.next() {
                                Some('t') => TokenType::Let,
                                _ => TokenType::Identifier
                            }
                        }
                        _ => TokenType::Identifier
                    }
                }
                Some('s') => {
                    if Self::remaining(local_iter.as_str(), "truct") {
                        TokenType::Struct
                    } else {
                        TokenType::Identifier
                    }
                }
                Some(c) => {
                    if c.is_uppercase() {
                        TokenType::KIdentifier
                    } else {
                        TokenType::Identifier
                    }
                }
                _ => TokenType::Identifier
            }
        }
    }

    fn remaining(iter: &str, remain: &str) -> bool  {
        iter.starts_with(remain)
    }

    fn number(&mut self) -> Token<'a> {
        while self.peek().map(|c| c.is_numeric()).unwrap_or(false) {
            self.advance();
        }

        if self.peek() == Some('.') && self.peek_next().map(|c| c.is_numeric()).unwrap_or(false) {
            self.advance();

            while self.peek().map(|c| c.is_numeric()).unwrap_or(false) {
                self.advance();
            }
        }
        self.make_token(TokenType::Number)
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.iter.next();
        self.col_offset += 1;
        self.offset += c.map(|c| c.len_utf8()).unwrap_or(0);
        c
    }

    fn peek(&mut self) -> Option<char> {
        self.iter.as_str().chars().next()
    }

    fn peek_next(&mut self) -> Option<char> {
        let mut i = self.iter.as_str().chars();
        i.next();
        i.next()
    }

    fn is_at_end(&self) -> bool {
        self.current == ""
    }

    fn make_token(&mut self, kind: TokenType) -> Token<'a> {
        let t = Token {
            kind,
            lexeme: &self.current[0..self.offset],
            line: self.line,
            col: self.col,
        };
        self.col = self.col_offset;
        t
    }

    fn error_token(&mut self, msg: &'a str) -> Token<'a> {
        let t = Token {
            kind: TokenType::Error,
            lexeme: msg,
            line: self.line,
            col: self.col,
        };
        self.col = self.col_offset;
        t
    }
}
