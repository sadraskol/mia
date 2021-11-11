use crate::token::Token;
use crate::token::TokenType;
use std::str::Chars;

pub struct Scanner<'a> {
    iter: Chars<'a>,
    current: &'a str,
    offset: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn init(source: &'a str) -> Self {
        Scanner {
            iter: source.chars(),
            current: source,
            offset: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self, debug: bool) -> Token<'a> {
        self.skip_whitespace();
        self.current = self.iter.as_str();
        self.offset = 0;
        let t = if let Some(c) = self.advance() {
            if c.is_alphabetic() || c == '_' {
                return self.identifier();
            }
            if c.is_numeric() {
                return self.number();
            }

            match c {
                '[' => self.make_token(TokenType::LeftBracket),
                ']' => self.make_token(TokenType::RightBracket),
                '(' => self.make_token(TokenType::LeftParen),
                ')' => self.make_token(TokenType::RightParen),
                '=' => self.make_token(TokenType::Equal),
                '.' => self.make_token(TokenType::Dot),
                '?' => self.make_token(TokenType::Question),
                '\n' => {
                    let t = self.make_token(TokenType::NewLine);
                    self.line += 1;
                    t
                }
                '"' => self.string(),
                '`' => self.process(),
                _ => self.error_token("Unexpected character."),
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
                if c != '\n' && c.is_whitespace() {
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

    fn process(&mut self) -> Token<'a> {
        while self.peek() != Some('`') {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_token("Unterminated process.")
        } else {
            self.advance();
            self.make_token(TokenType::Process)
        }
    }

    fn string(&mut self) -> Token<'a> {
        while self.peek() != Some('"') {
            if self.peek() == Some('\n') {
                self.line += 1;
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
        match self.current[] {
            'i' => {
                if self.iter.peek_next() {

                }
            }
            _ => TokenType::Identifier
        }
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

    fn make_token(&self, kind: TokenType) -> Token<'a> {
        Token {
            kind,
            lexeme: &self.current[0..self.offset],
            line: self.line,
        }
    }

    fn error_token(&self, msg: &'a str) -> Token<'a> {
        Token {
            kind: TokenType::Error,
            lexeme: msg,
            line: self.line,
        }
    }
}
