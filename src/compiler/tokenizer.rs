use crate::compiler::token::{CodeLocation, Token, TokenType};
use regex::Regex;

pub fn tokenize(code: &str) -> Vec<Token> {
    // We only want to compile the regexes once
    // The ordering of these is important!
    let regexes = vec![
        (TokenType::Comment, Regex::new(r"^(\\\\|#).*\n").unwrap()),
        (TokenType::Whitespace, Regex::new(r"^[\s\t\n]+").unwrap()),
        (
            TokenType::Operator,
            Regex::new(r"^(==|!=|<=|>=|=|<|>|\+|-|\*|/)").unwrap(),
        ),
        (TokenType::Punctuation, Regex::new(r"^[\(\){},;]").unwrap()),
        (TokenType::Integer, Regex::new(r"^[0-9]+").unwrap()),
        (
            TokenType::Identifier,
            Regex::new(r"^[[:alpha:]_][[:alpha:]0-9_]*").unwrap(),
        ),
    ];

    let mut tokens = Vec::new();

    let mut pos = 0;

    while pos < code.len() {
        let mut valid_token = false;

        for (token_type, regex_matcher) in &regexes {
            let found_match = regex_matcher.find(&code[pos..]);

            if let Some(token) = found_match {
                if !token_type.ignore() {
                    tokens.push(Token::new(
                        &code[pos + token.start()..pos + token.end()],
                        *token_type,
                        CodeLocation::new(0, 0),
                    ));
                }

                valid_token = true;
                pos += token.end();
            }
        }

        if !valid_token {
            panic!("Invalid token at {pos}");
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic() {
        let loc = CodeLocation::new(-1, -1);
        let result = tokenize("if   3 \n\twhile");

        use TokenType::*;
        assert_eq!(
            result,
            vec!(
                Token::new("if", Identifier, loc),
                Token::new("3", Integer, loc),
                Token::new("while", Identifier, loc),
            )
        );
    }

    #[test]
    fn test_tokenize_comment() {
        let loc = CodeLocation::new(-1, -1);
        let result = tokenize("if   3 \n\n\\\\Comment\n#Another\n\twhile");

        use TokenType::*;
        assert_eq!(
            result,
            vec!(
                Token::new("if", Identifier, loc),
                Token::new("3", Integer, loc),
                Token::new("while", Identifier, loc),
            )
        );
    }

    #[test]
    fn test_tokenize_operators_basic() {
        let loc = CodeLocation::new(-1, -1);
        let result = tokenize("var = 1 + 2");

        use TokenType::*;
        assert_eq!(
            result,
            vec!(
                Token::new("var", Identifier, loc),
                Token::new("=", Operator, loc),
                Token::new("1", Integer, loc),
                Token::new("+", Operator, loc),
                Token::new("2", Integer, loc),
            )
        );
    }

    #[test]
    fn test_tokenize_operators_all() {
        let loc = CodeLocation::new(-1, -1);
        let result = tokenize("var 1 + - * 1/2 = == != < <= > >= 2");

        use TokenType::*;
        assert_eq!(
            result,
            vec!(
                Token::new("var", Identifier, loc),
                Token::new("1", Integer, loc),
                Token::new("+", Operator, loc),
                Token::new("-", Operator, loc),
                Token::new("*", Operator, loc),
                Token::new("1", Integer, loc),
                Token::new("/", Operator, loc),
                Token::new("2", Integer, loc),
                Token::new("=", Operator, loc),
                Token::new("==", Operator, loc),
                Token::new("!=", Operator, loc),
                Token::new("<", Operator, loc),
                Token::new("<=", Operator, loc),
                Token::new(">", Operator, loc),
                Token::new(">=", Operator, loc),
                Token::new("2", Integer, loc),
            )
        );
    }

    #[test]
    fn test_tokenize_punctuation_basic() {
        let loc = CodeLocation::new(-1, -1);
        let result = tokenize("{var = (1 + 2, 3);}");

        use TokenType::*;
        assert_eq!(
            result,
            vec!(
                Token::new("{", Punctuation, loc),
                Token::new("var", Identifier, loc),
                Token::new("=", Operator, loc),
                Token::new("(", Punctuation, loc),
                Token::new("1", Integer, loc),
                Token::new("+", Operator, loc),
                Token::new("2", Integer, loc),
                Token::new(",", Punctuation, loc),
                Token::new("3", Integer, loc),
                Token::new(")", Punctuation, loc),
                Token::new(";", Punctuation, loc),
                Token::new("}", Punctuation, loc),
            )
        );
    }
}
