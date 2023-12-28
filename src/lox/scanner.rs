use std::fmt::Error;

use super::token::{Token, TokenType};

#[allow(unused)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

#[allow(unused)]
impl Scanner {
    pub fn new(source: String) -> Scanner {
        return Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        };
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Error> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });

        return Ok(self.tokens.clone());
    }

    fn scan_token(&self) {}

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }
}
