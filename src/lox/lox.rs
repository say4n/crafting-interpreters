use std::{fs::read_to_string, io::stdin, path::Path};

use super::{scanner::Scanner, token::Token};

#[allow(unused)]
struct Lox {
    had_error: bool,
}

#[allow(unused)]
impl Lox {
    pub fn new() -> Lox {
        return Lox { had_error: false };
    }

    pub fn main(&mut self, args: &[String]) {
        self.had_error = false;

        match args.len() {
            0 => self.run_prompt(),
            1 => self.run_file(&args[0]),
            _ => {
                panic!("Usage: lox [script]");
            }
        }
    }

    fn run_file(&mut self, path_string: &String) {
        let path = Path::new(path_string);
        let source = read_to_string(path).unwrap();

        self.run(source);

        if self.had_error {
            panic!("Exiting.")
        }
    }

    fn run_prompt(&mut self) {
        let mut source = String::new();
        let stdin = stdin();

        loop {
            let mut buffer = String::new();
            let nbytes = stdin.read_line(&mut buffer).unwrap();

            if nbytes == 0 {
                break;
            } else {
                source += &buffer
            }
        }

        self.run(source);
    }

    fn run(&mut self, source: String) {
        let scanner = Scanner::new(source);
        let mut tokens: Vec<Token> = vec![];
        
        if let Ok(_tokens) = scanner.scan_tokens() {
            tokens = _tokens
        } else {
            self.had_error = true
        }

        for token in tokens {
            println!("{:?}", token)
        }
    }

    fn error(&self, line: usize, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn report(&self, line: usize, location: String, message: String) {
        println!("[line: {line}] Error{location}: {message}")
    }
}
