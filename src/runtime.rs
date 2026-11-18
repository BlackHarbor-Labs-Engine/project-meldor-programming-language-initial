use crate::engine::Engine;
use crate::syntax::SyntaxNode;
use crate::ir::IrBuilder;
use crate::state_machine::{StateMachine, State};

/// Enhanced Taint Tracker to log execution paths for deep data flow tracing.
pub struct TaintTracker {
    is_tainted: bool,
    trace_log: Vec<String>,
}

impl TaintTracker {
    pub fn new() -> Self {
        TaintTracker {
            is_tainted: false,
            trace_log: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.is_tainted = false;
        self.trace_log.clear();
        println!("TaintTracker: Context reset for new run.");
    }

    pub fn mark_source(&mut self, node: &SyntaxNode) {
        self.is_tainted = true;
        let node_repr = format!("SourceNode({:?})", node);
        self.trace_log.push(format!("[TRACE_START] Taint source marked: {}", node_repr));
        println!("TaintTracker: Marking source node: {}", node_repr);
    }

    /// Propagates taint and logs the current execution context/node for tracing.
    pub fn propagate(&mut self, node: &SyntaxNode, context: &str) {
        if self.is_tainted {
            let trace_entry = format!("[TRACE_STEP] Context: {}, Node: {:?}", context, node);
            self.trace_log.push(trace_entry);
            println!("TaintTracker: Propagating taint. Logged step: {}", context);
        } else {
            // Log when taint is absent, useful for path validation
            let trace_entry = format!("[TRACE_PATH] Context: {}, Node: {:?} (Untainted)", context, node);
            self.trace_log.push(trace_entry);
        }
    }

    pub fn get_trace_log(&self) -> &Vec<String> {
        &self.trace_log
    }
}

/// Trait defining the pluggable execution strategy (e.g., Engine, MockEngine, etc.).
pub trait ExecutionStrategy {
    /// Executes the analysis phase, ensuring the system is in a processing state.
    fn analyze(&mut self, ast_root: &SyntaxNode, tracker: &mut TaintTracker) -> Result<(), String>;

    /// Executes the core logic, ensuring the system is in a processing state.
    fn execute(&mut self, ast_root: &SyntaxNode, tracker: &mut TaintTracker) -> Result<(), String>;
}

/// The Runtime manages the lifecycle of the program execution.
pub struct Runtime {
    pub strategy: Box<dyn ExecutionStrategy>, // Pluggable component
    pub state_machine: StateMachine,
    pub taint_tracker: TaintTracker,
}

impl Runtime {
    /// Constructor now accepts any type implementing ExecutionStrategy.
    pub fn new<S: ExecutionStrategy + 'static>(strategy: S, builder: IrBuilder) -> Self {
        Runtime {
            strategy: Box::new(strategy),
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
        // Boundary Check: Ensure state is Processing before proceeding
        if self.state_machine.current_state != State::Processing {
            return Err(format!("Runtime Boundary Violation: Analysis called when state is {:?}. Immediate failure.", self.state_machine.current_state));
        }
        
        // Hook: Mark AST root as initial source of potential taint
        self.taint_tracker.mark_source(ast_root);
        
        // Delegate to the pluggable strategy
        self.strategy.analyze(ast_root, &mut self.taint_tracker).map_err(|e| {
            // If analysis fails, transition to Error state
            let _ = self.state_machine.transition(State::Error);
            format!("Analysis failed: {}", e)
        })
    }

    /// Internal method to handle the execution step via the hardened interface.
    fn execute_system_call_execution(&mut self, ast_root: &SyntaxNode) -> Result<(), String> {
        // Boundary Check: Ensure state is Processing before proceeding
        if self.state_machine.current_state != State::Processing {
            return Err(format!("Runtime Boundary Violation: Execution called when state is {:?}. Immediate failure.", self.state_machine.current_state));
        }
        
        // Hook: Propagate taint context during execution
        // We pass a context string to simulate deeper tracing hooks inside the engine
        self.taint_tracker.propagate(ast_root, "ExecutionStart");
        
        // Delegate to the pluggable strategy
        self.strategy.execute(ast_root, &mut self.taint_tracker).map_err(|e| {
            // If execution fails, transition to Error state
            let _ = self.state_machine.transition(State::Error);
            format!("Execution failed: {}", e)
        })
    }
}

// --- Adapter for existing Engine implementation ---
// To maintain backward compatibility for now, we create an adapter that wraps Engine.
// This allows us to keep the existing Engine logic while satisfying the new trait.
pub struct EngineAdapter { 
    pub engine: Engine,
    pub builder: IrBuilder,
}

impl ExecutionStrategy for EngineAdapter {
    fn analyze(&mut self, ast_root: &SyntaxNode, tracker: &mut TaintTracker) -> Result<(), String> {
        // Delegate to the existing Engine's analysis method, passing the tracker
        self.engine.analyze_and_plan(ast_root).map_err(|e| format!("Engine Analysis Error: {}", e))
    }

    fn execute(&mut self, ast_root: &SyntaxNode, tracker: &mut TaintTracker) -> Result<(), String> {
        // Delegate to the existing Engine's execution method, passing the tracker
        self.engine.execute(ast_root).map_err(|e| format!("Engine Execution Error: {}", e))
    }
}

// The old SystemInterface trait is now obsolete as Runtime directly uses ExecutionStrategy
// We remove the old impl SystemInterface for Runtime to enforce the new design.
