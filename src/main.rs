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
    println!("\n=== Parsing ===");
    let parser = parser::Parser::new();
    let syntax_tree = parser.parse(&tokens).unwrap();
    println!("Syntax Tree: {:?}");

    // 3. Runtime Orchestration & Execution
    let malware_engine = malware_engine::MalwareEngine::new();
    let builder = ir::IrBuilder::new();

    // Initialize the full runtime environment
    let mut runtime = runtime::Runtime::new(malware_engine, builder);

    // Run the entire lifecycle: Analyze -> Execute, controlled by StateMachine
    println!("\n=== Running Meldor Lifecycle ===");
    match runtime.run(&syntax_tree) {
        Ok(_) => println!("Lifecycle completed successfully."),
        Err(e) => {
            eprintln!("Runtime Error during lifecycle: {}");
            return;
        }
    }
}
