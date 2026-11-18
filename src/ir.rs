//! Intermediate Representation (IR) module for the Meldor programming language.
//! This module handles early structural validation and optimization passes to improve
//! the semantic analyzer's performance.

use std::collections::HashMap;
use crate::syntax::{SyntaxNode, Span};

/// Represents an IR node in the intermediate representation.
#[derive(Debug, Clone)]
pub enum IrNode {
    /// Function definition node
    FunctionDef {
        name: String,
        params: Vec<(String, IrType)>,
        return_type: IrType,
        body: Vec<IrNode>,
        span: Span,
    },
    /// Variable declaration node
    VarDecl {
        name: String,
        ty: IrType,
        initializer: Option<Box<IrNode>>,
        span: Span,
    },
    /// Assignment node
    Assignment {
        lhs: String,
        rhs: Box<IrNode>,
        span: Span,
    },
    /// Literal value node
    Literal {
        value: String,
        ty: IrType,
        span: Span,
    },
    /// Binary operation node
    BinaryOp {
        op: String,
        left: Box<IrNode>,
        right: Box<IrNode>,
        ty: IrType,
        span: Span,
    },
    /// Unary operation node
    UnaryOp {
        op: String,
        operand: Box<IrNode>,
        ty: IrType,
        span: Span,
    },
    /// Block node (for scoping)
    Block {
        statements: Vec<IrNode>,
        span: Span,
    },
    /// Return statement node
    Return {
        value: Option<Box<IrNode>>,
        span: Span,
    },
    /// Variable use node
    VariableUse {
        name: String,
        span: Span,
    },
}

/// Represents a type in the IR.
#[derive(Debug, Clone, PartialEq)]
pub enum IrType {
    Int,
    Float,
    Bool,
    String,
    Void,
    Struct(String),
    Pointer(Box<IrType>),
    Array(Box<IrType>, usize),
    Function(Vec<IrType>, Box<IrType>),
}

impl IrType {
    /// Create a new integer type
    pub fn int() -> Self {
        IrType::Int
    }

    /// Create a new float type
    pub fn float() -> Self {
        IrType::Float
    }

    /// Create a new boolean type
    pub fn bool() -> Self {
        IrType::Bool
    }

    /// Create a new string type
    pub fn string() -> Self {
        IrType::String
    }

    /// Create a new void type
    pub fn void() -> Self {
        IrType::Void
    }

    /// Create a new struct type
    pub fn struct_type(name: &str) -> Self {
        IrType::Struct(name.to_string())
    }

    /// Create a new pointer type
    pub fn pointer(ty: IrType) -> Self {
        IrType::Pointer(Box::new(ty))
    }

    /// Create a new array type
    pub fn array(ty: IrType, size: usize) -> Self {
        IrType::Array(Box::new(ty), size)
    }

    /// Create a new function type
    pub fn function(params: Vec<IrType>, return_ty: IrType) -> Self {
        IrType::Function(params, Box::new(return_ty))
    }
}

/// Intermediate representation builder and optimizer.
pub struct IrBuilder {
    /// The IR nodes collected so far
    pub nodes: Vec<IrNode>,
    /// Type mapping for quick lookups
    pub types: HashMap<String, IrType>,
    /// Current scope stack
    pub scopes: Vec<HashMap<String, IrType>>,
}

impl IrBuilder {
    /// Create a new IR builder
    pub fn new() -> Self {
        IrBuilder {
            nodes: Vec::new(),
            types: HashMap::new(),
            scopes: vec![HashMap::new()],
        }
    }

    /// Enter a new scope
    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Exit the current scope
    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    /// Get the current scope
    pub fn current_scope(&mut self) -> &mut HashMap<String, IrType> {
        self.scopes.last_mut().unwrap()
    }

    /// Insert a variable into the current scope
    pub fn insert_var(&mut self, name: String, ty: IrType) {
        self.current_scope().insert(name, ty);
    }

    /// Resolve a variable by name in the scopes
    pub fn resolve_var(&self, name: &str) -> Option<&IrType> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty);
            }
        }
        None
    }

    /// Add a node to the IR
    pub fn add_node(&mut self, node: IrNode) {
        self.nodes.push(node);
    }

    /// Perform early structural validation on the IR
    pub fn validate(&self) -> Result<(), String> {
        // Check for undefined variables
        for node in &self.nodes {
            match node {
                IrNode::Assignment { lhs, .. } => {
                    if self.resolve_var(lhs).is_none() {
                        return Err(format!("Undefined variable '{}'", lhs));
                    }
                },
                _ => (),
            }
        }
        Ok(())
    }

    /// Perform optimization passes on the IR
    pub fn optimize(&mut self) -> Result<(), String> {
        // Simple optimization: remove unreachable code
        let mut optimized_nodes = Vec::new();
        for node in &self.nodes {
            match node {
                IrNode::Return { .. } => {
                    // If we encounter a return, everything after it is unreachable
                    optimized_nodes.push(node.clone());
                    break;
                },
                _ => {
                    optimized_nodes.push(node.clone());
                }
            }
        }
        self.nodes = optimized_nodes;
        Ok(())
    }

    /// Convert syntax nodes to IR nodes
    pub fn from_syntax(&mut self, syntax_nodes: Vec<SyntaxNode>) -> Result<(), String> {
        for node in syntax_nodes {
            match node {
                SyntaxNode::Keyword(ref value) => {
                    if *value == "fn" {
                        // Function definition placeholder
                        self.add_node(IrNode::FunctionDef {
                            name: String::new(),
                            params: Vec::new(),
                            return_type: IrType::void(),
                            body: Vec::new(),
                            span: Span::new(1, 1, 0, 2),
                        });
                    } else if *value == "let" {
                        // Variable declaration placeholder
                        self.add_node(IrNode::VarDecl {
                            name: String::new(),
                            ty: IrType::int(),
                            initializer: None,
                            span: Span::new(1, 1, 0, 2),
                        });
                    }
                },
                SyntaxNode::Identifier(ref name) => {
                    // Placeholder for identifier usage
                    self.add_node(IrNode::VariableUse {
                        name: name.clone(),
                        span: Span::new(1, 1, 0, 2),
                    });
                },
                SyntaxNode::Literal { value, .. } => {
                    // Handle literal values
                    let ty = if value.starts_with('"') {
                        IrType::string()
                    } else if value.parse::<i64>().is_ok() {
                        IrType::int()
                    } else if value.parse::<f64>().is_ok() {
                        IrType::float()
                    } else {
                        return Err(format!("Unknown literal type: {}", value));
                    };
                    self.add_node(IrNode::Literal {
                        value,
                        ty,
                        span: Span::new(1, 1, 0, 2),
                    });
                },
                _ => {
                    // Handle other node types as needed
                }
            }
        }
        Ok(())
    }

    /// Get a reference to the IR nodes
    pub fn get_nodes(&self) -> &[IrNode] {
        &self.nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ir_builder_creation() {
        let builder = IrBuilder::new();
        assert_eq!(builder.nodes.len(), 0);
        assert_eq!(builder.scopes.len(), 1);
    }

    #[test]
    fn test_scope_management() {
        let mut builder = IrBuilder::new();
        builder.enter_scope();
        assert_eq!(builder.scopes.len(), 2);
        builder.exit_scope();
        assert_eq!(builder.scopes.len(), 1);
    }

    #[test]
    fn test_variable_resolution() {
        let mut builder = IrBuilder::new();
        builder.insert_var("x".to_string(), IrType::int());
        assert!(builder.resolve_var("x").is_some());
        assert!(builder.resolve_var("y").is_none());
    }

    #[test]
    fn test_validation_pass() {
        let mut builder = IrBuilder::new();
        // Add a node that references an undefined variable
        builder.add_node(IrNode::Assignment {
            lhs: "undefined".to_string(),
            rhs: Box::new(IrNode::Literal {
                value: "42".to_string(),
                ty: IrType::int(),
                span: Span::new(1, 1, 0, 2),
            }),
            span: Span::new(1, 1, 0, 2),
        });

        let result = builder.validate();
        assert!(result.is_err());
    }
}
