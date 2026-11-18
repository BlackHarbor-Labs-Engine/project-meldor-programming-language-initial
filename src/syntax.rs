#[derive(Debug, Clone)]
pub struct Span {
    pub line: u32,
    pub col: u32,
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(line: u32, col: u32, start: usize, end: usize) -> Self {
        Span { line, col, start, end }
    }
}

use std::fmt;

#[derive(Debug, Clone)]
pub enum SyntaxNode {
    Keyword(String),
    Literal { value: String, span: Span },
    Identifier(String),
}

impl fmt::Display for SyntaxNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxNode::Keyword(s) => write!(f, "{}", s),
            SyntaxNode::Literal { value, .. } => write!(f, "{}", value),
            SyntaxNode::Identifier(s) => write!(f, "{}", s),
        }
    }
}

impl std::string::ToString for SyntaxNode {
    fn to_string(&self) -> String {
        self.to_string()
    }
}

// This function is not used in the current code but we'll keep it for completeness
pub fn parse_literal(input: &str) -> Result<(&str, SyntaxNode), SyntaxNode> {
    // Simplified version - in practice this would be more complex
    if input.starts_with(|c: char| c.is_ascii_digit()) {
        let mut end = 0;
        for (i, c) in input.chars().enumerate() {
            if c.is_ascii_digit() {
                end = i + 1;
            } else {
                break;
            }
        }
        let number_str = &input[..end];
        Ok((
            &input[end..],
            SyntaxNode::Literal { 
                value: number_str.to_string(), 
                span: Span::new(0, 0, 0, 0) 
            }
        ))
    } else {
        Err(SyntaxNode::Keyword("unknown".to_string()))
    }
}
