// src/lexer.rs
//! The lexer for the Meldor language.



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

    // Helper function to consume the current character and advance
    fn advance(&mut self) {
        self.read_char();
    }

    // This function was previously called incorrectly in the loop.
    // We need a mechanism to loop until the input is exhausted.
    // The original logic seemed to rely on next_token() being called repeatedly.
    // I'm restructuring tokenize to be a loop that consumes all tokens.
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.ch.is_some() {
            let token = self.next_token();
            tokens.push(token);
        }
        tokens
    }

    // This function needs to be defined/adapted to return a single token
    // and advance the state, which is what the original code seemed to imply
    // when it was called inside the loop.
    fn next_token(&mut self) -> Token {
        let token = match self.ch {
            Some(ch) => {
                if ch.is_alphabetic() {
                    self.read_identifier()
                } else if ch.is_whitespace() {
                    self.read_whitespace();
                    // After reading whitespace, we must read the next token
                    self.next_token()
                } else if ch.is_digit(10) {
                    self.read_number()
                } else {
                    self.read_special_token()
                }
            }
            None => Token::Unknown, // Should not happen if called correctly
        };
        // Crucially, advance after processing the token to move to the next character
        self.advance(); 
        token
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some(ch) = self.ch {
            if ch.is_alphabetic() || ch.is_digit(10) {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        Token::Identifier(identifier)
    }

    fn read_number(&mut self) -> Token {
        let mut number_str = String::new();
        while let Some(ch) = self.ch {
            if ch.is_digit(10) {
                number_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(number_str.parse().unwrap())
    }

    fn read_special_token(&mut self) -> Token {
        let ch = self.ch.unwrap();
        // Consume the character immediately after identifying it as special
        self.advance(); 
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
                self.advance();
            } else {
                break;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
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
