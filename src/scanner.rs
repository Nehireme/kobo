use crate::token::{keyword, Token, TokenType};

/// Cut `source` into tokens. Returns everything it managed to scan alongside every
/// error it found; the caller decides whether to go on.
pub fn scan(source: &str) -> (Vec<Token>, Vec<String>) {
    let mut s = Scanner {
        src: source.chars().collect(),
        start: 0,
        current: 0,
        line: 1,
        tokens: Vec::new(),
        errors: Vec::new(),
    };
    s.run();
    (s.tokens, s.errors)
}

struct Scanner {
    src: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
    errors: Vec<String>,
}

impl Scanner {
    fn run(&mut self) {
         while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }
        let line = self.tokens.last().map_or(1, |token| token.line);
        self.tokens.push(Token {
            kind: TokenType::Eof,
            lexeme: String::new(),
            line,
        });
    
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add(TokenType::LeftParen),
            ')' => self.add(TokenType::RightParen),
            '{' => self.add(TokenType::LeftBrace),
            '}' => self.add(TokenType::RightBrace),
            ',' => self.add(TokenType::Comma),
            '.' => self.add(TokenType::Dot),
            '-' => self.add(TokenType::Minus),
            '+' => self.add(TokenType::Plus),
            ';' => self.add(TokenType::Semicolon),
            '*' => self.add(TokenType::Star),
            '&' => self.add(TokenType::And),
            '|' => self.add(TokenType::Or),
            'else' => self.add(TokenType::Else),
            'fun' => self.add(TokenType::Fun),
            'if' => self.add(TokenType::If),
            'nil' => self.add(TokenType::Nil),
            'print' => self.add(TokenType::Print),
            'return' => self.add(TokenType::Return),
            'true' => self.add(TokenType::True),
            'var' => self.add(TokenType::Var),
            'while' => self.add(TokenType::While),
            '!' => {
                let kind = if self.matches('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add(kind);
            }
            '=' => {
                let kind = if self.matches('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add(kind);
            }
            '<' => {
                let kind = if self.matches('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add(kind);
            }
            '>' => {
                let kind = if self.matches('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add(kind);
            }
    }

    fn string(&mut self) {
    }

    fn number(&mut self) {
      
    }

    fn identifier(&mut self) {
        
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let word:String = self.src[self.start..self.current].iter().collect();

        self.add(keyword(&word).unwrap_or(TokenType::Identifier));
    }

    // --- primitives ---------------------------------------------------------------

    fn at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn advance(&mut self) -> char {
        let c = self.src[self.current];
        self.current += 1;
        c
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.at_end() || self.src[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.src[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.src.len() {
            '\0'
        } else {
            self.src[self.current + 1]
        }
    }

    fn add(&mut self, kind: TokenType) {
        self.tokens.push(Token {
            kind,
            lexeme: self.src[self.start..self.current].iter().collect(),
            line: self.line,
        });
    }

    fn error(&mut self, line: usize, message: &str) {
        self.errors.push(format!("[line {}] Error: {}", line, message));
    }
}}
