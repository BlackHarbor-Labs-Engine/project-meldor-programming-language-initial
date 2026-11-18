// src/semantic_analyzer.rs
use crate::syntax::{SyntaxNode, NodeType, LiteralNode, Type, Op};

/// Performs deep semantic analysis on the Abstract Syntax Tree.
pub struct SemanticAnalyzer;

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {}
    }

    /// Analyzes the AST to ensure type correctness and structural integrity.
    /// Returns the resulting Type if successful, or an error string.
    pub fn analyze_syntax_nodes(&self, node: &SyntaxNode) -> Result<Type, String> {
        match &node.kind {
            NodeType::Literal(_lit_node) => {
                // Assuming LiteralNode has a method to determine its type
                // Since we fixed syntax.rs, we can safely assume numeric literals resolve to Integer.
                Ok(Type::Integer)
            },
            NodeType::Identifier(id) => {
                // In a real system, this would look up the symbol table for the type
                println!("Analyzing Identifier: {}", id);
                Ok(Type::Boolean)
            },
            NodeType::UnknownNode => {
                Err("Semantic Error: Encountered an unknown node type during analysis.".to_string())
            },
            NodeType::Declaration { ty, .. } => {
                Ok(ty.clone())
            },
            NodeType::Assignment { value_node, .. } => {
                // 1. Analyze the value expression to get its type
                let value_type = self.analyze_syntax_nodes(value_node)?;
                // 2. Structural validation: We assume the target type is available via context or the node itself.
                if value_type == Type::Unknown {
                    return Err("Semantic Error: Assignment value type could not be determined.".to_string());
                }
                println!("Analyzing Assignment: Value type is {:?}", value_type);
                Ok(value_type) // Return the type of the assigned value
            },
            NodeType::BinaryOp { left, right, op } => {
                // 1. Recursively analyze left and right operands
                let left_type = self.analyze_syntax_nodes(left)?;
                let right_type = self.analyze_syntax_nodes(right)?;

                // 2. Structural/Type Validation based on operator
                match op {
                    Op::Add | Op::Subtract | Op::Multiply | Op::Divide => {
                        // Arithmetic operations require numeric types
                        if left_type != Type::Integer || right_type != Type::Integer {
                            return Err(format!("Semantic Error: Binary operation {:?} requires Integer types, got {:?} and {:?}.", op, left_type, right_type));
                        }
                        // Assuming Integer is the result type for arithmetic
                        Ok(Type::Integer)
                    },
                    Op::Equal | Op::NotEqual => {
                        // Comparison operations require compatible types
                        if left_type != right_type {
                            return Err(format!("Semantic Error: Comparison operation {:?} requires matching types, got {:?} and {:?}.", op, left_type, right_type));
                        }
                        // Using a placeholder type for Boolean if Type::Boolean is missing
                        Ok(Type::Unknown) 
                    },
                    Op::Unsupported => { return Err(format!("Semantic Error: Unsupported binary operation {:?}.", op)); }
                        // Default handling for other ops
                        Ok(Type::Unknown)
                    }
                }
            },
        }
    }
}
