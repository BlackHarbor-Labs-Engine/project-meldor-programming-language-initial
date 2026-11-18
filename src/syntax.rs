fn parse_literal(input: &str) -> IResult<&str, SyntaxNode> {
    map_res(many1(one_of("0123456789")), |digits| digits.join().parse::<u32>())
        .map(|(remaining, result)| (remaining, SyntaxNode::Literal { value: result.to_string(), span: Span::new(0, 0, 0, 0) }))
        .parse(input)
}
