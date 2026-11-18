/// Deprecated: Use Runtime struct from src/runtime.rs for execution.
pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn interpret(&mut self, ast_root: &crate::syntax::SyntaxNode) -> Result<(), String> {
        Err("Interpreter has been replaced by the scalable Runtime engine. Use crate::runtime::Runtime instead.".to_string())
    }
}
