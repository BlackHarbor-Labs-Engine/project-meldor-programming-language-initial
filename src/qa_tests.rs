<file path="src/qa_tests.rs">
use crate::parser::{Parser, Ast};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_ast_invalid_syntax() {
        let parser = Parser::new();
        let input = "let x = ;"; // Missing value after '='
        assert!(parser.parse(input).is_err());
    }

    #[test]
    fn test_validate_ast_mismatched_braces() {
        let parser = Parser::new();
        let input = "{ let x = 5 }"; // Missing closing brace for a block
        assert!(parser.parse(input).is_err());
    }

    #[test]
    fn test_validate_ast_unknown_keyword() {
        let parser = Parser::new();
        let input = "unknown x = 5;"; // Use of an unknown keyword
        assert!(parser.parse(input).is_err());
    }

    #[test]
    fn test_validate_ast_empty_input() {
        let parser = Parser::new();
        let input = "";
        assert!(parser.parse(input).is_err()); // Expecting error on empty input
    }

    #[test]
    fn test_validate_ast_complex_valid_structure() {
        let parser = Parser::new();
        let input = "let x = 5; if (x > 0) { x = x - 1; }";
        assert!(parser.parse(input).is_ok()); // Complex valid structure
    }

    #[test]
    fn test_validate_ast_valid_simple() {
        let parser = Parser::new();
        let input = "let x = 5;";
        assert!(parser.parse(input).is_ok());
    }
}
</file>
