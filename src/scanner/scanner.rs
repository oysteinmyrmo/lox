use crate::scanner::token::{Literal, Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let mut tokens: Vec<Token> = Vec::new();
        tokens.reserve(1024); // Assume tons of code!

        Scanner {
            source,
            tokens,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        // Insert token for end of file.
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".into(),
            literal: Literal::None,
            line: self.line,
        });
    }

    fn scan_token(&mut self) {}
}
