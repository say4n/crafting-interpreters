use std::fmt::Error;

use super::token::Token;

#[allow(unused)]
pub struct Scanner {
    source: String,
}

#[allow(unused)]
impl Scanner {
    pub fn new(source: String) -> Scanner {
        return Scanner { source };
    }

    pub fn scan_tokens(&self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();
        return Ok(tokens);
    }
}
