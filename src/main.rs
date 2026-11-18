use meldor::{lexer, parser, semantic_analyzer, ir};
// use meldor::syntax::SyntaxNode;

fn main() {
    let input = "let x = 5";
    let tokens = lexer::lexer(input).unwrap();
    println!("Tokens: {:?}", tokens);

    // Parse the tokens into a syntax tree
    let syntax_tree = parser::parse_statement(input).unwrap().1;
    println!("Syntax Tree: {:?}", syntax_tree);

    let mut analyzer = semantic_analyzer::SemanticAnalyzer::new();
    // Create a dummy vector of SyntaxNode to satisfy the API
    let nodes = vec![];
    let result = analyzer.analyze_syntax_nodes(nodes);
    println!("Analysis Result: {:?}", result);

    let mut builder = ir::IrBuilder::new();
    // In a real implementation, we'd use the actual syntax tree here
    let ir_result = builder.from_syntax(syntax_tree);
    println!("IR Result: {:?}", ir_result);
}
