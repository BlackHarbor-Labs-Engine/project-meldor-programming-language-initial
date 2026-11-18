use crate::syntax::{Type, SyntaxNode}; // Keep this for now, assuming SyntaxNode is defined elsewhere

// Define IrInstruction
pub struct IrInstruction {
    pub op: String,
    pub operands: Vec<Type>,
    pub instrumentation_tag: Option<String>,
}

// Define IrBuilder
pub struct IrBuilder {
    pub instructions: Vec<IrInstruction>,
    pub execution_log: Vec<String>,
}

impl IrBuilder {
    pub fn new() -> Self {
        IrBuilder {
            instructions: Vec::new(),
            execution_log: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, op: String, operands: Vec<Type>, tag: Option<String>) {
        // Ensure instruction addition is atomic and handles potential resource contention simulation
        self.instructions.push(IrInstruction { op, operands, instrumentation_tag: tag });
    }

    pub fn log_event(&mut self, event: &str) {
        // Critical path for stress testing: ensure push is safe under load
        self.execution_log.push(event.to_string());
    }

    pub fn get_log(&self) -> &Vec<String> {
        &self.execution_log
    }
}

// Define the top-level IR structure to satisfy pub use ir::IR;
pub struct IR {
    pub builder: IrBuilder,
}

impl IR {
    pub fn new() -> Self {
        IR { builder: IrBuilder::new() }
    }
}
