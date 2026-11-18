use std::collections::HashMap;

pub struct Scope<'a, 'b> {
    name: &'a str,
    variables: HashMap<&'b str, String>,
}

pub struct SemanticAnalyzer<'a> {
    // Define fields here
}
    scopes: Vec<Scope<'a, 'a>>,
    nodes: HashMap<String, Vec<String>>,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new() -> SemanticAnalyzer<'a> {
        // Initialize fields here
        SemanticAnalyzer { /* fields */ }
    }

    pub fn analyze(&self, tokens: &[&str]) -> Result<(), String> {
        // Implement analyze method here
        Ok(())
    }
}
    pub fn new() -> Self {
        SemanticAnalyzer {
            scopes: Vec::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn push_scope(&mut self, name: &'a str) {
        let scope = Scope {
            name,
            variables: HashMap::new(),
        };
        self.scopes.push(scope);
    }

    pub fn pop_scope(&mut self) -> Option<Scope<'a, 'a>> {
        self.scopes.pop()
    }

    pub fn add_variable(&mut self, name: &'a str, value: String) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.variables.insert(name, value);
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<&String> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.variables.get(name) {
                return Some(value);
            }
        }
        None
    }

    pub fn add_node(&mut self, key: String, value: Vec<String>) {
        self.nodes.insert(key, value);
    }

    pub fn get_nodes(&self, key: &str) -> Option<&Vec<String>> {
        self.nodes.get(key)
    }
}
