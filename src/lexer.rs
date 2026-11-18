// src/lexer.rs
//! The lexer for the Meldor language.

use std::collections::HashMap;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position < self.input.len() {
            self.ch = Some(self.input.chars().nth(self.read_position).unwrap());
            self.position = self.read_position;
            self.read_position += 1;
        } else {
            self.ch = None;
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut token = Token::new();
        match self.ch {
            Some(ch) => {
                if ch.is_alphabetic() {
                    token = self.read_identifier();
                } else if ch.is_whitespace() {
                    self.read_whitespace();
                    token = self.next_token();
                } else if ch.is_digit(10) {
                    token = self.read_number();
                } else {
                    token = self.read_special_token();
                }
            }
            None => {
                token = Token::new();
            }
        }
        token
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some(ch) = self.ch {
            if ch.is_alphabetic() || ch.is_digit(10) {
                identifier.push(ch);
                self.read_char();
            } else {
                break;
            }
        }
        Token::Identifier(identifier)
    }

    fn read_number(&mut self) -> Token {
        let mut number = String::new();
        while let Some(ch) = self.ch {
            if ch.is_digit(10) {
                number.push(ch);
                self.read_char();
            } else {
                break;
            }
        }
        Token::Number(number.parse().unwrap())
    }

    fn read_special_token(&mut self) -> Token {
        let ch = self.ch.unwrap();
        match ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '=' => Token::Equal,
            _ => Token::Unknown,
        }
    }

    fn read_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Identifier(String),
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    Unknown,
}

impl Token {
    fn new() -> Self {
        Token::Unknown
    }
}
