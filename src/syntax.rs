use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    Bool,
    String,
    Custom(String),
    Unknown,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Bool => write!(f, "bool"),
            Type::String => write!(f, "string"),
            Type::Custom(s) => write!(f, "custom(\"{}\")", s),
            Type::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator { 
    Add, 
    Sub, 
    Mul, 
    Div, 
    And, 
    Or, 
    Not,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::And => write!(f, "&&"),
            Operator::Or => write!(f, "||"),
            Operator::Not => write!(f, "!"),
        }
    }
} 
    Add, 
    Sub, 
    Mul, 
    Div, 
    And, 
    Or, 
    Not,

    // Implement Display for use in interpreter/logging
    #[allow(dead_code)]
    fn to_display(&self) -> &'static str { 
        match self { 
            Operator::Add => "+", 
            Operator::Sub => "-", 
            Operator::Mul => "*", 
            Operator::Div => "/", 
            Operator::And => "&&", 
            Operator::Or => "||", 
            Operator::Not => "!", 
        }
    }
}
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    pub value: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SyntaxNode {
    Literal(Literal),
    BinaryOp { left: Box<SyntaxNode>, op: Operator, right: Box<SyntaxNode> },
    Variable(String),
    FunctionCall { name: String, args: Vec<Box<SyntaxNode>> },
    UnknownNode, // Add more nodes as needed
}

impl Display for SyntaxNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxNode::Literal(l) => write!(f, "Literal({:?})", l),
            SyntaxNode::BinaryOp { left, op, right } => write!(f, "BinaryOp({:?}, {:?})", op, (left, right)),
            SyntaxNode::Variable(name) => write!(f, "Variable("{}")", name),
            SyntaxNode::FunctionCall { name, args } => write!(f, "Call("{}", {:?})", name, args),
        }
    }
}

pub struct Analyzer {
    pub types: std::collections::HashMap<String, Type>,
}

impl Analyzer {
    pub fn new() -> Self {
        Analyzer { types: std::collections::HashMap::new() }
    }

    pub fn analyze_syntax_nodes(&mut self, nodes: Vec<SyntaxNode>) -> Result<(), String> {
        // Placeholder for actual analysis logic
        println!("Running semantic analysis on {} nodes...", nodes.len());
        Ok(())
    }
}
