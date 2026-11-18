// Parser implementation for Meldor language
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{alphanumeric1, space0, digit1},
    sequence::{pair, tuple},
    branch::alt,
};

pub fn parse_expression(input: &str) -> IResult<&str, String> {
    let (remaining, expr) = alt((
        // Parse a simple identifier or number
        alphanumeric1,
        // Parse a literal number
        digit1,
    ))(input)?;

    Ok((remaining, expr.to_string()))
}

pub fn parse_statement(input: &str) -> IResult<&str, String> {
    // let analyzer = SemanticAnalyzer::new(); // Initialize properly
    let (remaining, _) = space0(input)?;

    // Parse a simple statement like "let x = 5" or "print(123)"
    let (remaining, stmt) = alt((
        // Parse assignment statements like "let x = 5"
        pair(
            tag("let"),
            tuple((
                space0,
                alphanumeric1,
                space0,
                tag("="),
                space0,
                alphanumeric1,
            )),
        ),
        // Parse function calls like "print(123)"
        pair(
            alphanumeric1,
            tuple((
                space0,
                tag("("),
                space0,
                alphanumeric1,
                space0,
                tag(")"),
            )),
        ),
    ))(remaining)?;

    // Call the analyzer on each token
    // for token in stmt.0.split_whitespace() {
    //     analyzer.analyze(token)?; // TODO: Fix this call
    // }

    // Return the parsed statement as a string
    Ok((remaining, stmt.0.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression() {
        assert_eq!(parse_expression("123").unwrap().1, "123");
        assert_eq!(parse_expression("abc").unwrap().1, "abc");
    }

    #[test]
    fn test_parse_statement() {
        // Test assignment statement
        let result = parse_statement("let x = 5");
        assert!(result.is_ok());

        // Test function call
        let result = parse_statement("print(123)");
        assert!(result.is_ok());
    }
}
