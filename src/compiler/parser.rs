use crate::compiler::{
    ast::Expression::{self, *},
    token::{Token, TokenType},
};

pub fn parse<'source>(tokens: &[Token<'source>]) -> Expression<'source> {
    let mut pos = 0;
    let result = parse_expression(0, &mut pos, tokens);

    if pos != tokens.len() {
        panic!(
            "Parsing naturally stopped at {}, despite there being more tokens!",
            peek(&mut pos, tokens)
        );
    }

    result
}

fn parse_expression<'source>(
    level: usize,
    pos: &mut usize,
    tokens: &[Token<'source>],
) -> Expression<'source> {
    const LEFT_ASSOC_BIN_OPS: [&[&str]; 2] = [&["+", "-"], &["*", "/"]];

    if level == LEFT_ASSOC_BIN_OPS.len() {
        match peek(pos, tokens).text {
            "if" => parse_conditional(pos, tokens),
            "(" => parse_parenthesized(pos, tokens),
            _ => parse_term(pos, tokens),
        }
    } else {
        let mut left = parse_expression(level + 1, pos, tokens);
        while LEFT_ASSOC_BIN_OPS[level].contains(&peek(pos, tokens).text) {
            let operator_token = consume_strings(pos, tokens, LEFT_ASSOC_BIN_OPS[level]);
            let right = parse_expression(level + 1, pos, tokens);

            left = BinaryOp(Box::new(left), operator_token.text, Box::new(right));
        }
        left
    }
}

fn parse_conditional<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Expression<'source> {
    consume_string(pos, tokens, "if");
    let condition = Box::new(parse_expression(0, pos, tokens));
    consume_string(pos, tokens, "then");
    let then_expr = Box::new(parse_expression(0, pos, tokens));

    let else_expr = match peek(pos, tokens).text {
        "else" => {
            consume_string(pos, tokens, "else");
            Some(Box::new(parse_expression(0, pos, tokens)))
        }
        _ => None,
    };

    Conditional(condition, then_expr, else_expr)
}

fn parse_parenthesized<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Expression<'source> {
    consume_string(pos, tokens, "(");
    let expression = parse_expression(0, pos, tokens);
    consume_string(pos, tokens, ")");
    expression
}

fn parse_term<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Expression<'source> {
    let token = peek(pos, tokens);

    match token.token_type {
        TokenType::Integer => parse_int_literal(pos, tokens),
        TokenType::Identifier => match token.text {
            "true" | "false" => parse_bool_literal(pos, tokens),
            _ => {
                if peek(&mut (*pos + 1), tokens).text == "(" {
                    parse_function(pos, tokens)
                } else {
                    parse_identifier(pos, tokens)
                }
            }
        },
        _ => panic!("Unexpected {}", token),
    }
}

fn parse_function<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Expression<'source> {
    let identifier = consume_type(pos, tokens, TokenType::Identifier);
    consume_string(pos, tokens, "(");

    let mut arguments = Vec::new();
    // If/loop used instead of while to show that we will always use break to exit the loop
    if peek(pos, tokens).text != ")" {
        loop {
            arguments.push(parse_expression(0, pos, tokens));

            match peek(pos, tokens).text {
                "," => consume_string(pos, tokens, ","),
                _ => break, // Break out of the loop. Intentionally causes a panic with a missing comma
            };
        }
    }
    consume_string(pos, tokens, ")");
    FunCall(identifier.text, arguments)
}

fn parse_int_literal<'source>(pos: &mut usize, tokens: &[Token]) -> Expression<'source> {
    let token = consume_type(pos, tokens, TokenType::Integer);

    IntLiteral(
        token
            .text
            .parse::<u32>()
            .unwrap_or_else(|_| panic!("Fatal parser error! Invalid value in token {token}")),
    )
}

fn parse_bool_literal<'source>(pos: &mut usize, tokens: &[Token]) -> Expression<'source> {
    let token = consume_type(pos, tokens, TokenType::Identifier);

    match token.text {
        "true" => BoolLiteral(true),
        "false" => BoolLiteral(false),
        _ => panic!("Fatal parser error! Expected bool literal but found {token}"),
    }
}

fn parse_identifier<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Expression<'source> {
    let token = consume_type(pos, tokens, TokenType::Identifier);
    Identifier(token.text)
}

fn consume_string<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
    expected_string: &str,
) -> Token<'source> {
    consume_strings(pos, tokens, &[expected_string])
}

fn consume_strings<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
    strings: &[&str],
) -> Token<'source> {
    let token = consume(pos, tokens);

    if strings.contains(&token.text) {
        token
    } else {
        panic!(
            "Parsing error: expected one of {:?} but found {}",
            strings, token
        );
    }
}

fn consume_type<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
    expected_type: TokenType,
) -> Token<'source> {
    consume_types(pos, tokens, &[expected_type])
}

fn consume_types<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
    types: &[TokenType],
) -> Token<'source> {
    let token = consume(pos, tokens);

    if types.contains(&token.token_type) {
        token
    } else {
        panic!(
            "Parsing error: expected one of {:?} but found {}",
            types, token
        );
    }
}

fn consume<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Token<'source> {
    let token = peek(pos, tokens);
    *pos += 1;
    token
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::token::CodeLocation;

    fn int_tok(text: &str) -> Token {
        Token::new(
            text,
            TokenType::Integer,
            CodeLocation::new(usize::MAX, usize::MAX),
        )
    }

    macro_rules! int_ast {
        ($x:expr) => {
            Box::new(IntLiteral($x))
        };
    }

    fn id_tok(text: &str) -> Token {
        Token::new(
            text,
            TokenType::Identifier,
            CodeLocation::new(usize::MAX, usize::MAX),
        )
    }

    macro_rules! id_ast {
        ($x:expr) => {
            Box::new(Identifier($x))
        };
    }

    fn punc_tok(text: &str) -> Token {
        Token::new(
            text,
            TokenType::Punctuation,
            CodeLocation::new(usize::MAX, usize::MAX),
        )
    }

    macro_rules! bin_ast {
        ($x:expr, $y:expr, $z:expr) => {
            Box::new(BinaryOp($x, $y, $z))
        };
    }

    macro_rules! bool_ast {
        ($x:expr) => {
            Box::new(BoolLiteral($x))
        };
    }

    #[test]
    fn test_binary_op_basic() {
        let result = parse(&vec![int_tok("1"), id_tok("+"), int_tok("23")]);
        assert_eq!(result, BinaryOp(int_ast!(1), "+", int_ast!(23)));

        let result = parse(&vec![int_tok("4"), id_tok("-"), int_tok("56")]);
        assert_eq!(result, BinaryOp(int_ast!(4), "-", int_ast!(56)));

        let result = parse(&vec![int_tok("1"), id_tok("*"), int_tok("2")]);
        assert_eq!(result, BinaryOp(int_ast!(1), "*", int_ast!(2)));

        let result = parse(&vec![int_tok("1"), id_tok("/"), int_tok("2")]);
        assert_eq!(result, BinaryOp(int_ast!(1), "/", int_ast!(2)));
    }

    #[test]
    fn test_binary_op_identifier() {
        let result = parse(&vec![id_tok("a"), id_tok("+"), int_tok("1")]);
        assert_eq!(result, BinaryOp(id_ast!("a"), "+", int_ast!(1)));

        let result = parse(&vec![int_tok("1"), id_tok("-"), id_tok("a")]);
        assert_eq!(result, BinaryOp(int_ast!(1), "-", id_ast!("a")));
    }

    #[test]
    fn test_binary_op_multiple() {
        let result = parse(&vec![
            int_tok("1"),
            id_tok("+"),
            int_tok("2"),
            id_tok("-"),
            int_tok("3"),
        ]);
        assert_eq!(
            result,
            BinaryOp(bin_ast!(int_ast!(1), "+", int_ast!(2)), "-", int_ast!(3))
        );
    }

    #[test]
    fn test_binary_op_precedence() {
        let result = parse(&vec![
            int_tok("1"),
            id_tok("+"),
            int_tok("2"),
            id_tok("*"),
            int_tok("3"),
        ]);
        assert_eq!(
            result,
            BinaryOp(int_ast!(1), "+", bin_ast!(int_ast!(2), "*", int_ast!(3)),)
        );

        let result = parse(&vec![
            int_tok("1"),
            id_tok("-"),
            int_tok("2"),
            id_tok("/"),
            int_tok("3"),
        ]);
        assert_eq!(
            result,
            BinaryOp(int_ast!(1), "-", bin_ast!(int_ast!(2), "/", int_ast!(3)),)
        );
    }

    #[test]
    fn test_parenthesized() {
        let result = parse(&vec![
            id_tok("("),
            int_tok("1"),
            id_tok("+"),
            int_tok("2"),
            id_tok(")"),
            id_tok("*"),
            int_tok("3"),
        ]);
        assert_eq!(
            result,
            BinaryOp(bin_ast!(int_ast!(1), "+", int_ast!(2)), "*", int_ast!(3),)
        );

        let result = parse(&vec![
            id_tok("("),
            id_tok("("),
            int_tok("1"),
            id_tok("-"),
            int_tok("2"),
            id_tok(")"),
            id_tok(")"),
            id_tok("/"),
            int_tok("3"),
        ]);
        assert_eq!(
            result,
            BinaryOp(bin_ast!(int_ast!(1), "-", int_ast!(2)), "/", int_ast!(3),)
        );
    }

    #[test]
    fn test_if_then() {
        let result = parse(&vec![
            id_tok("if"),
            int_tok("1"),
            id_tok("+"),
            int_tok("2"),
            id_tok("then"),
            int_tok("3"),
        ]);
        assert_eq!(
            result,
            Conditional(bin_ast!(int_ast!(1), "+", int_ast!(2)), int_ast!(3), None,)
        );
    }

    #[test]
    fn test_if_then_else() {
        let result = parse(&vec![
            id_tok("if"),
            id_tok("a"),
            id_tok("then"),
            id_tok("b"),
            id_tok("+"),
            id_tok("c"),
            id_tok("else"),
            int_tok("1"),
            id_tok("*"),
            int_tok("2"),
        ]);
        assert_eq!(
            result,
            Conditional(
                id_ast!("a"),
                bin_ast!(id_ast!("b"), "+", id_ast!("c")),
                Some(bin_ast!(int_ast!(1), "*", int_ast!(2)))
            )
        );
    }

    #[test]
    fn test_embedded_if_then_else() {
        let result = parse(&vec![
            int_tok("1"),
            id_tok("+"),
            id_tok("if"),
            id_tok("true"),
            id_tok("then"),
            int_tok("2"),
            id_tok("else"),
            int_tok("3"),
        ]);
        assert_eq!(
            result,
            BinaryOp(
                int_ast!(1),
                "+",
                Box::new(Conditional(bool_ast!(true), int_ast!(2), Some(int_ast!(3))))
            )
        );
    }

    #[test]
    fn test_nested_if_then_else() {
        let result = parse(&vec![
            id_tok("if"),
            id_tok("true"),
            id_tok("then"),
            id_tok("if"),
            id_tok("false"),
            id_tok("then"),
            int_tok("1"),
            id_tok("else"),
            int_tok("2"),
            id_tok("else"),
            int_tok("3"),
        ]);
        assert_eq!(
            result,
            Conditional(
                bool_ast!(true),
                Box::new(Conditional(
                    bool_ast!(false),
                    int_ast!(1),
                    Some(int_ast!(2))
                )),
                Some(int_ast!(3))
            )
        );
    }

    #[test]
    fn test_func_basic() {
        let result = parse(&vec![
            id_tok("f"),
            punc_tok("("),
            id_tok("a"),
            punc_tok(","),
            id_tok("b"),
            punc_tok(")"),
        ]);
        assert_eq!(
            result,
            FunCall("f", vec![Identifier("a"), Identifier("b"),])
        );

        let result = parse(&vec![
            id_tok("f"),
            punc_tok("("),
            id_tok("a"),
            punc_tok(","),
            int_tok("1"),
            id_tok("+"),
            int_tok("2"),
            punc_tok(")"),
        ]);
        assert_eq!(
            result,
            FunCall(
                "f",
                vec![Identifier("a"), BinaryOp(int_ast!(1), "+", int_ast!(2),),]
            )
        );

        let result = parse(&vec![id_tok("f"), punc_tok("("), punc_tok(")")]);
        assert_eq!(result, FunCall("f", vec![]));
    }

    #[test]
    fn test_func_nested() {
        let result = parse(&vec![
            id_tok("f"),
            punc_tok("("),
            id_tok("a"),
            punc_tok(","),
            id_tok("g"),
            punc_tok("("),
            id_tok("b"),
            punc_tok(")"),
            punc_tok(")"),
        ]);
        assert_eq!(
            result,
            FunCall(
                "f",
                vec![Identifier("a"), FunCall("g", vec![Identifier("b")]),]
            )
        );
    }

    #[test]
    fn test_func_embedded() {
        let result = parse(&vec![
            int_tok("1"),
            id_tok("+"),
            id_tok("f"),
            punc_tok("("),
            id_tok("a"),
            punc_tok(")"),
        ]);
        assert_eq!(
            result,
            BinaryOp(
                int_ast!(1),
                "+",
                Box::new(FunCall("f", vec![Identifier("a")]))
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_parenthesized_mismatched() {
        parse(&vec![
            id_tok("("),
            int_tok("1"),
            id_tok("+"),
            int_tok("2"),
            id_tok("*"),
            int_tok("3"),
        ]);
    }

    #[test]
    #[should_panic]
    fn test_func_missing_comma() {
        parse(&vec![
            id_tok("f"),
            punc_tok("("),
            id_tok("a"),
            id_tok("b"),
            punc_tok(")"),
        ]);
    }

    #[test]
    #[should_panic]
    fn test_func_missing_close() {
        parse(&vec![id_tok("f"), punc_tok("("), id_tok("a")]);
    }

    #[test]
    #[should_panic]
    fn test_if_no_then() {
        parse(&vec![id_tok("if"), id_tok("true")]);
    }

    #[test]
    #[should_panic]
    fn test_empty() {
        parse(&vec![]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_start() {
        parse(&vec![int_tok("1"), int_tok("2"), id_tok("+"), int_tok("3")]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_middle() {
        parse(&vec![
            int_tok("1"),
            id_tok("+"),
            int_tok("2"),
            int_tok("2"),
            id_tok("+"),
            int_tok("3"),
        ]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_end() {
        parse(&vec![
            int_tok("1"),
            id_tok("+"),
            int_tok("2"),
            int_tok("2"),
            id_tok("+"),
            int_tok("3"),
        ]);
    }
}
