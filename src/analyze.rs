use crate::syntax::{NodeType, SyntaxNode};

pub fn analyze(node: &SyntaxNode) -> Result<(), String> {
    match &node.kind {
        NodeType::BinaryOp { left, right, op: _ } => {
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
