use nom::{IResult, combinator::delimited, character::complete::char, multi::take_while1, sequence::tuple, branch::alt, character::complete::tag};

fn literal(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        take_while1(|c: char| c != '"'),
        char('"'),
    )(input).map(|(remaining, result)| (remaining, result.to_string()))
}

fn keyword(input: &str) -> IResult<&str, String> {
    alt((
        tag("let"),
        tag("in"),
        tag("fn"),
        tag("return"),
    ))(input).map(|(remaining, result)| (remaining, result.to_string()))
}
