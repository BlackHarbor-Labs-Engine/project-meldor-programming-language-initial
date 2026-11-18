use crate::engine::Engine;
use crate::syntax::SyntaxNode;
use crate::ir::IrBuilder;

/// The Runtime manages the lifecycle of the program execution.
pub struct Runtime {
    pub engine: Engine,
}

impl Runtime {
    pub fn new(malware_engine: crate::malware_engine::MalwareEngine, builder: IrBuilder) -> Self {
        Runtime {
            engine: Engine::new(malware_engine, builder),
        }
    }

    /// Runs the entire program lifecycle: Analyze -> Execute.
    pub fn run(&mut self, ast_root: &SyntaxNode) -> Result<(), String> {
        // 1. Analysis Phase
        self.engine.analyze_and_plan(ast_root)?;
        
        // 2. Execution Phase
        self.engine.execute(ast_root)?;
        
        Ok(())
    }
}
