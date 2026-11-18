use crate::engine::Engine;
use crate::syntax::SyntaxNode;
use crate::ir::IrBuilder;
use crate::state_machine::{StateMachine, State};

/// Minimal Taint Tracker implementation for demonstration of the hook.
pub struct TaintTracker {
    // In a real scenario, this would hold flow graphs, taint sets, etc.
    is_tainted: bool,
}

impl TaintTracker {
    pub fn new() -> Self {
        TaintTracker { is_tainted: false }
    }

    pub fn reset(&mut self) {
        self.is_tainted = false;
        // Reset all internal state
    }

    pub fn mark_source(&mut self, node: &SyntaxNode) {
        // Logic to mark data originating from an untrusted source
        println!("TaintTracker: Marking source node: {:?}", node);
        self.is_tainted = true;
    }

    pub fn propagate(&mut self, node: &SyntaxNode) {
        // Logic to track how taint flows through operations
        if self.is_tainted {
            println!("TaintTracker: Propagating taint through node: {:?}", node);
        }
    }
}

/// A trait defining the hardened interface for system operations.
/// This abstracts complex calls, ensuring they are only invoked in stable states.
pub trait SystemInterface {
    /// Executes the analysis phase, ensuring the system is in a processing state.
    fn analyze(&mut self, ast_root: &SyntaxNode) -> Result<(), String>;

    /// Executes the core logic, ensuring the system is in a processing state.
    fn execute(&mut self, ast_root: &SyntaxNode) -> Result<(), String>;
}

/// The Runtime manages the lifecycle of the program execution.
pub struct Runtime {
    pub engine: Engine,
    pub state_machine: StateMachine,
    pub taint_tracker: TaintTracker,
}

impl Runtime {
    pub fn new(malware_engine: crate::malware_engine::MalwareEngine, builder: IrBuilder) -> Self {
        Runtime {
            engine: Engine::new(malware_engine, builder),
            state_machine: StateMachine::new(),
            taint_tracker: TaintTracker::new(),
        }
    }

    /// Runs the entire program lifecycle: Analyze -> Execute, controlled by the state machine.
    pub fn run(&mut self, ast_root: &SyntaxNode) -> Result<(), String> {
        // Initialize taint tracking context for this run
        self.taint_tracker.reset();
        // 1. Transition to Processing state
        self.state_machine.transition(State::Processing)?;

        // 2. Hardened Analysis Phase
        self.execute_system_call_analysis(ast_root)?;

        // 3. Hardened Execution Phase
        self.execute_system_call_execution(ast_root)?;

        // 4. Transition back to Idle state upon successful completion
        self.state_machine.transition(State::Idle)?;

        Ok(()) 
    }

    /// Internal method to handle the analysis step via the hardened interface.
    fn execute_system_call_analysis(&mut self, ast_root: &SyntaxNode) -> Result<(), String> {
        // Hook: Mark AST root as initial source of potential taint
        self.taint_tracker.mark_source(ast_root);
        // Check state before calling the system call abstraction
        if self.state_machine.current_state != State::Processing { 
            return Err(format!("System stability failure: Analysis called when state is {:?}", self.state_machine.current_state));
        }

        // Delegate to the engine via the interface abstraction
        self.engine.analyze_and_plan(ast_root).map_err(|e| { 
            // If analysis fails, transition to Error state
            let _ = self.state_machine.transition(State::Error);
            format!("Analysis failed: {}", e)
        })
    }

    /// Internal method to handle the execution step via the hardened interface.
    fn execute_system_call_execution(&mut self, ast_root: &SyntaxNode) -> Result<(), String> {
        // Hook: Propagate taint context during execution
        self.taint_tracker.propagate(ast_root);
        // Check state before calling the system call abstraction
        if self.state_machine.current_state != State::Processing { 
            return Err(format!("System stability failure: Execution called when state is {:?}", self.state_machine.current_state));
        }

        // Delegate to the engine via the interface abstraction
        self.engine.execute(ast_root).map_err(|e| { 
            // If execution fails, transition to Error state
            let _ = self.state_machine.transition(State::Error);
            format!("Execution failed: {}", e)
        })
    }
}

// Implementation of the SystemInterface trait for Runtime
impl SystemInterface for Runtime {
    fn analyze(&mut self, ast_root: &SyntaxNode) -> Result<(), String> {
        self.execute_system_call_analysis(ast_root)
    }

    fn execute(&mut self, ast_root: &SyntaxNode) -> Result<(), String> {
        self.execute_system_call_execution(ast_root)
    }
}
