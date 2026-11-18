use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Literal(i64),
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
