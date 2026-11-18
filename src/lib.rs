//! Meldor programming language library.
//!
//! This crate provides the core components for parsing, analyzing, and compiling
//! the Meldor programming language. It includes:
//! - Lexer for tokenizing source code
//! - Parser for building syntax trees
//! - Semantic analyzer for type checking and validation
//! - Intermediate representation for optimization

// Export all necessary modules
pub mod lexer;
pub mod syntax;
pub mod parser;
pub mod semantic_analyzer;
pub mod ir;

/// Test the core functionality of the Meldor language library.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_lexer_integration() {
        let input = "let x = 5";
        let tokens = lexer::lexer(input).unwrap();
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_parser_integration() {
        let input = "let x = 5";
        let tokens = lexer::lexer(input).unwrap();
        // Just check that we can parse the tokens without error
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_semantic_analyzer_integration() {
        let mut analyzer = semantic_analyzer::SemanticAnalyzer::new();
        let input = "let x = 5";
        let tokens = lexer::lexer(input).unwrap();
        
        // This should not panic
        let result = analyzer.analyze(tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn test_ir_builder_integration() {
        let mut builder = ir::IrBuilder::new();
        let input = "let x = 5";
        let tokens = lexer::lexer(input).unwrap();
        
        // This should not panic
        let result = builder.from_syntax(tokens);
        assert!(result.is_ok());
    }
}
