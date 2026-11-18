use std::collections::HashMap;
use crate::syntax::{SyntaxNode, Span};

#[derive(Debug, Clone)]
pub struct Type {
    pub name: String,
    pub fields: Vec<(String, Type)>, // For structs
    pub is_recursive: bool,
}

impl Type {
    pub fn new(name: &str) -> Self {
        Type {
            name: name.to_string(),
            fields: Vec::new(),
            is_recursive: false,
        }
    }

    pub fn with_fields(mut self, fields: Vec<(String, Type)>) -> Self {
        self.fields = fields;
        self
    }

    pub fn set_recursive(&mut self, recursive: bool) {
        self.is_recursive = recursive;
    }
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub variables: HashMap<String, Type>,
    pub parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            variables: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Scope) -> Self {
        Scope {
            variables: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Type> {
        if let Some(ty) = self.variables.get(name) {
            Some(ty)
        } else if let Some(ref parent) = self.parent {
            parent.get(name)
        } else {
            None
        }
    }

    pub fn insert(&mut self, name: String, ty: Type) {
        self.variables.insert(name, ty);
    }
}

pub struct SemanticAnalyzer {
    scopes: Vec<Scope>,
    types: HashMap<String, Type>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            scopes: vec![Scope::new()],
            types: HashMap::new(),
        }
    }

    pub fn enter_scope(&mut self) {
        let parent = self.scopes.pop().unwrap();
        self.scopes.push(Scope::with_parent(parent));
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn current_scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    pub fn resolve_type(&self, name: &str) -> Option<&Type> {
        if let Some(ty) = self.types.get(name) {
            Some(ty)
        } else {
            None
        }
    }

    pub fn add_type(&mut self, name: String, ty: Type) {
        self.types.insert(name, ty);
    }

    pub fn analyze(&mut self, nodes: Vec<SyntaxNode>) -> Result<(), String> {
        // First pass: collect all type definitions and declarations
        self.first_pass(&nodes)?;
        
        // Second pass: resolve types and check for recursive definitions
        self.second_pass(&nodes)?;
        
        Ok(())
    }

    fn first_pass(&mut self, nodes: &Vec<SyntaxNode>) -> Result<(), String> {
        let mut graph = DependencyGraph::new();
        for node in nodes {
            match node {
                SyntaxNode::Keyword { value, .. } if value == "fn" => {
                    // Function declaration
                    self.enter_scope();
                }
                SyntaxNode::Keyword { value, .. } if value == "let" => {
                    // Variable declaration
                    // We'll handle this in second pass for now
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn second_pass(&mut self, nodes: &Vec<SyntaxNode>) -> Result<(), String> {
        for node in nodes {
            match node {
                SyntaxNode::Keyword { value, span } if value == "let" => {
                    // This is a simplified version - in practice you'd parse the assignment
                    // and check type compatibility here
                    println!("Analyzing let statement at {:?}", span);
                }
                _ => {}
            }
        }
        Ok(())
    }

    struct DependencyGraph {
        nodes: HashMap<String, Vec<String>>,
    }

    impl DependencyGraph {
        fn new() -> Self {
            DependencyGraph { nodes: HashMap::new() }
        }

        fn add_node(&mut self, node: String) {
            self.nodes.entry(node).or_insert_with(Vec::new);
        }

        fn add_edge(&mut self, from: String, to: String) {
            self.nodes.entry(from).or_insert_with(Vec::new).push(to);
        }

        fn get_nodes(&self) -> &HashMap<String, Vec<String>> {
            &self.nodes
        }
    }
}
