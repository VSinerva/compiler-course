use super::*;

pub fn consume_string<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
    expected_string: &str,
) -> Result<Token<'source>, ParserError> {
    consume_strings(pos, tokens, &[expected_string])
}

pub fn consume_strings<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
    strings: &[&str],
) -> Result<Token<'source>, ParserError> {
    let token = consume(pos, tokens)?;

    if strings.contains(&token.text) {
        Ok(token)
    } else {
        Err(ParserError {
            msg: format!("Expected one of {:?} but found {}", strings, token),
        })
    }
}

pub fn consume_type<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
    expected_type: TokenType,
) -> Result<Token<'source>, ParserError> {
    consume_types(pos, tokens, &[expected_type])
}

pub fn consume_types<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
    types: &[TokenType],
) -> Result<Token<'source>, ParserError> {
    let token = consume(pos, tokens)?;

    if types.contains(&token.token_type) {
        Ok(token)
    } else {
        Err(ParserError {
            msg: format!("Expected one of {:?} but found {}", types, token),
        })
    }
}

pub fn consume<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
) -> Result<Token<'source>, ParserError> {
    let token = peek(pos, tokens);
    *pos += 1;
    token
}

pub fn peek<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
) -> Result<Token<'source>, ParserError> {
    if let Some(token) = tokens.get(*pos) {
        Ok(token.clone())
    } else if let Some(last_token) = tokens.get(*pos - 1) {
        Ok(Token::new("", TokenType::End, last_token.loc))
    } else {
        Err(ParserError {
            msg: String::from("Input to parser appears to be empty!"),
        })
    }
}
