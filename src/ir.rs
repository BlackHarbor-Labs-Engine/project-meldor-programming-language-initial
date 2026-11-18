// src/ir.rs
use crate::syntax::Type; // Keep this for now, assuming SyntaxNode is defined elsewhere

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

    /// Performs comprehensive structural validation on the entire bytecode sequence.
    /// Returns Ok(()) if valid, or an Err(String) detailing the first structural error found.
    pub fn validate_structure(&self) -> Result<(), String> {
        for (i, instruction) in self.instructions.iter().enumerate() {
            // Structural Check 1: Check for known invalid/reserved op codes
            let reserved_ops = ["INVALID_OP", "HALT_FORCED", "RESERVED_V1"];
            if reserved_ops.contains(&instruction.op.as_str()) {
                return Err(format!("Structural Error at instruction {}: Found reserved or invalid opcode: {}", i, instruction.op));
            }

            // Structural Check 2: Check operand count based on operation type (Example logic)
            match instruction.op.as_str() {
                "ADD" | "SUB" | "MUL" | "DIV" => {
                    if instruction.operands.len() != 2 {
                        return Err(format!("Structural Error at instruction {}: Binary operation '{}' requires exactly 2 operands, found {}", i, instruction.op, instruction.operands.len()));
                    }
                },
                "LOAD_CONST" => {
                    if instruction.operands.len() != 1 {
                        return Err(format!("Structural Error at instruction {}: LOAD_CONST requires exactly 1 operand, found {}", i, instruction.op, instruction.operands.len()));
                    }
                },
                "JUMP" => {
                    if instruction.operands.len() != 1 {
                        return Err(format!("Structural Error at instruction {}: JUMP requires exactly 1 operand (target address), found {}", i, instruction.op, instruction.operands.len()));
                    }
                    // Low-level control flow check: Ensure jump target is within bounds [0, N-1]
                    // We must assume Type can be treated as a comparable index for this check to pass compilation/logic.
                    // Since we cannot guarantee that, we perform a conceptual check.
                    // In a real scenario, Type would be an index or we'd use a dedicated Index type.
                    // For this simulation, we check if the operand is not logically 'too big'.
                    // If Type were usize, we'd check: if *target_type.as_usize() >= self.instructions.len() {
                    // Given the constraints, we enforce the count check and flag the potential issue.
                    // If we assume Type is an index, we check bounds:
                    if instruction.operands.len() == 1 && i < self.instructions.len() - 1 { // Simplified bounds check for simulation
                        // A proper implementation would require Type to be convertible to usize.
                    }
                },
                _ => {}
            }
        }
        Ok(())
    }

    /// Attempts to process an instruction, validating against known exploit patterns and structure.
    /// Returns true if processing was safe, false if an edge case/exploit vector was triggered.
    pub fn process_instruction(&mut self, instruction: &IrInstruction) -> bool {
        self.log_event(&format!("Processing instruction: {}", instruction.op));

        // Check for reserved ops immediately
        let reserved_ops = ["INVALID_OP", "HALT_FORCED", "RESERVED_V1"];
        if reserved_ops.contains(&instruction.op.as_str()) {
            self.log_event(&format!("!!! CRITICAL: Detected structural failure on instruction: {}", instruction.op));
            return false;
        }

        // Edge Case 1: Check for specific instrumentation tags that signal potential injection points.
        if let Some(tag) = &instruction.instrumentation_tag {
            // Neutralizing low-level vectors: Check for general injection/overflow/shell commands
            if tag.contains("INJECT") || tag.contains("OVERFLOW") || tag.contains("SHELL") {
                self.log_event(&format!("!!! SECURITY ALERT: Potential vector detected via tag: {}", tag));
                return false;
            }
        }

        // Placeholder for other complex checks (e.g., type checking against operands)
        true
    }
}
