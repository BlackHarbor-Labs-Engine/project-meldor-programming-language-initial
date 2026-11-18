// A functional implementation of the Analyzer trait

use crate::syntax::{SyntaxNode, Type};

pub struct SemanticAnalyzer;

impl Analyzer for SemanticAnalyzer {
    fn analyze(&self, node: &SyntaxNode) -> Result<(), String> {
        match node {
            SyntaxNode::Declaration { name, ty } => {
                println!("Analyzing declaration: {} with type {:?}", name, ty);
                Ok(())
            }
            SyntaxNode::Assignment { name, value_node } => {
                println!("Analyzing assignment for: {}", name);
                // Recursive analysis would happen here
                Ok(())
            }
            _ => Ok(())
        }
    }
}
