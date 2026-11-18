use crate::parser::{Parser, Ast};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_ast() {
        let parser = Parser::new();
        let input = "// Malformed input";
        assert!(parser.parse(input).is_err());
    }

    #[test]
    fn test_validate_ast_valid() {
        let parser = Parser::new();
        let input = "// Valid input";
        assert!(parser.parse(input).is_ok());
    }

    #[test]
    fn test_validate_ast_empty_input() {
        let parser = Parser::new();
        let input = "";
        assert!(parser.parse(input).is_err());
    }

    #[test]
    fn test_validate_ast_single_token() {
        let parser = Parser::new();
        let input = "let";
        assert!(parser.parse(input).is_err());
    }

    #[test]
    fn test_validate_ast_multiple_tokens() {
        let parser = Parser::new();
        let input = "let x = 5;";
        assert!(parser.parse(input).is_ok());
    }
}
