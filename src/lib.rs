// src/lib.rs
//! The main library crate for Meldor.

pub mod syntax;
pub mod lexer;
pub mod parser;
pub mod analyze;
pub mod ir;
pub mod interpreter;

// Re-exporting core types for easy access
pub use syntax::{SyntaxNode, Type, Analyzer};
