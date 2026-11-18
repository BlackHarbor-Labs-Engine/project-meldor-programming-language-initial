use crate::syntax::{SyntaxNode, Span,};

pub fn lexer(input: &str) -> Result<&str, Vec<SyntaxNode>> {
    // Lexer implementation
    let mut syntax_nodes = Vec::new();
    let mut remaining = input;
    while !remaining.is_empty() {
        match keyword(remaining) {
            Ok((rem, node)) => {
                syntax_nodes.push(node);
                remaining = rem;
            }
            Err(_) => break,
        }
    }
    Ok((remaining, syntax_nodes))
}

// Remove unused import
// use crate::lexer::{Token, Lexer};

fn keyword(input: &str) -> Result<(&str, SyntaxNode), SyntaxNode> {
    // Simplified for now - in practice this would be more complex
    if input.starts_with("let") {
        Ok(("let", SyntaxNode::Keyword { value: "let".to_string(), span: Span::new(0, 0, 0, 0) }))
    } else if input.starts_with("fn") {
        Ok(("fn", SyntaxNode::Keyword { value: "fn".to_string(), span: Span::new(0, 0, 0, 0) }))
    } else {
        Err(SyntaxNode::Keyword { value: "unknown".to_string(), span: Span::new(0, 0, 0, 0) })
    }
}

