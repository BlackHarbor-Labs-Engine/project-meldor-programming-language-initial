use std::collections::HashMap;
use crate::syntax::{SyntaxNode, Span};

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone)]
pub struct DependencyGraph {
    nodes: HashMap<String, Vec<String>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        DependencyGraph { nodes: HashMap::new() }
    }

    // pub fn add_node(&mut self, node: String) {
        self.nodes.entry(node).or_insert_with(Vec::new);
    }

    // pub fn add_edge(&mut self, from: String, to: String) {
        self.nodes.entry(from).or_insert_with(Vec::new).push(to);
    }

    // pub fn get_nodes(&self) -> &HashMap<String, Vec<String>> {
        &self.nodes
    }
}

// pub struct SemanticAnalyzer {
    scopes: Vec<Scope>,
    // types: HashMap<String, Type>,
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

    // pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    // pub fn current_scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    // pub fn resolve_type(&self, name: &str) -> Option<&Type> {
        if let Some(ty) = self.types.get(name) {
            Some(ty)
        } else {
            None
        }
    }

    // pub fn add_type(&mut self, name: String, ty: Type) {
        self.types.insert(name, ty);
    }

    pub fn analyze(&mut self, nodes: Vec<SyntaxNode>) -> Result<(), String> {
        // First pass: collect all type definitions and declarations
        self.first_pass(&nodes)?;
        
        // Second pass: resolve types and check for semantic errors
        self.second_pass(&nodes)?;
        
        Ok(())
    }

    fn first_pass(&mut self, nodes: &Vec<SyntaxNode>) -> Result<(), String> {
        // In a real implementation, this would collect all type definitions
        // For now, we'll just add some basic types
        self.types.insert("i32".to_string(), Type::new("i32"));
        self.types.insert("bool".to_string(), Type::new("bool"));
        self.types.insert("string".to_string(), Type::new("string"));
        
        Ok(())
    }

    fn second_pass(&mut self, nodes: &Vec<SyntaxNode>) -> Result<(), String> {
        let mut i = 0;
        while i < nodes.len() {
            match &nodes[i] {
                SyntaxNode::Keyword(value) if value == "let" => {
                    // Handle variable declaration
                    if i + 3 < nodes.len() {
                        // Expected pattern: let IDENTIFIER = EXPRESSION
                        if let SyntaxNode::Identifier(name) = &nodes[i + 1] {
                            // Check if we have an assignment operator and expression
                            if let SyntaxNode::Keyword(assignment_op) = &nodes[i + 2] {
                                if assignment_op == "=" {
                                    // For now, we'll just validate the structure
                                    // In a real implementation, we'd evaluate the expression type
                                    // and check compatibility with declared type
                                    println!("Processing variable declaration: {}", name);
                                }
                            }
                        }
                    }
                }
                SyntaxNode::Keyword(value) if value == "fn" => {
                    // Handle function declaration
                    self.enter_scope();
                    // Function parameters would be processed here in a real implementation
                }
                SyntaxNode::Keyword(value) if value == "end" => {
                    // End of scope
                    self.exit_scope();
                }
                _ => {}
            }
            i += 1;
        }
        
        Ok(())
    }
    
    /// Check if two types are compatible for assignment
    pub fn is_type_compatible(&self, source: &Type, target: &Type) -> bool {
        // For now, we'll just check if they're the same type
        // In a real implementation, this would be more sophisticated
        source.name == target.name
    }
    
    /// Resolve a literal value to its type
    pub fn resolve_literal_type(&self, literal_value: &str) -> Type {
        // Simple type resolution for literals
        if literal_value.parse::<i32>().is_ok() {
            Type::new("i32")
        } else if literal_value == "true" || literal_value == "false" {
            Type::new("bool")
        } else {
            Type::new("string") // Default to string for unknown literals
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_graph_new() {
        let graph = DependencyGraph::new();
        assert!(graph.get_nodes().is_empty());
    }

    #[test]
    fn test_dependency_graph_add_node() {
        let mut graph = DependencyGraph::new();
        graph.add_node("A".to_string());
        assert!(graph.get_nodes().contains_key("A"));
    }

    #[test]
    fn test_dependency_graph_add_edge() {
        let mut graph = DependencyGraph::new();
        graph.add_node("A".to_string());
        graph.add_edge("A".to_string(), "B".to_string());
        assert_eq!(graph.get_nodes().get("A"), Some(&vec!["B".to_string()]));
    }

    #[test]
    fn test_scope_new() {
        let scope = Scope::new();
        assert!(scope.variables.is_empty());
        assert!(scope.parent.is_none());
    }

    #[test]
    fn test_scope_with_parent() {
        let parent = Scope::new();
        let child = Scope::with_parent(parent);
        assert!(child.variables.is_empty());
        assert!(child.parent.is_some());
    }

    #[test]
    fn test_scope_get() {
        let mut scope = Scope::new();
        let ty = Type::new("i32");
        scope.insert("x".to_string(), ty.clone());
        
        assert_eq!(scope.get("x"), Some(&ty));
        assert_eq!(scope.get("y"), None);
    }

    #[test]
    fn test_scope_get_in_parent() {
        let mut parent = Scope::new();
        let ty = Type::new("i32");
        parent.insert("x".to_string(), ty.clone());
        
        let child = Scope::with_parent(parent);
        assert_eq!(child.get("x"), Some(&ty));
    }

    #[test]
    fn test_semantic_analyzer_new() {
        let analyzer = SemanticAnalyzer::new();
        assert_eq!(analyzer.scopes.len(), 1);
        assert!(!analyzer.types.is_empty()); // Should have basic types
    }
    
    #[test]
    fn test_is_type_compatible() {
        let mut analyzer = SemanticAnalyzer::new();
        let int_type = Type::new("i32");
        let bool_type = Type::new("bool");
        
        assert!(analyzer.is_type_compatible(&int_type, &int_type));
        assert!(!analyzer.is_type_compatible(&int_type, &bool_type));
    }
    
    #[test]
    fn test_resolve_literal_type() {
        let analyzer = SemanticAnalyzer::new();
        
        let int_type = analyzer.resolve_literal_type("42");
        assert_eq!(int_type.name, "i32");
        
        let bool_type = analyzer.resolve_literal_type("true");
        assert_eq!(bool_type.name, "bool");
        
        let string_type = analyzer.resolve_literal_type("hello");
        assert_eq!(string_type.name, "string");
    }
}
