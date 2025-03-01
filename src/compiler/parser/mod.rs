mod parser_utilities;
#[cfg(test)]
mod tests;

use std::{error::Error, fmt::Display};

use crate::compiler::{
    ast::{
        AstNode,
        Expression::*,
        TypeExpression::{self},
    },
    parser::parser_utilities::*,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct ParserError {
    msg: String,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParserError: {}", self.msg)
    }
}

impl Error for ParserError {}

pub fn parse<'source>(tokens: &[Token<'source>]) -> Result<AstNode<'source>, ParserError> {
    let mut pos = 0;

    let first_expression = parse_block_level_expressions(&mut pos, tokens)?;

    if pos != tokens.len() {
        let mut expressions = vec![first_expression];

        // Blocks don't need to be followed by a semicolon, but can be
        if peek(&mut (pos - 1), tokens)?.text == "}" {
            if peek(&mut pos, tokens)?.text == ";" {
                consume_string(&mut pos, tokens, ";")?;
            }
        } else {
            consume_string(&mut pos, tokens, ";")?;
        }

        while peek(&mut pos, tokens)?.token_type != TokenType::End {
            expressions.push(parse_block_level_expressions(&mut pos, tokens)?);

            if peek(&mut pos, tokens)?.token_type == TokenType::End {
                break;
            }

            // Blocks don't need to be followed by a semicolon, but can be
            if peek(&mut (pos - 1), tokens)?.text == "}" {
                if peek(&mut pos, tokens)?.text == ";" {
                    consume_string(&mut pos, tokens, ";")?;
                }
            } else {
                consume_string(&mut pos, tokens, ";")?;
            }
        }

        let last_token = peek(&mut (pos - 1), tokens)?;
        if last_token.text == ";" {
            expressions.push(AstNode::new(last_token.loc, EmptyLiteral()));
        }

        Ok(AstNode::new(tokens[0].loc, Block(expressions)))
    } else {
        Ok(first_expression)
    }
}

// Horrible name, basically used to get the full expressions contained
// in blocks or at the top level of the program
fn parse_block_level_expressions<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
) -> Result<AstNode<'source>, ParserError> {
    // Special handling for variable declaration, since it is only allowed in very specifc places
    if peek(pos, tokens)?.text == "var" {
        parse_var_declaration(pos, tokens)
    } else {
        parse_expression(0, pos, tokens)
    }
}

fn parse_expression<'source>(
    level: usize,
    pos: &mut usize,
    tokens: &[Token<'source>],
) -> Result<AstNode<'source>, ParserError> {
    const OPS: [&[&str]; 8] = [
        &["="],                  // 0
        &["or"],                 // 1
        &["and"],                // 2
        &["==", "!="],           // 3
        &["<", "<=", ">=", ">"], // 4
        &["+", "-"],             // 5
        &["*", "/", "%"],        // 6
        &["not", "-"],           // 7
                                 // 8, everything not explicitly listed above goes here
    ];

    match level {
        0 => {
            let left = parse_expression(level + 1, pos, tokens)?;
            if OPS[level].contains(&peek(pos, tokens)?.text) {
                let operator_token = consume_strings(pos, tokens, OPS[level])?;
                let right = parse_expression(level, pos, tokens)?;
                Ok(AstNode::new(
                    operator_token.loc,
                    BinaryOp(Box::new(left), operator_token.text, Box::new(right)),
                ))
            } else {
                Ok(left)
            }
        }
        1..=6 => {
            let mut left = parse_expression(level + 1, pos, tokens)?;
            while OPS[level].contains(&peek(pos, tokens)?.text) {
                let operator_token = consume_strings(pos, tokens, OPS[level])?;
                let right = parse_expression(level + 1, pos, tokens)?;

                left = AstNode::new(
                    operator_token.loc,
                    BinaryOp(Box::new(left), operator_token.text, Box::new(right)),
                );
            }
            Ok(left)
        }
        7 => {
            if OPS[level].contains(&peek(pos, tokens)?.text) {
                let operator_token = consume_strings(pos, tokens, OPS[level])?;
                let right = parse_expression(level, pos, tokens)?;
                Ok(AstNode::new(
                    operator_token.loc,
                    UnaryOp(operator_token.text, Box::new(right)),
                ))
            } else {
                parse_expression(level + 1, pos, tokens)
            }
        }
        8 => parse_term(pos, tokens),
        _ => unreachable!(),
    }
}

fn parse_term<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
) -> Result<AstNode<'source>, ParserError> {
    let token = peek(pos, tokens)?;

    match token.token_type {
        TokenType::Integer => Ok(parse_int_literal(pos, tokens)?),
        TokenType::Identifier => match token.text {
            "if" => parse_conditional(pos, tokens),
            "while" => parse_while_loop(pos, tokens),
            "true" | "false" => parse_bool_literal(pos, tokens),
            "var" => Err(ParserError {
                msg: format!("Invalid variable declaration {}", token),
            }),
            _ => {
                if peek(&mut (*pos + 1), tokens)?.text == "(" {
                    parse_function(pos, tokens)
                } else {
                    parse_identifier(pos, tokens)
                }
            }
        },
        TokenType::Punctuation => match token.text {
            "(" => parse_parenthesized(pos, tokens),
            "{" => parse_block(pos, tokens),
            _ => Err(ParserError {
                msg: format!("Unexpected {}", token),
            }),
        },
        _ => Err(ParserError {
            msg: format!("Unexpected {}", token),
        }),
    }
}

fn parse_var_declaration<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
) -> Result<AstNode<'source>, ParserError> {
    consume_string(pos, tokens, "var")?;
    let name_token = consume_type(pos, tokens, TokenType::Identifier)?;

    let mut type_expr = None;
    if peek(pos, tokens)?.text == ":" {
        consume_string(pos, tokens, ":")?;

        let type_token = consume_type(pos, tokens, TokenType::Identifier)?;
        type_expr = match type_token.text {
            "Int" => Some(TypeExpression::Int(type_token.loc)),
            "Bool" => Some(TypeExpression::Bool(type_token.loc)),
            _ => {
                return Err(ParserError {
                    msg: format!("Invalid type specifier {}", type_token.text),
                });
            }
        }
    }

    consume_string(pos, tokens, "=")?;
    let value = parse_expression(0, pos, tokens)?;
    Ok(AstNode::new(
        name_token.loc,
        VarDeclaration(name_token.text, Box::new(value), type_expr),
    ))
}

fn parse_conditional<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
) -> Result<AstNode<'source>, ParserError> {
    let start = consume_string(pos, tokens, "if")?;
    let condition = Box::new(parse_expression(0, pos, tokens)?);
    consume_string(pos, tokens, "then")?;
    let then_expr = Box::new(parse_expression(0, pos, tokens)?);

    let else_expr = match peek(pos, tokens)?.text {
        "else" => {
            consume_string(pos, tokens, "else")?;
            Some(Box::new(parse_expression(0, pos, tokens)?))
        }
        _ => None,
    };

    Ok(AstNode::new(
        start.loc,
        Conditional(condition, then_expr, else_expr),
    ))
}

fn parse_while_loop<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
) -> Result<AstNode<'source>, ParserError> {
    let start = consume_string(pos, tokens, "while")?;
    let condition = Box::new(parse_expression(0, pos, tokens)?);
    consume_string(pos, tokens, "do")?;
    let do_expr = Box::new(parse_expression(0, pos, tokens)?);

    Ok(AstNode::new(start.loc, While(condition, do_expr)))
}

fn parse_parenthesized<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
) -> Result<AstNode<'source>, ParserError> {
    consume_string(pos, tokens, "(")?;
    let expression = parse_expression(0, pos, tokens)?;
    consume_string(pos, tokens, ")")?;
    Ok(expression)
}

fn parse_block<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
) -> Result<AstNode<'source>, ParserError> {
    let start = consume_string(pos, tokens, "{")?;

    let mut expressions = Vec::new();
    while peek(pos, tokens)?.text != "}" {
        expressions.push(parse_block_level_expressions(pos, tokens)?);

        // Last expression left as return expression, if no semicolon is present
        if peek(pos, tokens)?.text == "}" {
            break;
        }

        // Blocks don't need to be followed by a semicolon, but can be
        if peek(&mut (*pos - 1), tokens)?.text == "}" {
            if peek(pos, tokens)?.text == ";" {
                consume_string(pos, tokens, ";")?;
            }
        } else {
            consume_string(pos, tokens, ";")?;
        }

        // If the last expression of the block ended in a semicolon, empty return
        let next_token = peek(pos, tokens)?;
        if next_token.text == "}" {
            expressions.push(AstNode::new(next_token.loc, EmptyLiteral()));
            break;
        }
    }

    consume_string(pos, tokens, "}")?;
    Ok(AstNode::new(start.loc, Block(expressions)))
}

fn parse_function<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
) -> Result<AstNode<'source>, ParserError> {
    let identifier = consume_type(pos, tokens, TokenType::Identifier)?;
    consume_string(pos, tokens, "(")?;

    let mut arguments = Vec::new();
    // If/loop used instead of while to show that we will always use break to exit the loop
    if peek(pos, tokens)?.text != ")" {
        loop {
            arguments.push(parse_expression(0, pos, tokens)?);

            match peek(pos, tokens)?.text {
                "," => consume_string(pos, tokens, ",")?,
                _ => break, // Break out of the loop. Intentionally causes an error with a missing comma
            };
        }
    }
    consume_string(pos, tokens, ")")?;
    Ok(AstNode::new(
        identifier.loc,
        FunCall(identifier.text, arguments),
    ))
}

fn parse_int_literal<'source>(
    pos: &mut usize,
    tokens: &[Token],
) -> Result<AstNode<'source>, ParserError> {
    let token = consume_type(pos, tokens, TokenType::Integer)?;

    let expr = match token.text.parse::<i128>() {
        Ok(val) => IntLiteral(val),
        Err(_) => {
            return Err(ParserError {
                msg: format!("Invalid value in token {token}"),
            });
        }
    };

    Ok(AstNode::new(token.loc, expr))
}

fn parse_bool_literal<'source>(
    pos: &mut usize,
    tokens: &[Token],
) -> Result<AstNode<'source>, ParserError> {
    let token = consume_type(pos, tokens, TokenType::Identifier)?;

    let expr = match token.text {
        "true" => BoolLiteral(true),
        "false" => BoolLiteral(false),
        _ => {
            return Err(ParserError {
                msg: format!("Expected bool literal but found {token}"),
            })
        }
    };

    Ok(AstNode::new(token.loc, expr))
}

fn parse_identifier<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
) -> Result<AstNode<'source>, ParserError> {
    let token = consume_type(pos, tokens, TokenType::Identifier)?;
    Ok(AstNode::new(token.loc, Identifier(token.text)))
}
