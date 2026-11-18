use std::fmt;
use std::error::Error;

// --- Core AST/Syntax Definitions ---

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralNode {
    Integer(i64),
    String(String),
}

#[derive(Debug)]
pub struct ParseError { 
    details: String 
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing Error: {}", self.details)
    }
}

impl Error for ParseError {}

// --- Type System ---

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    String,
    Bool,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    String(String),
    Bool(bool),
}

// --- Node Types and AST ---

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    // Structural Nodes
    Declaration { name: String, ty: Type },
    Assignment { name: String, value_node: Box<SyntaxNode> },
    BinaryOp { left: Box<SyntaxNode>, right: Box<SyntaxNode>, op: String },
    Literal(LiteralNode), // <-- Correctly using LiteralNode
    Identifier(String),
    UnknownNode,
}

#[derive(Debug, Clone)]
pub struct SyntaxNode {
    pub kind: NodeType,
}
