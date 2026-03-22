// src/compiler/mod.rs

use crate::chunk::Chunk;
use crate::common::OpCode;
use crate::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum TokenType {
    LeftParen, RightParen, Plus, Minus, Star, Slash, Semicolon,
    Identifier, Number, False, Nil, Print, True, Error, Eof,
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub t_type: TokenType,
    pub lexeme: &'a str,
    pub line: usize,
}

#[derive(PartialOrd, PartialEq)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    fn next(&self) -> Self {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::Primary,
        }
    }
}

pub struct Compiler<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
    current_token: Token<'a>,
    previous_token: Token<'a>,
    had_error: bool,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        let dummy = Token { t_type: TokenType::Error, lexeme: "", line: 0 };
        Compiler {
            source,
            start: 0,
            current: 0,
            line: 1,
            current_token: dummy,
            previous_token: dummy,
            had_error: false,
        }
    }

    // --- High Level API ---

    pub fn compile(&mut self, chunk: &mut Chunk) -> bool {
        self.advance_parser(); 

        while !self.match_token(TokenType::Eof) {
            self.statement(chunk);
        }

        self.emit_byte(chunk, OpCode::Return as u8);
        !self.had_error
    }

    // --- Parser Engine (Pratt) ---

    fn advance_parser(&mut self) {
        self.previous_token = self.current_token;
        loop {
            self.current_token = self.scan_token(); // Este é o método que o Rust não estava achando
            if self.current_token.t_type != TokenType::Error { break; }
            eprintln!("[line {}] Error: {}", self.current_token.line, self.current_token.lexeme);
            self.had_error = true;
        }
    }

    fn statement(&mut self, chunk: &mut Chunk) {
        if self.match_token(TokenType::Print) {
            self.expression(chunk);
            self.consume(TokenType::Semicolon, "Expect ';' after value.");
        } else {
            self.expression(chunk);
            self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        }
    }

    fn expression(&mut self, chunk: &mut Chunk) {
        self.parse_precedence(chunk, Precedence::Assignment);
    }

    fn parse_precedence(&mut self, chunk: &mut Chunk, precedence: Precedence) {
        self.advance_parser();
        
        match self.previous_token.t_type {
            TokenType::Number => self.number(chunk),
            TokenType::Minus => {
                self.parse_precedence(chunk, Precedence::Unary);
                self.emit_byte(chunk, OpCode::Negate as u8);
            },
            _ => {
                eprintln!("[line {}] Error: Expect expression.", self.previous_token.line);
                self.had_error = true;
                return;
            }
        }

        while precedence <= self.get_precedence(self.current_token.t_type) {
            self.advance_parser();
            let op = self.previous_token.t_type;
            let rule_precedence = self.get_precedence(op);
            self.parse_precedence(chunk, rule_precedence.next());

            match op {
                TokenType::Plus => self.emit_byte(chunk, OpCode::Add as u8),
                TokenType::Minus => self.emit_byte(chunk, OpCode::Subtract as u8),
                TokenType::Star => self.emit_byte(chunk, OpCode::Multiply as u8),
                TokenType::Slash => self.emit_byte(chunk, OpCode::Divide as u8),
                _ => {}
            }
        }
    }

    fn number(&mut self, chunk: &mut Chunk) {
        let value: f64 = self.previous_token.lexeme.parse().expect("Invalid number");
        let constant_index = chunk.add_constant(Value::number(value));
        self.emit_bytes(chunk, OpCode::Constant as u8, constant_index);
    }

    // --- Scanner Logic (The "Missing" Methods) ---

    fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() { return self.make_token(TokenType::Eof); }

        let c = self.advance();
        if c.is_ascii_digit() { return self.number_token(); }
        if c.is_alphabetic() || c == '_' { return self.identifier_token(); }

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            ';' => self.make_token(TokenType::Semicolon),
            '+' => self.make_token(TokenType::Plus),
            '-' => self.make_token(TokenType::Minus),
            '*' => self.make_token(TokenType::Star),
            '/' => self.make_token(TokenType::Slash),
            _ => self.make_token(TokenType::Error),
        }
    }

    fn number_token(&mut self) -> Token<'a> {
        while self.peek().is_ascii_digit() { self.advance(); }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() { self.advance(); }
        }
        self.make_token(TokenType::Number)
    }

    fn identifier_token(&mut self) -> Token<'a> {
        while self.peek().is_alphanumeric() || self.peek() == '_' { self.advance(); }
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

    // --- Low-level Helpers ---

    fn is_at_end(&self) -> bool { self.current >= self.source.len() }
    
    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0'; }
        self.source.as_bytes()[self.current] as char
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { return '\0'; }
        self.source.as_bytes()[self.current + 1] as char
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => { self.advance(); }
                '\n' => { self.line += 1; self.advance(); }
                _ => break,
            }
        }
    }

    fn make_token(&self, t_type: TokenType) -> Token<'a> {
        Token { t_type, lexeme: &self.source[self.start..self.current], line: self.line }
    }

    fn consume(&mut self, t_type: TokenType, message: &str) {
        if self.current_token.t_type == t_type { self.advance_parser(); return; }
        eprintln!("[line {}] Error: {}", self.current_token.line, message);
        self.had_error = true;
    }

    fn match_token(&mut self, t_type: TokenType) -> bool {
        if self.current_token.t_type != t_type { return false; }
        self.advance_parser();
        true
    }

    fn get_precedence(&self, t_type: TokenType) -> Precedence {
        match t_type {
            TokenType::Plus | TokenType::Minus => Precedence::Term,
            TokenType::Star | TokenType::Slash => Precedence::Factor,
            _ => Precedence::None,
        }
    }

    fn emit_byte(&self, chunk: &mut Chunk, byte: u8) { chunk.write(byte); }
    fn emit_bytes(&self, chunk: &mut Chunk, b1: u8, b2: u8) {
        chunk.write(b1);
        chunk.write(b2);
    }
}