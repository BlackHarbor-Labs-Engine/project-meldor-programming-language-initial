use crate::state_machine::StateMachine;

pub fn parser(input: &str) -> IResult<&str, Vec<SyntaxNode>> {
    // Parser implementation
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
