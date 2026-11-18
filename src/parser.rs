// Placeholder for parser implementation

use nom::{IResult, Parser};

pub fn parse_expression(input: &str) -> IResult<&str, String> {
    nom::character::complete::alphanumeric1(input)
}

pub fn parse_statement(input: &str) -> IResult<&str, String> {
    nom::character::complete::alphanumeric1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression() {
        assert_eq!(parse_expression("123").unwrap().1, "123");
    }

    #[test]
    fn test_parse_statement() {
        assert_eq!(parse_statement("print(123)").unwrap().1, "print(123)");
    }
}
