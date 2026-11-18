use crate::ir::{IR, IrInstruction}; // Assuming IR and IrInstruction are public in ir.rs

/// The InstrumentedRuntime acts as the execution engine, wrapping the IR to enforce type safety and capture execution flow.
pub struct InstrumentedRuntime {
    ir: IR,
}

impl InstrumentedRuntime {
    /// Creates a new runtime instance initialized with the provided IR.
    pub fn new(ir: IR) -> Self {
        InstrumentedRuntime { ir }
    }

    /// Performs comprehensive validation on the IR structure and then simulates execution.
    /// Returns Ok(()) if structural validation passes and execution completes without critical type errors.
    pub fn execute_and_validate(&mut self) -> Result<(), String> {
        // 1. Structural Validation (from IR)
        println!("--- Starting Structural Validation ---");
        if let Err(e) = self.ir.builder.validate_structure() {
            return Err(format!("Structural Validation Failed: {}", e));
        }
        println!("--- Structural Validation Passed ---");

        // 2. Execution Simulation & Type Safety Instrumentation
        println!("--- Starting Execution Instrumentation ---");
        let mut type_safety_ok = true;
        let mut execution_log = Vec::new();

        for instruction in self.ir.builder.instructions.iter_mut() {
            // Process instruction using the IR's built-in safety checks
            let safe = self.ir.builder.process_instruction(instruction);
            execution_log.push(format!("Instruction processed: {}", instruction.op));

            if !safe {
                // If process_instruction returns false, it means a critical tag/exploit vector was found.
                type_safety_ok = false;
                println!("!!! CRITICAL FAILURE DETECTED DURING EXECUTION SIMULATION.");
                break;
            }

            // --- Type Safety Instrumentation Hook ---
            // In a real scenario, this is where we would check the types of operands against the expected type of 'op'.
            // For this implementation, we simulate a type check based on the instruction's presence.
            if instruction.op.starts_with("LOAD") || instruction.op.starts_with("ADD") {
                // Simulate type check: if operands are present, assume they are valid types for this stage.
                // A real implementation would check instruction.operands[i].type_id against expected types.
                if instruction.operands.len() > 0 && instruction.operands.iter().any(|t| t.is_untyped()) { // Assuming Type has an is_untyped() method
                    type_safety_ok = false;
                    execution_log.push(format!("TYPE ERROR: Untyped operand found in operation {}", instruction.op));
                    break;
                }
            }
            // --------------------------------------
        }

        if !type_safety_ok {
            return Err("Type Safety Validation Failed during execution simulation.".to_string());
        }

        println!("--- Execution Simulation Complete ---");
        Ok(())
    }

    /// Retrieves the full execution log for external analysis.
    pub fn get_execution_log(&self) -> &Vec<String> {
        self.ir.builder.get_log()
    }
}
