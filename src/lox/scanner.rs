use std::{char, io::Error};

use super::token::{Object, Token, TokenType};

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

    fn scan_token(&mut self) {
        let ch = self.advance().unwrap();

        match ch {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.r#match('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            },
            '=' => {
                if self.r#match('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            },
            '<' => {
                if self.r#match('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            },
            '>' => {
                if self.r#match('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            },
            _ => eprintln!("Unexpected character ecncountered."),
        }
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn advance(&mut self) -> Option<char> {
        let _current = self.current;
        self.current += 1;

        return self.source.chars().nth(_current);
    }

    fn r#match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None)
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Object>) {
        let text = &self.source[self.start..self.current];

        self.tokens.push(Token {
            token_type,
            lexeme: text.to_string(),
            literal,
            line: self.line,
        })
    }
}
