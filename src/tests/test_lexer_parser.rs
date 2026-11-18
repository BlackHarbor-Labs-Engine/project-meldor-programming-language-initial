// Test file for lexer and parser functions

use super::{lexer, literal, keyword, SyntaxNode, Span};

#[test]
fn test_lexer() {
    let input = "let x = 5; fn y() -> 10; return;";
    let result = lexer(input);
    assert!(result.is_ok());
}

#[test]
fn test_parser() {
    let input = "let x = 5;";
    let result = literal(input);
    assert!(result.is_ok());
}

#[test]
fn test_parser_empty_input() {
    let input = "";
    let result = literal(input);
    assert!(result.is_err());
}

#[test]
fn test_parser_single_token() {
    let input = "let";
    let result = literal(input);
    assert!(result.is_err());
}

#[test]
fn test_parser_multiple_tokens() {
    let input = "let x = 5;";
    let result = literal(input);
    assert!(result.is_ok());
}

#[test]
fn test_parser_expression() {
    // Test simple arithmetic expression: 1 + 2 * 3
    let input = "1 + 2 * 3";
    let result = literal(input);
    assert!(result.is_ok());
}
