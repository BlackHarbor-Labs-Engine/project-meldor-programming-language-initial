use std::collections::HashMap;

pub struct Scope<'a, 'b> {
    name: &'a str,
    variables: HashMap<&'b str, String>,
}

pub struct SemanticAnalyzer<'a> {
    scopes: Vec<Scope<'a, 'a>>,
    nodes: HashMap<String, Vec<String>>,
}

impl<'a> SemanticAnalyzer<'a> {
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

    pub fn analyze(&mut self, nodes: Vec<SyntaxNode>) -> Result<(), String> {
        // Basic semantic analysis: validate syntax nodes and build scope
        for node in nodes.iter() {
            match node {
                SyntaxNode::Keyword(kw) => {
                    // Validate known keywords
                    match kw.as_str() {
                        "let" | "fn" | "if" | "else" | "while" | "for" | "return" => {}
                        _ => return Err(format!("Unknown keyword: {}", kw)),
                    }
                }
                SyntaxNode::Identifier(id) => {
                    // Track identifiers in the current scope
                    self.add_variable(id, id.clone());
                }
                SyntaxNode::Literal { value, .. } => {
                    // Validate literal values (basic check)
                    if value.parse::<i64>().is_err() && value.parse::<f64>().is_err() {
                        return Err(format!("Invalid literal value: {}", value));
                    }
                }
            }
        }
        Ok(())
    }
}
