use nom::{IResult, bytes::complete::tag, character::complete::{digit1, alpha1, alphanumeric1}, combinator::recognize, multi::many0, sequence::tuple};
use crate::syntax::{SyntaxNode, Span};

pub fn lexer(input: &str) -> Result<Vec<SyntaxNode>, String> {
    let mut syntax_nodes = Vec::new();
    let mut remaining = input;
    
    while !remaining.is_empty() {
        match tokenize(remaining) {
            Ok((rem, node)) => {
                syntax_nodes.push(node);
                remaining = rem;
            }
            Err(e) => return Err(format!("Lexing error: {:?}", e)),
        }
    }
    
    Ok(syntax_nodes)
}

fn tokenize(input: &str) -> IResult<&str, SyntaxNode> {
    // Use nom::branch::alt instead of nom::alt
    let (input, token) = nom::branch::alt((
        keyword,
        identifier,
        literal,
    ))(input)?;
    Ok((input, token))
}

fn keyword(input: &str) -> IResult<&str, SyntaxNode> {
    let (input, keyword_str) = nom::branch::alt((
        tag("let"),
        tag("fn"),
        tag("if"),
        tag("else"),
        tag("while"),
        tag("for"),
        tag("return")
    ))(input)?;
    
    Ok((input, SyntaxNode::Keyword(keyword_str.to_string())))
}

fn identifier(input: &str) -> IResult<&str, SyntaxNode> {
    let (input, ident) = recognize(tuple((
        alpha1,
        many0(alphanumeric1)
    )))(input)?;
    
    Ok((input, SyntaxNode::Identifier(ident.to_string())))
}

fn literal(input: &str) -> IResult<&str, SyntaxNode> {
    let (input, lit) = recognize(digit1)(input)?;
    
    Ok((input, SyntaxNode::Literal { 
        value: lit.to_string(), 
        span: Span::new(0, 0, 0, 0) 
    }))
}
