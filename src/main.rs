use meldor::{lexer, parser, runtime, malware_engine, ir};
use meldor::syntax::SyntaxNode;

fn main() {
    let input = "let x = 5";
    
    // 1. Lexing: Assuming Lexer::new(input) and tokenize() returns Result<Vec<Token>>
    let lexer = lexer::Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    println!("Tokens: {:?}", tokens);
    
    // 2. Parsing: Assuming Parser::new() and parse(tokens) returns Result<SyntaxNode>
    let parser = parser::Parser::new();
    let syntax_tree = parser.parse(&tokens).unwrap();
    println!("Syntax Tree: {:?}", syntax_tree);
    
    // 3. Semantic Analysis
    let malware_engine = malware_engine::MalwareEngine::new();
    // Create a dummy vector of SyntaxNode to satisfy the API
    let nodes = vec![];
    // Call the function directly, as it's now exported from semantic_analyzer
    let analysis_result = semantic_analyzer::analyze(&nodes);
    println!("Analysis Result: {:?}", analysis_result);
    
    // 4. IR Generation and Runtime
    let builder = ir::IrBuilder::new();
    let mut runtime = runtime::Runtime::new(malware_engine, builder);
    let runtime_result = runtime.run(&syntax_tree);
    println!("IR Result: {:?}", runtime_result);
}
