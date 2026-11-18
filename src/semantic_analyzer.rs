use std::collections::HashMap;

use crate::syntax::{SyntaxNode, Type, Literal, Value, NodeType};

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
            match node.kind {
                NodeType::Declaration { name, ty } => {
                    if self.types.contains_key(&name) {
                        return Err(format!("Semantic Error: Variable '{}' already declared.", name));
                    }
                    self.types.insert(name, ty);
                }
                NodeType::Assignment { name, value_node } => {
                    let expected_type = self.types.get(&name).cloned().unwrap_or(Type::Unknown);
                    
                    // Determine actual type based on the value node's kind
                    let actual_type = match &value_node.kind {
                        NodeType::Literal(Literal { value: Value::String(_) }) => Type::String,
                        NodeType::Literal(Literal { value: Value::Int(_) }) => Type::Int,
                        NodeType::Identifier(id) => self.types.get(&id).cloned().unwrap_or(Type::Unknown),
                        _ => Type::Unknown,
                    };

                    if expected_type != actual_type && expected_type != Type::Unknown {
                        return Err(format!("Semantic Error: Type mismatch for '{}'. Expected {:?}, found {:?}.", name, expected_type, actual_type));
                    }
                }
                NodeType::BinaryOp { left, op, right } => {
                    // In a real system, we'd recursively check types here.
                    // For now, we just ensure operands are not unknown.
                    if matches!(&left.kind, NodeType::UnknownNode) || matches!(&right.kind, NodeType::UnknownNode) {
                        return Err("Semantic Error: Binary operation involves an unknown node.".to_string());
                    }
                }
                _ => { /* Other node types pass basic checks */ }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{SyntaxNode, Type, Literal, Value, NodeType};

    #[test]
    fn test_semantic_analyzer_basic_declaration() {
        let mut analyzer = SemanticAnalyzer::new();
        let nodes = vec![SyntaxNode { kind: NodeType::Declaration { name: "x".to_string(), ty: Type::Int } }];
        assert!(analyzer.analyze_syntax_nodes(nodes).is_ok());
        assert_eq!(analyzer.types.get("x"), Some(&Type::Int));
    }

    #[test]
    fn test_semantic_analyzer_type_mismatch() {
        let mut analyzer = SemanticAnalyzer::new();
        // Declare x as Int, then try to assign a string literal
        analyzer.types.insert("x".to_string(), Type::Int);
        let nodes = vec![SyntaxNode {
            kind: NodeType::Assignment {
                name: "x".to_string(),
                value_node: Box::new(SyntaxNode { kind: NodeType::Literal(Literal { value: Value::String("hello".to_string()) }) })
            }
        }];
        let result = analyzer.analyze_syntax_nodes(nodes);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Type mismatch"));
    }

    #[test]
    fn test_semantic_analyzer_redeclaration() {
        let mut analyzer = SemanticAnalyzer::new();
        let nodes = vec![SyntaxNode { kind: NodeType::Declaration { name: "y".to_string(), ty: Type::Int } }, SyntaxNode { kind: NodeType::Declaration { name: "y".to_string(), ty: Type::Int } }];
        let result = analyzer.analyze_syntax_nodes(nodes);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already declared"));
    }
}
