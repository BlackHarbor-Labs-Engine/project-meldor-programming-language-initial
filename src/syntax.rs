// src/syntax.rs
//! Defines the core Abstract Syntax Tree (AST) nodes and structural elements for Meldor.

use std::fmt;

/// Represents the positional span of a token/node in the source code.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Span { start, end, line, column }
    }
}

/// Enumeration of all possible types in Meldor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    Int,
    String,
    Bool,
    Unknown,
}

/// The core structure representing a node in the Abstract Syntax Tree.
#[derive(Debug, Clone)]
pub enum SyntaxNode {
    // Lexical Tokens
    Keyword(String),
    Identifier(String),
    Literal { value: String, span: Span },

    // AST Structure
    Declaration { name: String, ty: Type },
    Assignment { name: String, value_node: SyntaxNode },
    // ... other nodes like BinaryOp, FunctionCall, etc. would go here
}
