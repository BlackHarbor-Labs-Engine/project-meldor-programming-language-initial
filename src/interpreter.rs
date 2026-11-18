// This file defines the BytecodeInterpreter for executing IR instructions.

use crate::ir::{IrInstruction, IrValue};

struct BytecodeInterpreter {
    stack: Vec<IrValue>,
}

impl BytecodeInterpreter {
    fn new() -> Self {
        BytecodeInterpreter { stack: Vec::new() }
    }

    fn execute(&mut self, instruction: IrInstruction) {
        match instruction {
            IrInstruction::Push(value) => self.stack.push(value),
            IrInstruction::Pop => self.stack.pop(),
            IrInstruction::Add => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a + b);
            }
            IrInstruction::Sub => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a - b);
            }
            IrInstruction::Mul => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a * b);
            }
            IrInstruction::Div => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a / b);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter() {
        let mut interpreter = BytecodeInterpreter::new();
        interpreter.execute(IrInstruction::Push(10));
        interpreter.execute(IrInstruction::Push(5));
        interpreter.execute(IrInstruction::Add);
        assert_eq!(interpreter.stack.pop().unwrap(), 15);
    }
}
