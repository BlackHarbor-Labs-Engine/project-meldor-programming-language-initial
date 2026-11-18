// Placeholder

// Implement the parser here

pub fn parse(input: &str) -> Result<SyntaxNode, String> {
    let tokens = lexer::lex(input);
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}
