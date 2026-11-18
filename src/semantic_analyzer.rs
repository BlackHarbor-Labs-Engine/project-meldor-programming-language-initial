use crate::syntax::{NodeType, SyntaxNode};

/// Analyzes a slice of SyntaxNodes for semantic correctness.
pub fn analyze(nodes: &[SyntaxNode]) -> Result<(), String> {
    for node in nodes {
        if let Err(e) = analyze_node(node) {
            return Err(e);
        }
    }
    Ok(())
}

/// Wrapper function to satisfy external API expectations.
pub fn analyze(nodes: &[SyntaxNode]) -> Result<(), String> {
    analyze_syntax_nodes(nodes)
}

/// Helper function to analyze a single node.
fn analyze_node(node: &SyntaxNode) -> Result<(), String> {
    match &node.kind {
        NodeType::BinaryOp { left, right, op: _ } => {
            if matches!(&left.kind, NodeType::UnknownNode) || matches!(&right.kind, NodeType::UnknownNode) {
                return Err("Semantic Error: Binary operation involves an unknown node.".to_string());
            }
            // Placeholder for actual binary op validation
            Ok(())
        }
        _ => Ok(())
    }
}
