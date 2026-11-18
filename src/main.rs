use meldor::{lexer, parser, semantic_analyzer, ir};
use meldor::syntax::SyntaxNode;

fn main() {
    let input = "let x = 5";
    let tokens = lexer::lexer(input).unwrap();
    println!("Tokens: {:?}", tokens);

    let syntax_tree = parser::parse(tokens).unwrap();
    println!("Syntax Tree: {:?}", syntax_tree);

    let mut analyzer = semantic_analyzer::SemanticAnalyzer::new();
    let result = analyzer.analyze_syntax_nodes(syntax_tree);
    println!("Analysis Result: {:?}", result);

    let mut builder = ir::IrBuilder::new();
    let ir_result = builder.from_syntax(syntax_tree);
    println!("IR Result: {:?}", ir_result);
}
