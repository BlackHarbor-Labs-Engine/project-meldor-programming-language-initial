use crate::semantic_analyzer::{SemanticAnalyzer, Type};
use crate::syntax::{SyntaxNode, Span};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_checking() {
        let mut analyzer = SemanticAnalyzer::new();
        analyzer.types.insert("x".to_string(), Type::Int);

        let let_node = SyntaxNode {
            value: "let".to_string(),
            span: Span { start: 0, end: 3 },
        };

        let nodes = vec![let_node];

        let result = analyzer.analyze(nodes);

        assert!(result.is_ok(), "Type checking should pass");
    }

    #[test]
    fn test_type_checking_invalid_type() {
        let mut analyzer = SemanticAnalyzer::new();
        analyzer.types.insert("x".to_string(), Type::Int);

        let let_node = SyntaxNode {
            value: "let".to_string(),
            span: Span { start: 0, end: 3 },
        };

        let nodes = vec![let_node];

        let result = analyzer.analyze(nodes);

        assert!(result.is_err(), "Type checking should fail");
    }

    #[test]
    fn test_type_checking_empty_input() {
        let mut analyzer = SemanticAnalyzer::new();

        let nodes = vec![];

        let result = analyzer.analyze(nodes);

        assert!(result.is_err(), "Type checking should fail");
    }

    #[test]
    fn test_type_checking_single_node() {
        let mut analyzer = SemanticAnalyzer::new();

        let let_node = SyntaxNode {
            value: "let".to_string(),
            span: Span { start: 0, end: 3 },
        };

        let nodes = vec![let_node];

        let result = analyzer.analyze(nodes);

        assert!(result.is_err(), "Type checking should fail");
    }
}
