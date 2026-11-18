use meldor::{lexer, parser, runtime, malware_engine, ir};
use meldor::syntax::SyntaxNode;
// use meldor::syntax::SyntaxNode;

fn main() {
    let input = "let x = 5";
    let tokens = lexer::lexer(input).unwrap();
    println!("Tokens: {:?}", tokens);

    // Parse the tokens into a syntax tree
    let syntax_tree = parser::parse_statement(input).unwrap();
    println!("Syntax Tree: {:?}", syntax_tree);

    let malware_engine = malware_engine::MalwareEngine::new();
    // Create a dummy vector of SyntaxNode to satisfy the API
    let nodes = vec![];
    let result = analyzer.analyze_syntax_nodes(nodes);
    println!("Analysis Result: {:?}", result);

    let builder = ir::IrBuilder::new();
    // In a real implementation, we'd use the actual syntax tree here
    let mut runtime = runtime::Runtime::new(malware_engine, builder);
let runtime_result = runtime.run(&syntax_tree);
    println!("IR Result: {:?}", ir_result);
}
