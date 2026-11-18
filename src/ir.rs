use crate::syntax::{Type, SyntaxNode};

// Define IrInstruction
pub struct IrInstruction {
    pub op: String,
    pub operands: Vec<Type>,
}

// Define IrBuilder
pub struct IrBuilder {
    pub instructions: Vec<IrInstruction>,
}

impl IrBuilder {
    pub fn new() -> Self {
        IrBuilder { instructions: Vec::new() }
    }
}

// ... rest of the IR logic ...
