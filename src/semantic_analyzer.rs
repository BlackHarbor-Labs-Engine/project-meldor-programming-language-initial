/* ... (content up to line 29) ... */
        NodeType::Literal(lit_node) => {
            println!("Analyzing Literal: {:?}", lit_node);
            Ok(())
        },
        NodeType::Identifier(id) => {
            println!("Analyzing Identifier: {}", id);
            Ok(())
        },
        NodeType::UnknownNode => {
            Err("Semantic Error: Encountered an unknown node type during analysis.".to_string())
        }
    }
}
