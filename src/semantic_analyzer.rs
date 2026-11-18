use crate::syntax::{NodeType, LiteralNode;

use crate::syntax::SyntaxNode;

pub fn analyze_syntax_nodes(nodes: &[SyntaxNode]) -> Result<(), String> {
    for node in nodes {
        if let Err(e) = analyze(node) {
            return Err(e);
        }
    }
    Ok(())
}
    match &node.kind {
        NodeType::BinaryOp { left, right, op } => {
            if matches!(&left.kind, NodeType::UnknownNode) || matches!(&right.kind, NodeType::UnknownNode) {
                return Err("Semantic Error: Binary operation involves an unknown node.".to_string());
            }
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
