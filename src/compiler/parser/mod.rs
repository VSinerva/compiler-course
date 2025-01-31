mod parser_utilities;
#[cfg(test)]
mod tests;

use crate::compiler::{
    ast::Expression::{self, *},
    parser::parser_utilities::*,
    token::{Token, TokenType},
};

pub fn parse<'source>(tokens: &[Token<'source>]) -> Expression<'source> {
    let mut pos = 0;
    let result = parse_block_level_expressions(&mut pos, tokens);

    if pos != tokens.len() {
        panic!(
            "Parsing naturally stopped at {}, despite there being more tokens!",
            peek(&mut pos, tokens)
        );
    }

    result
}

// Horrible name, basically used to get the full expressions contained
// in blocks or at the top level of the program
fn parse_block_level_expressions<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
) -> Expression<'source> {
    // Special handling for variable declaration, since it is only allowed in very specifc places
    if peek(pos, tokens).text == "var" {
        parse_var_declaration(pos, tokens)
    } else {
        parse_expression(0, pos, tokens)
    }
}

fn parse_expression<'source>(
    level: usize,
    pos: &mut usize,
    tokens: &[Token<'source>],
) -> Expression<'source> {
    const OPS: [&[&str]; 8] = [
        &["="],                  // 0
        &["or"],                 // 1
        &["and"],                // 2
        &["==", "!="],           // 3
        &["<", "<=", "=>", ">"], // 4
        &["+", "-"],             // 5
        &["*", "/", "%"],        // 6
        &["not", "-"],           // 7
                                 // 8, everything not explicitly listed above goes here
    ];

    match level {
        0 => {
            let left = parse_expression(level + 1, pos, tokens);
            if OPS[level].contains(&peek(pos, tokens).text) {
                let operator_token = consume_strings(pos, tokens, OPS[level]);
                let right = parse_expression(level, pos, tokens);
                BinaryOp(Box::new(left), operator_token.text, Box::new(right))
            } else {
                left
            }
        }
        1..=6 => {
            let mut left = parse_expression(level + 1, pos, tokens);
            while OPS[level].contains(&peek(pos, tokens).text) {
                let operator_token = consume_strings(pos, tokens, OPS[level]);
                let right = parse_expression(level + 1, pos, tokens);

                left = BinaryOp(Box::new(left), operator_token.text, Box::new(right));
            }
            left
        }
        7 => {
            if OPS[level].contains(&peek(pos, tokens).text) {
                let operator_token = consume_strings(pos, tokens, OPS[level]);
                let right = parse_expression(level, pos, tokens);
                UnaryOp(operator_token.text, Box::new(right))
            } else {
                parse_expression(level + 1, pos, tokens)
            }
        }
        8 => parse_term(pos, tokens),
        _ => unreachable!(),
    }
}

fn parse_term<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Expression<'source> {
    let token = peek(pos, tokens);

    match token.token_type {
        TokenType::Integer => parse_int_literal(pos, tokens),
        TokenType::Identifier => match token.text {
            "if" => parse_conditional(pos, tokens),
            "true" | "false" => parse_bool_literal(pos, tokens),
            "var" => panic!("Invalid variable declaration {}", token),
            _ => {
                if peek(&mut (*pos + 1), tokens).text == "(" {
                    parse_function(pos, tokens)
                } else {
                    parse_identifier(pos, tokens)
                }
            }
        },
        TokenType::Punctuation => match token.text {
            "(" => parse_parenthesized(pos, tokens),
            "{" => parse_block(pos, tokens),
            _ => unreachable!(),
        },
        _ => panic!("Unexpected {}", token),
    }
}

fn parse_var_declaration<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
) -> Expression<'source> {
    consume_string(pos, tokens, "var");
    let name = consume_type(pos, tokens, TokenType::Identifier).text;
    consume_string(pos, tokens, "=");
    let value = parse_expression(0, pos, tokens);
    VarDeclaration(name, Box::new(value))
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

fn parse_block<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Expression<'source> {
    consume_string(pos, tokens, "{");

    let mut expressions = Vec::new();
    loop {
        expressions.push(parse_block_level_expressions(pos, tokens));

        if peek(pos, tokens).text == "}" {
            break;
        }
        consume_string(pos, tokens, ";");
        if peek(pos, tokens).text == "}" {
            expressions.push(EmptyLiteral());
            break;
        }
    }

    consume_string(pos, tokens, "}");
    Block(expressions)
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
