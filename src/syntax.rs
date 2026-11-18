/* Corrected content for src/syntax.rs */
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
    pub details: String 
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

// --- Token Definitions (Fix for parser.rs) ---
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Literal(LiteralNode),
    Operator(String),
    LParen, 
    RParen,
    // ... other kinds
}
    // Structural Nodes
    Declaration { name: String, ty: Type },
    Assignment { name: String, value_node: Box<SyntaxNode> },
    BinaryOp { left: Box<SyntaxNode>, right: Box<SyntaxNode>, op: String },
    Literal(LiteralNode), // <-- Correctly using LiteralNode
    Identifier(String),
    UnknownNode,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(Debug, Clone, PartialEq)]
#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxNode {
    pub kind: NodeType,
}

impl SyntaxNode {
    /// Recursively determines the resulting Type of this node.
    pub fn get_type(&self) -> Type {
        match &self.kind {
            NodeType::Literal(LiteralNode::Integer(_)) => Type::Int,
            NodeType::Literal(LiteralNode::String(_)) => Type::String,
            NodeType::Identifier(_) => Type::Unknown, // Needs symbol table context for real resolution
            NodeType::Declaration { ty, .. } => ty.clone(),
            NodeType::Assignment { value_node, .. } => value_node.get_type(),
            NodeType::BinaryOp { left, right, .. } => {
                let left_type = left.get_type();
                let right_type = right.get_type();
                // Simplified type checking for now; real logic would check op compatibility
                if left_type == right_type { left_type } else { Type::Unknown }
            },
            NodeType::UnknownNode => Type::Unknown,
        }
    }
}

impl fmt::Display for SyntaxNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            NodeType::Literal(LiteralNode::Integer(i)) => write!(f, "Literal(i:{})", i),
            NodeType::Literal(LiteralNode::String(s)) => write!(f, "Literal("{}")", s),
            NodeType::Identifier(s) => write!(f, "Identifier("{}")", s),
            NodeType::Declaration { name, .. } => write!(f, "Decl("{}")", name),
            NodeType::Assignment { name, .. } => write!(f, "Assign("{}")", name),
            NodeType::BinaryOp { op, .. } => write!(f, "BinOp("{}")", op),
            NodeType::UnknownNode => write!(f, "UnknownNode"),
        }
    }
}
    /// Recursively determines the resulting Type of this node.
    pub fn get_type(&self) -> Type {
        match &self.kind {
            NodeType::Literal(LiteralNode::Integer(_)) => Type::Int,
            NodeType::Literal(LiteralNode::String(_)) => Type::String,
            NodeType::Identifier(_) => Type::Unknown, // Needs symbol table context for real resolution
            NodeType::Declaration { ty, .. } => ty.clone(),
            NodeType::Assignment { value_node, .. } => value_node.get_type(),
            NodeType::BinaryOp { left, right, .. } => {
                let left_type = left.get_type();
                let right_type = right.get_type();
                // Simplified type checking for now; real logic would check op compatibility
                if left_type == right_type { left_type } else { Type::Unknown }
            },
            NodeType::UnknownNode => Type::Unknown,
        }
    }
}

impl fmt::Display for SyntaxNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            NodeType::Literal(LiteralNode::Integer(i)) => write!(f, "Literal(i:{})", i),
            NodeType::Literal(LiteralNode::String(s)) => write!(f, "Literal("{}")", s),
            NodeType::Identifier(s) => write!(f, "Identifier("{}")", s),
            NodeType::Declaration { name, .. } => write!(f, "Decl("{}")", name),
            NodeType::Assignment { name, .. } => write!(f, "Assign("{}")", name),
            NodeType::BinaryOp { op, .. } => write!(f, "BinOp("{}")", op),
            NodeType::UnknownNode => write!(f, "UnknownNode"),
        }
    }
}
    pub kind: NodeType,
}

impl SyntaxNode {
    /// Recursively determines the resulting Type of this node.
    pub fn get_type(&self) -> Type {
        match &self.kind {
            NodeType::Literal(LiteralNode::Integer(_)) => Type::Int,
            NodeType::Literal(LiteralNode::String(_)) => Type::String,
            NodeType::Identifier(_) => Type::Unknown, // Needs symbol table context for real resolution
            NodeType::Declaration { ty, .. } => ty.clone(),
            NodeType::Assignment { value_node, .. } => value_node.get_type(),
            NodeType::BinaryOp { left, right, .. } => {
                let left_type = left.get_type();
                let right_type = right.get_type();
                // Simplified type checking for now; real logic would check op compatibility
                if left_type == right_type { left_type } else { Type::Unknown }
            },
            NodeType::UnknownNode => Type::Unknown,
        }
    }
}

impl fmt::Display for SyntaxNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            NodeType::Literal(LiteralNode::Integer(i)) => write!(f, "Literal(i:{})", i),
            NodeType::Literal(LiteralNode::String(s)) => write!(f, "Literal("{}")", s),
            NodeType::Identifier(s) => write!(f, "Identifier("{}")", s),
            NodeType::Declaration { name, .. } => write!(f, "Decl("{}")", name),
            NodeType::Assignment { name, .. } => write!(f, "Assign("{}")", name),
            NodeType::BinaryOp { op, .. } => write!(f, "BinOp("{}")", op),
            NodeType::UnknownNode => write!(f, "UnknownNode"),
        }
    }
}
    pub kind: NodeType,
}
