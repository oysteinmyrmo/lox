use crate::logger::logger::LoxLogger;
use crate::scanner::token::{Literal, Token, TokenType, KEYWORDS_MAP};

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

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
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
            literal: Literal::Nil,
            line: self.line,
        });
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
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
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }
            // Ignore whitespace
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.add_string();
            }
            c => {
                if c.is_ascii_digit() {
                    self.add_number();
                } else if Self::is_alpha_or_underscore(c) {
                    self.add_identifier();
                } else {
                    LoxLogger::scanner_error(self.line, format!("Unknown character found: {}", c));
                }
            }
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.current_char() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn current_char(&self) -> char {
        self.source.as_bytes()[self.current] as char
    }

    fn next_char(&self) -> char {
        self.source.as_bytes()[self.current + 1] as char
    }

    fn is_alpha_or_underscore(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alpha_numeric_or_underscore(c: char) -> bool {
        Self::is_alpha_or_underscore(c) || c.is_ascii_digit()
    }

    fn advance(&mut self) -> char {
        let c = self.current_char();
        self.current += 1;
        c
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        };
        self.current_char()
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.next_char()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Literal::Nil)
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        let sub = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: String::from(sub),
            literal,
            line: self.line,
        })
    }

    fn add_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.is_at_end() {
            LoxLogger::scanner_error(self.line, format!("{}", "Unterminated string literal"));
            return;
        }

        let value = self.source[self.start + 1..self.current].to_string();
        self.add_token_with_literal(TokenType::String, Literal::String(value));
        self.advance(); // Skip the trailing '"'
    }

    fn add_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        if let Ok(number) = self.source[self.start..self.current].parse::<f64>() {
            self.add_token_with_literal(TokenType::Number, Literal::Number(number));
        } else {
            LoxLogger::scanner_error(self.line, "Invalid number literal".into());
        };
    }

    fn add_identifier(&mut self) {
        while Self::is_alpha_numeric_or_underscore(self.peek()) {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        let token_type = KEYWORDS_MAP
            .get(text.as_str())
            .unwrap_or(&TokenType::Identifier);
        self.add_token(token_type.clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::scanner::Scanner;
    use crate::scanner::token::{Literal, Token, TokenType};

    #[test]
    fn test_string_literal() {
        const SCRIPT: &str = r#""This is a string literal""#;
        let mut scanner = Scanner::new(SCRIPT.into());
        scanner.scan_tokens();

        let tokens = scanner.tokens();
        assert_eq!(tokens.len(), 2);

        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::String,
                lexeme: SCRIPT.into(),
                literal: Literal::String(SCRIPT.into()),
                line: 1,
            }
        );

        assert_eq!(
            tokens[1],
            Token {
                token_type: TokenType::Eof,
                lexeme: "".into(),
                literal: Literal::Nil,
                line: 1,
            }
        );
    }

    #[test]
    fn test_number_literal() {
        const SCRIPT: &str = "123.456";
        const NUMBER: f64 = 123.456;

        let mut scanner = Scanner::new(SCRIPT.into());
        scanner.scan_tokens();

        let tokens = scanner.tokens();
        assert_eq!(tokens.len(), 2);

        assert_eq!(
            tokens[0],
            Token {
                token_type: TokenType::Number,
                lexeme: SCRIPT.into(),
                literal: Literal::Number(NUMBER),
                line: 1,
            }
        );

        assert_eq!(
            tokens[1],
            Token {
                token_type: TokenType::Eof,
                lexeme: "".into(),
                literal: Literal::Nil,
                line: 1,
            }
        );
    }
}
