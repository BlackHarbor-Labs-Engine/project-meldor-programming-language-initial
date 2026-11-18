mod lexer;
mod syntax;
mod parser;
mod semantic_analyzer;

use syntax::{SyntaxNode, Span};
use semantic_analyzer::SemanticAnalyzer;

fn main() {
    let mut analyzer = SemanticAnalyzer::new();
    
    // Test with a simple function definition
    let nodes = vec![
        SyntaxNode::Keyword("fn".to_string()),
        SyntaxNode::Identifier("test".to_string()),
        SyntaxNode::Literal { 
            value: "42".to_string(), 
            span: Span::new(1, 5, 3, 5) 
        },
    ];
    
    match analyzer.analyze(&nodes) {
        Ok(_) => println!("Semantic analysis passed successfully."),
        Err(e) => eprintln!("Semantic analysis failed: {}", e),
    }
    
    // Test lexer
    let input = "let x = 5";
    match lexer::lexer(input) {
        Ok(tokens) => println!("Lexer produced {} tokens", tokens.len()),
        Err(e) => eprintln!("Lexer error: {}", e),
    }
    
    // Test parser
    match parser::parse_expression("123") {
        Ok((remaining, expr)) => println!("Parsed expression '{}', remaining: '{}'", expr, remaining),
        Err(e) => eprintln!("Parse error: {:?}", e),
    }
    
    println!("Hello, Meldor!");
}
