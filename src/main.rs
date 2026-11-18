use meldor::{lexer, parser, runtime, malware_engine, ir};
use meldor::syntax::SyntaxNode;

fn main() {
    let input = "let x = 5";
    
    // 1. Lexing
    println!("=== Lexing ===");
    let lexer = lexer::Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    println!("Tokens: {:?}", tokens);

    // 2. Parsing
    println!("
=== Parsing ===");
    let parser = parser::Parser::new();
    let syntax_tree = parser.parse(&tokens).unwrap();
    println!("Syntax Tree: {:?}", syntax_tree);

    // 3. Engine Setup & Analysis
    println!("
=== Engine Setup & Analysis ===");
    let malware_engine = malware_engine::MalwareEngine::new();
    let builder = ir::IrBuilder::new();
    
    let mut engine = engine::Engine::new(malware_engine, builder);
    
    // Perform semantic analysis and risk check
    match engine.analyze_and_plan(&syntax_tree) {
        Ok(_) => println!("Engine: Analysis and planning successful."),
        Err(e) => { 
            eprintln!("Engine Error during analysis: {}", e);
            return;
        }
    }

    // 4. Execution
    println!("
=== Execution ===");
    match engine.execute(&syntax_tree) {
        Ok(_) => println!("Execution finished successfully."),
        Err(e) => { 
            eprintln!("Engine Error during execution: {}", e);
            return;
        }
    }
}
