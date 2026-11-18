// src/syntax.rs
//! Defines the Abstract Syntax Tree (AST) structure for Meldor.

use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Bool,
    String,
    Custom(String),
}

// --- Core AST Node Definition ---
#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxNode {
    // Basic types
    Literal(Literal),
    Identifier(String),
    
    // Structure nodes
    Declaration { name: String, ty: Type },
    Assignment { name: String, value_node: Box<SyntaxNode> },
    BinaryOp { left: Box<SyntaxNode>, op: Operator, right: Box<SyntaxNode> },
    
    // Control flow (simplified)
    IfStatement { condition: Box<SyntaxNode>, then_block: Vec<SyntaxNode>, else_block: Option<Vec<SyntaxNode>> },
    
    // Placeholder for complex nodes
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i32),
    Boolean(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
}

// --- Trait Definition for Analysis ---
// This trait must be available to SemanticAnalyzer in analyze.rs
pub trait Analyzer {
    fn analyze(&self, node: &SyntaxNode) -> Result<(), String>;
}

// Default implementation for nodes that don't require deep analysis
impl Analyzer for SyntaxNode {
    fn analyze(&self, node: &SyntaxNode) -> Result<(), String> {
        // Default implementation does nothing, allowing specific nodes to override
        Ok(())
    }
}
