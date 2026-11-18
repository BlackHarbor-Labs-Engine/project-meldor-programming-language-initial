// src/semantic_analyzer.rs
//! This module implements a semantic analyzer for the Meldor language.
//! It tracks type information and validates type consistency across syntax nodes.

use std::collections::HashMap;

use crate::syntax::{SyntaxNode, Type, Literal};

/// Represents a semantic analyzer that tracks types of variables and functions.
pub struct SemanticAnalyzer {
    types: HashMap<String, Type>,
}

impl SemanticAnalyzer {
    /// Creates a new `SemanticAnalyzer` with an empty type registry.
    pub fn new() -> Self {
        SemanticAnalyzer {
            types: HashMap::new(),
        }
    }

    /// Analyzes a collection of syntax nodes to check for semantic errors (e.g., type mismatches).
    pub fn analyze_syntax_nodes(&mut self, nodes: Vec<SyntaxNode>) -> Result<(), String> {
        for node in nodes {
            match node {
                SyntaxNode::Declaration { name, ty } => {
                    if self.types.contains_key(&name) {
                        return Err(format!("Semantic Error: Variable '{}' already declared.", name));
                    }
                    self.types.insert(name, ty);
                }
                SyntaxNode::Assignment { name, value_node } => {
                    let expected_type = self.types.get(&name).cloned().unwrap_or(Type::Unknown);
                    let actual_type = match &*value_node {
                        SyntaxNode::Literal(Literal::String(_)) => Type::String, // Assuming string literals are strings for now
                        SyntaxNode::Identifier(id) => self.types.get(&id.to_string()).cloned().unwrap_or(Type::Unknown),
                        _ => Type::Unknown,
                    };

                    if expected_type != actual_type && expected_type != Type::Unknown {
                        return Err(format!("Semantic Error: Type mismatch for '{}'. Expected {:?}, found {:?}.", name, expected_type, actual_type));
                    }
                }
                // Other node types would be handled here
                _ => {}
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{SyntaxNode, Type};

    #[test]
    fn test_semantic_analyzer_basic_declaration() {
        let mut analyzer = SemanticAnalyzer::new();
        let nodes = vec![SyntaxNode::Declaration { name: "x".to_string(), ty: Type::Int }];
        assert!(analyzer.analyze_syntax_nodes(nodes).is_ok());
        assert_eq!(analyzer.types.get("x"), Some(&Type::Int));
    }

    #[test]
    fn test_semantic_analyzer_type_mismatch() {
        let mut analyzer = SemanticAnalyzer::new();
        // Declare x as Int, then try to assign a string literal (represented as Unknown for simplicity in this test)
        analyzer.types.insert("x".to_string(), Type::Int);
        let nodes = vec![SyntaxNode::Assignment {
            name: "x".to_string(),
            value_node: Box::new(SyntaxNode::Literal(Literal::String("hello".to_string())))
        }];
        let result = analyzer.analyze_syntax_nodes(nodes);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Type mismatch"));
    }

    #[test]
    fn test_semantic_analyzer_redeclaration() {
        let mut analyzer = SemanticAnalyzer::new();
        let nodes = vec![SyntaxNode::Declaration { name: "y".to_string(), ty: Type::Int }, SyntaxNode::Declaration { name: "y".to_string(), ty: Type::Int }];
        let result = analyzer.analyze_syntax_nodes(nodes);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already declared"));
    }
}
