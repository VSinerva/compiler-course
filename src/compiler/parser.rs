use crate::compiler::{
    ast::Expression::{self, *},
    token::{Token, TokenType},
};

pub fn parse<'source>(tokens: &[Token<'source>]) -> Expression<'source> {
    let mut pos = 0;

    parse_expression(&mut pos, tokens)
}

fn peek<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Token<'source> {
    if let Some(token) = tokens.get(*pos) {
        token.clone()
    } else if let Some(last_token) = tokens.get(*pos - 1) {
        Token::new("", TokenType::End, last_token.loc)
    } else {
        panic!("Input to parser appears to be empty!");
    }
}

fn next<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Token<'source> {
    let token = peek(pos, tokens);
    *pos += 1;
    token
}

fn next_expect_types<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
    types: &Vec<TokenType>,
) -> Token<'source> {
    let token = next(pos, tokens);

    if types.contains(&token.token_type) {
        token
    } else {
        panic!(
            "Parsing error: expected one of {:?} but found {}",
            types, token
        );
    }
}

fn next_expect_strings<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
    strings: &Vec<&str>,
) -> Token<'source> {
    let token = next(pos, tokens);

    if strings.contains(&token.text) {
        token
    } else {
        panic!(
            "Parsing error: expected one of {:?} but found {}",
            strings, token
        );
    }
}

fn next_expect_type<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
    expected_type: TokenType,
) -> Token<'source> {
    next_expect_types(pos, tokens, &vec![expected_type])
}

fn parse_int_literal<'source>(pos: &mut usize, tokens: &[Token]) -> Expression<'source> {
    let token = next_expect_type(pos, tokens, TokenType::Integer);

    IntLiteral(
        token
            .text
            .parse::<u32>()
            .unwrap_or_else(|_| panic!("Fatal parser error! Invalid value in token {token}")),
    )
}

fn parse_identifier<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Expression<'source> {
    let token = next_expect_type(pos, tokens, TokenType::Identifier);
    Identifier(token.text)
}

fn parse_term<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Expression<'source> {
    match peek(pos, tokens).token_type {
        TokenType::Integer => parse_int_literal(pos, tokens),
        TokenType::Identifier => parse_identifier(pos, tokens),
        _ => panic!("Unexpected token {}", peek(pos, tokens)),
    }
}

fn parse_expression<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Expression<'source> {
    let left = parse_term(pos, tokens);
    let operator_token = next_expect_strings(pos, tokens, &vec!["+", "-"]);
    let right = parse_term(pos, tokens);
    Expression::BinaryOp(Box::new(left), operator_token.text, Box::new(right))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::token::CodeLocation;

    fn new_int(text: &str) -> Token {
        Token::new(
            text,
            TokenType::Integer,
            CodeLocation::new(usize::MAX, usize::MAX),
        )
    }

    fn new_id(text: &str) -> Token {
        Token::new(
            text,
            TokenType::Identifier,
            CodeLocation::new(usize::MAX, usize::MAX),
        )
    }

    #[test]
    fn test_binary_op_basic() {
        let result = parse(&vec![new_int("1"), new_id("+"), new_int("23")]);
        assert_eq!(
            result,
            BinaryOp(Box::new(IntLiteral(1)), "+", Box::new(IntLiteral(23)))
        );

        let result = parse(&vec![new_int("4"), new_id("-"), new_int("56")]);
        assert_eq!(
            result,
            BinaryOp(Box::new(IntLiteral(4)), "-", Box::new(IntLiteral(56)))
        );
    }

    #[test]
    fn test_binary_op_identifier() {
        let result = parse(&vec![new_id("a"), new_id("+"), new_int("1")]);
        assert_eq!(
            result,
            BinaryOp(Box::new(Identifier("a")), "+", Box::new(IntLiteral(1)))
        );

        let result = parse(&vec![new_int("1"), new_id("-"), new_id("a")]);
        assert_eq!(
            result,
            BinaryOp(Box::new(IntLiteral(1)), "-", Box::new(Identifier("a")))
        );
    }

    #[test]
    fn test_binary_op_multiple() {
        let result = parse(&vec![
            new_int("1"),
            new_id("+"),
            new_int("2"),
            new_id("-"),
            new_int("3"),
        ]);
        assert_eq!(
            result,
            BinaryOp(
                Box::new(IntLiteral(1)),
                "+",
                Box::new(BinaryOp(
                    Box::new(IntLiteral(2)),
                    "-",
                    Box::new(IntLiteral(3))
                ))
            )
        );
    }
}
