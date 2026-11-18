use crate::engine::Engine;
use crate::syntax::SyntaxNode;
use crate::ir::IrBuilder;
use crate::state_machine::{StateMachine, State};

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
}

impl Runtime {
    pub fn new(malware_engine: crate::malware_engine::MalwareEngine, builder: IrBuilder) -> Self {
        Runtime {
            engine: Engine::new(malware_engine, builder),
            state_machine: StateMachine::new(),
        }
    }

    /// Runs the entire program lifecycle: Analyze -> Execute, controlled by the state machine.
    pub fn run(&mut self, ast_root: &SyntaxNode) -> Result<(), String> {
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
