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
}
