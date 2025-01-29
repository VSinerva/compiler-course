use super::*;

pub fn consume_string<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
    expected_string: &str,
) -> Token<'source> {
    consume_strings(pos, tokens, &[expected_string])
}

pub fn consume_strings<'source>(
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

pub fn consume_type<'source>(
    pos: &mut usize,
    tokens: &[Token<'source>],
    expected_type: TokenType,
) -> Token<'source> {
    consume_types(pos, tokens, &[expected_type])
}

pub fn consume_types<'source>(
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

pub fn consume<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Token<'source> {
    let token = peek(pos, tokens);
    *pos += 1;
    token
}

pub fn peek<'source>(pos: &mut usize, tokens: &[Token<'source>]) -> Token<'source> {
    if let Some(token) = tokens.get(*pos) {
        token.clone()
    } else if let Some(last_token) = tokens.get(*pos - 1) {
        Token::new("", TokenType::End, last_token.loc)
    } else {
        panic!("Input to parser appears to be empty!");
    }
}
