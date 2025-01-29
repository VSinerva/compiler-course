use crate::compiler::{
    ast::Expression::{self, *},
    token::{Token, TokenType},
};

pub fn parse<'source>(tokens: &[Token<'source>]) -> Expression<'source> {
    let mut pos = 0;

    let result = parse_expression(&mut pos, tokens);

    if pos != tokens.len() {
        panic!(
            "Parsing naturally stopped at {}, despite there being more tokens!",
            peek(&mut pos, tokens)
        );
    }

    result
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

fn next_expect_string<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
    expected_string: &str,
) -> Token<'source> {
    next_expect_strings(pos, tokens, &vec![expected_string])
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

fn parse_bool_literal<'source>(pos: &mut usize, tokens: &[Token]) -> Expression<'source> {
    let token = next_expect_type(pos, tokens, TokenType::Identifier);

    match token.text {
        "true" => BoolLiteral(true),
        "false" => BoolLiteral(false),
        _ => panic!("Fatal parser error! Expected bool literal but found {token}"),
    }
}

fn parse_function<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Expression<'source> {
    let identifier = next_expect_type(pos, tokens, TokenType::Identifier);

    next_expect_string(pos, tokens, "(");

    let mut arguments = Vec::new();

    // If/loop used instead of while to show that we will always use break to exit the loop
    if peek(pos, tokens).text != ")" {
        loop {
            arguments.push(Box::new(parse_expression(pos, tokens)));

            match peek(pos, tokens).text {
                "," => next_expect_string(pos, tokens, ","),
                _ => break, // Break out of the loop. Intentionally causes a panic with a missing comma
            };
        }
    }
    next_expect_string(pos, tokens, ")");

    FunCall(identifier.text, arguments)
}

fn parse_identifier<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Expression<'source> {
    if peek(&mut (*pos + 1), tokens).text == "(" {
        parse_function(pos, tokens)
    } else {
        let token = next_expect_type(pos, tokens, TokenType::Identifier);
        Identifier(token.text)
    }
}

fn parse_parenthesized<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Expression<'source> {
    next_expect_string(pos, tokens, "(");
    let expression = parse_expression(pos, tokens);
    next_expect_string(pos, tokens, ")");
    expression
}

fn parse_factor<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Expression<'source> {
    let token = peek(pos, tokens);

    if token.text == "(" {
        return parse_parenthesized(pos, tokens);
    }
    match token.token_type {
        TokenType::Integer => parse_int_literal(pos, tokens),
        TokenType::Identifier => match token.text {
            "true" | "false" => parse_bool_literal(pos, tokens),
            _ => parse_identifier(pos, tokens),
        },
        _ => panic!("Unexpected {}", token),
    }
}

fn parse_conditional<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Expression<'source> {
    next_expect_string(pos, tokens, "if");
    let condition = Box::new(parse_expression(pos, tokens));
    next_expect_string(pos, tokens, "then");
    let then_expr = Box::new(parse_expression(pos, tokens));

    let else_expr = match peek(pos, tokens).text {
        "else" => {
            next_expect_string(pos, tokens, "else");
            Some(Box::new(parse_expression(pos, tokens)))
        }
        _ => None,
    };

    Conditional(condition, then_expr, else_expr)
}

fn parse_term<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Expression<'source> {
    match peek(pos, tokens).text {
        "if" => parse_conditional(pos, tokens),
        _ => {
            let mut left = parse_factor(pos, tokens);
            while ["*", "/"].contains(&peek(pos, tokens).text) {
                let operator_token = next_expect_strings(pos, tokens, &vec!["*", "/"]);
                let right = parse_factor(pos, tokens);

                left = BinaryOp(Box::new(left), operator_token.text, Box::new(right));
            }
            left
        }
    }
}

fn parse_expression<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Expression<'source> {
    let mut left = parse_term(pos, tokens);

    while ["+", "-"].contains(&peek(pos, tokens).text) {
        let operator_token = next_expect_strings(pos, tokens, &vec!["+", "-"]);
        let right = parse_term(pos, tokens);

        left = BinaryOp(Box::new(left), operator_token.text, Box::new(right));
    }

    left
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

    fn new_punc(text: &str) -> Token {
        Token::new(
            text,
            TokenType::Punctuation,
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

        let result = parse(&vec![new_int("1"), new_id("*"), new_int("2")]);
        assert_eq!(
            result,
            BinaryOp(Box::new(IntLiteral(1)), "*", Box::new(IntLiteral(2)))
        );

        let result = parse(&vec![new_int("1"), new_id("/"), new_int("2")]);
        assert_eq!(
            result,
            BinaryOp(Box::new(IntLiteral(1)), "/", Box::new(IntLiteral(2)))
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
                Box::new(BinaryOp(
                    Box::new(IntLiteral(1)),
                    "+",
                    Box::new(IntLiteral(2))
                )),
                "-",
                Box::new(IntLiteral(3))
            )
        );
    }

    #[test]
    fn test_binary_op_precedence() {
        let result = parse(&vec![
            new_int("1"),
            new_id("+"),
            new_int("2"),
            new_id("*"),
            new_int("3"),
        ]);
        assert_eq!(
            result,
            BinaryOp(
                Box::new(IntLiteral(1)),
                "+",
                Box::new(BinaryOp(
                    Box::new(IntLiteral(2)),
                    "*",
                    Box::new(IntLiteral(3))
                )),
            )
        );

        let result = parse(&vec![
            new_int("1"),
            new_id("-"),
            new_int("2"),
            new_id("/"),
            new_int("3"),
        ]);
        assert_eq!(
            result,
            BinaryOp(
                Box::new(IntLiteral(1)),
                "-",
                Box::new(BinaryOp(
                    Box::new(IntLiteral(2)),
                    "/",
                    Box::new(IntLiteral(3))
                )),
            )
        );
    }

    #[test]
    fn test_parenthesized() {
        let result = parse(&vec![
            new_id("("),
            new_int("1"),
            new_id("+"),
            new_int("2"),
            new_id(")"),
            new_id("*"),
            new_int("3"),
        ]);
        assert_eq!(
            result,
            BinaryOp(
                Box::new(BinaryOp(
                    Box::new(IntLiteral(1)),
                    "+",
                    Box::new(IntLiteral(2))
                )),
                "*",
                Box::new(IntLiteral(3)),
            )
        );

        let result = parse(&vec![
            new_id("("),
            new_id("("),
            new_int("1"),
            new_id("-"),
            new_int("2"),
            new_id(")"),
            new_id(")"),
            new_id("/"),
            new_int("3"),
        ]);
        assert_eq!(
            result,
            BinaryOp(
                Box::new(BinaryOp(
                    Box::new(IntLiteral(1)),
                    "-",
                    Box::new(IntLiteral(2))
                )),
                "/",
                Box::new(IntLiteral(3)),
            )
        );
    }

    #[test]
    fn test_if_then() {
        let result = parse(&vec![
            new_id("if"),
            new_int("1"),
            new_id("+"),
            new_int("2"),
            new_id("then"),
            new_int("3"),
        ]);
        assert_eq!(
            result,
            Conditional(
                Box::new(BinaryOp(
                    Box::new(IntLiteral(1)),
                    "+",
                    Box::new(IntLiteral(2))
                )),
                Box::new(IntLiteral(3)),
                None,
            )
        );
    }

    #[test]
    fn test_if_then_else() {
        let result = parse(&vec![
            new_id("if"),
            new_id("a"),
            new_id("then"),
            new_id("b"),
            new_id("+"),
            new_id("c"),
            new_id("else"),
            new_int("1"),
            new_id("*"),
            new_int("2"),
        ]);
        assert_eq!(
            result,
            Conditional(
                Box::new(Identifier("a")),
                Box::new(BinaryOp(
                    Box::new(Identifier("b")),
                    "+",
                    Box::new(Identifier("c")),
                )),
                Some(Box::new(BinaryOp(
                    Box::new(IntLiteral(1)),
                    "*",
                    Box::new(IntLiteral(2)),
                )))
            )
        );
    }

    #[test]
    fn test_embedded_if_then_else() {
        let result = parse(&vec![
            new_int("1"),
            new_id("+"),
            new_id("if"),
            new_id("true"),
            new_id("then"),
            new_int("2"),
            new_id("else"),
            new_int("3"),
        ]);
        assert_eq!(
            result,
            BinaryOp(
                Box::new(IntLiteral(1)),
                "+",
                Box::new(Conditional(
                    Box::new(BoolLiteral(true)),
                    Box::new(IntLiteral(2)),
                    Some(Box::new(IntLiteral(3)))
                ))
            )
        );
    }

    #[test]
    fn test_nested_if_then_else() {
        let result = parse(&vec![
            new_id("if"),
            new_id("true"),
            new_id("then"),
            new_id("if"),
            new_id("false"),
            new_id("then"),
            new_int("1"),
            new_id("else"),
            new_int("2"),
            new_id("else"),
            new_int("3"),
        ]);
        assert_eq!(
            result,
            Conditional(
                Box::new(BoolLiteral(true)),
                Box::new(Conditional(
                    Box::new(BoolLiteral(false)),
                    Box::new(IntLiteral(1)),
                    Some(Box::new(IntLiteral(2)))
                )),
                Some(Box::new(IntLiteral(3)))
            )
        );
    }

    #[test]
    fn test_func_basic() {
        let result = parse(&vec![
            new_id("f"),
            new_punc("("),
            new_id("a"),
            new_punc(","),
            new_id("b"),
            new_punc(")"),
        ]);
        assert_eq!(
            result,
            FunCall(
                "f",
                vec![Box::new(Identifier("a")), Box::new(Identifier("b")),]
            )
        );

        let result = parse(&vec![
            new_id("f"),
            new_punc("("),
            new_id("a"),
            new_punc(","),
            new_int("1"),
            new_id("+"),
            new_int("2"),
            new_punc(")"),
        ]);
        assert_eq!(
            result,
            FunCall(
                "f",
                vec![
                    Box::new(Identifier("a")),
                    Box::new(BinaryOp(
                        Box::new(IntLiteral(1)),
                        "+",
                        Box::new(IntLiteral(2)),
                    )),
                ]
            )
        );

        let result = parse(&vec![new_id("f"), new_punc("("), new_punc(")")]);
        assert_eq!(result, FunCall("f", vec![]));
    }

    #[test]
    fn test_func_nested() {
        let result = parse(&vec![
            new_id("f"),
            new_punc("("),
            new_id("a"),
            new_punc(","),
            new_id("g"),
            new_punc("("),
            new_id("b"),
            new_punc(")"),
            new_punc(")"),
        ]);
        assert_eq!(
            result,
            FunCall(
                "f",
                vec![
                    Box::new(Identifier("a")),
                    Box::new(FunCall("g", vec![Box::new(Identifier("b"))])),
                ]
            )
        );
    }

    #[test]
    fn test_func_embedded() {
        let result = parse(&vec![
            new_int("1"),
            new_id("+"),
            new_id("f"),
            new_punc("("),
            new_id("a"),
            new_punc(")"),
        ]);
        assert_eq!(
            result,
            BinaryOp(
                Box::new(IntLiteral(1)),
                "+",
                Box::new(FunCall("f", vec![Box::new(Identifier("a"))]))
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_parenthesized_mismatched() {
        parse(&vec![
            new_id("("),
            new_int("1"),
            new_id("+"),
            new_int("2"),
            new_id("*"),
            new_int("3"),
        ]);
    }

    #[test]
    #[should_panic]
    fn test_func_missing_comma() {
        parse(&vec![
            new_id("f"),
            new_punc("("),
            new_id("a"),
            new_id("b"),
            new_punc(")"),
        ]);
    }

    #[test]
    #[should_panic]
    fn test_func_missing_close() {
        parse(&vec![new_id("f"), new_punc("("), new_id("a")]);
    }

    #[test]
    #[should_panic]
    fn test_if_no_then() {
        parse(&vec![new_id("if"), new_id("true")]);
    }

    #[test]
    #[should_panic]
    fn test_empty() {
        parse(&vec![]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_start() {
        parse(&vec![new_int("1"), new_int("2"), new_id("+"), new_int("3")]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_middle() {
        parse(&vec![
            new_int("1"),
            new_id("+"),
            new_int("2"),
            new_int("2"),
            new_id("+"),
            new_int("3"),
        ]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_end() {
        parse(&vec![
            new_int("1"),
            new_id("+"),
            new_int("2"),
            new_int("2"),
            new_id("+"),
            new_int("3"),
        ]);
    }
}
