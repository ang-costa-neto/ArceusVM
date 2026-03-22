use crate::chunk::Chunk;
use crate::common::OpCode;
use crate::value::Value;

pub struct Compiler<'a> {
    source: &'a str,
    start: usize,   // Start of the current token being scanned
    current: usize, // Current character being looked at
    line: usize,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        Compiler {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// The main entry point for the compiler.
    pub fn compile(&mut self, chunk: &mut Chunk) -> bool {
        // For now, we will just scan and print tokens to verify the Lexer.
        // In the next step, we'll implement the Parser to emit bytecode.
        println!("--- Token Scanning Phase ---");
        loop {
            let token = self.scan_token();
            println!("[Line {}] Type: {:?}, Lexeme: '{}'", token.line, token.t_type, token.lexeme);
            
            if token.t_type == TokenType::Eof { break; }
        }
        true
    }

    /// Scans the next token from the source string.
    fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();

        // Numeric Literals
        if c.is_ascii_digit() { return self.number(); }
        // Identifiers & Keywords
        if c.is_alphabetic() || c == '_' { return self.identifier(); }

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '+' => self.make_token(TokenType::Plus),
            '-' => self.make_token(TokenType::Minus),
            '*' => self.make_token(TokenType::Star),
            '/' => self.make_token(TokenType::Slash),
            ';' => self.make_token(TokenType::Semicolon),
            _ => self.error_token("Unexpected character."),
        }
    }

    // --- Scanner Helpers ---

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0'; }
        self.source.as_bytes()[self.current] as char
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.is_at_end() { break; }
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => { self.advance(); }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn make_token(&self, t_type: TokenType) -> Token<'a> {
        Token {
            t_type,
            lexeme: &self.source[self.start..self.current],
            line: self.line,
        }
    }

    fn error_token(&self, message: &'static str) -> Token<'a> {
        Token {
            t_type: TokenType::Error,
            lexeme: message,
            line: self.line,
        }
    }

    // --- Literals Processing ---

    fn number(&mut self) -> Token<'a> {
        while self.peek().is_ascii_digit() { self.advance(); }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // Consume the "."
            while self.peek().is_ascii_digit() { self.advance(); }
        }

        self.make_token(TokenType::Number)
    }

    fn identifier(&mut self) -> Token<'a> {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        
        let text = &self.source[self.start..self.current];
        let t_type = match text {
            "print" => TokenType::Print,
            "nil" => TokenType::Nil,
            "true" => TokenType::True,
            "false" => TokenType::False,
            _ => TokenType::Identifier,
        };
        
        self.make_token(t_type)
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { return '\0'; }
        self.source.as_bytes()[self.current + 1] as char
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen, RightParen, Plus, Minus, Star, Slash, Semicolon,
    Identifier, Number,
    False, Nil, Print, True,
    Error, Eof,
}

pub struct Token<'a> {
    pub t_type: TokenType,
    pub lexeme: &'a str,
    pub line: usize,
}