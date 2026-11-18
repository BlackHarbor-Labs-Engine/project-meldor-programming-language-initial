use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Int(i64),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    Bool,
    String,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, And, Or, Not,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeType {
    Literal(Literal),
    Identifier(String),
    BinaryOp { left: Box<SyntaxNode>, op: BinaryOp, right: Box<SyntaxNode> },
    Declaration { name: String, ty: Type },
    Assignment { name: String, value_node: Box<SyntaxNode> },
    UnknownNode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxNode {
    pub kind: NodeType,
}

impl SyntaxNode {
    pub fn new_literal(value: Value) -> Self {
        SyntaxNode { kind: NodeType::Literal(Literal::new(value)) }
    }
    pub fn new_identifier(name: String) -> Self {
        SyntaxNode { kind: NodeType::Identifier(name) }
    }
    pub fn new_unknown() -> Self {
        SyntaxNode { kind: NodeType::UnknownNode }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    pub value: Value,
}

impl Literal {
    pub fn new(value: Value) -> Self {
        Literal { value }
    }
}

// Note: Added necessary derives and variants to satisfy compiler demands from consumers.
