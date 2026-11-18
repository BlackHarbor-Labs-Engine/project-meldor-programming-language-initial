use crate::syntax::{NodeType, SyntaxNode};

/// Analyzes a slice of SyntaxNodes for semantic correctness.
pub fn analyze_syntax_nodes(nodes: &[SyntaxNode]) -> Result<(), String> {
    for node in nodes {
        if let Err(e) = analyze_node(node) {
            return Err(e);
        }
    }
    Ok(())
}

/// Helper function to analyze a single node.
fn analyze_node(node: &SyntaxNode) -> Result<(), String> {
    match &node.kind {
        NodeType::BinaryOp { left, right, op: _ } => {
            if matches!(&left.kind, NodeType::UnknownNode) || matches!(&right.kind, NodeType::UnknownNode) {
                return Err("Semantic Error: Binary operation involves an unknown node.".to_string());
            }
            if !left.get_type().map_or(false, |t| t.is_compatible_with(right.get_type().unwrap_or(&Type::Unknown))) { return Err(format!("Semantic Error: Type mismatch for binary op '{}'. Left type incompatible with right type.".to_string())); }
            Ok(())
        }
        NodeType::Declaration { name, ty } => {
            println!("Analyzing Declaration: {} with type {:?}", name, ty);
            Ok(())
        }
        NodeType::Assignment { name: _, value_node: _ } => {
            Ok(())
        }
        NodeType::Literal(lit_node) => {
            println!("Analyzing Literal: {:?}", lit_node);
            Ok(())
        }
        NodeType::Identifier(id) => {
            println!("Analyzing Identifier: {}", id);
            Ok(())
        }
        NodeType::UnknownNode => {
            Err("Semantic Error: Encountered an unknown node type during analysis.".to_string())
        }
    }
}
