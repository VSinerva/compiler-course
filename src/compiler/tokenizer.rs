use std::{error::Error, fmt::Display};

use crate::compiler::token::{CodeLocation, Token, TokenType};
use regex::Regex;

#[derive(Debug)]
pub struct TokenizerError {
    msg: String,
}

impl Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TokenizerError: {}", self.msg)
    }
}

impl Error for TokenizerError {}

pub fn tokenize(code: &str) -> Result<Vec<Token>, TokenizerError> {
    // We only want to compile the regexes once
    // The ordering of these is important!
    let regexes = vec![
        (TokenType::Comment, Regex::new(r"^(//|#).*").unwrap()),
        (TokenType::Whitespace, Regex::new(r"^[\s\t\n]+").unwrap()),
        (
            TokenType::Operator,
            Regex::new(r"^(==|!=|<=|>=|=|<|>|\+|-|\*|/|\%)").unwrap(),
        ),
        (TokenType::Punctuation, Regex::new(r"^[\(\){},;:]").unwrap()),
        (TokenType::Integer, Regex::new(r"^[0-9]+").unwrap()),
        (
            TokenType::Identifier,
            Regex::new(r"^[[:alpha:]_][[:alpha:]0-9_]*").unwrap(),
        ),
    ];

    let mut tokens = Vec::new();

    for (line_number, line) in code.lines().enumerate() {
        let mut pos = 0;

        while pos < line.len() {
            let mut valid_token = false;

            for (token_type, regex_matcher) in &regexes {
                let found_match = regex_matcher.find(&line[pos..]);

                if let Some(token) = found_match {
                    if !token_type.ignore() {
                        let start = pos + token.start();
                        let end = pos + token.end();
                        tokens.push(Token::new(
                            &line[start..end],
                            *token_type,
                            CodeLocation::new(line_number + 1, start + 1), // 1-indexing
                        ));
                    }

                    valid_token = true;
                    pos += token.end();
                    break;
                }
            }

            if !valid_token {
                return Err(TokenizerError {
                    msg: format!(
                        "Invalid token starting with '{}' on line {} in position {}",
                        &line[pos..pos + 1],
                        line_number + 1,
                        pos + 1
                    ),
                });
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tokenize_basic() {
        let loc = CodeLocation::new(usize::MAX, usize::MAX);
        let result = tokenize("if   3 \n\twhile").unwrap();

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
    fn test_tokenize_code_location() {
        let result = tokenize("if 3\n  while").unwrap();

        use TokenType::*;
        assert_eq!(
            result,
            vec!(
                Token::new("if", Identifier, CodeLocation::new(1, 1)),
                Token::new("3", Integer, CodeLocation::new(1, 4)),
                Token::new("while", Identifier, CodeLocation::new(2, 3)),
            )
        );
        assert_ne!(
            result,
            vec!(
                Token::new("if", Identifier, CodeLocation::new(1, 1)),
                Token::new("3", Integer, CodeLocation::new(1, 5)),
                Token::new("while", Identifier, CodeLocation::new(2, 3)),
            )
        );
        assert_ne!(
            result,
            vec!(
                Token::new("if", Identifier, CodeLocation::new(1, 1)),
                Token::new("3", Integer, CodeLocation::new(1, 4)),
                Token::new("while", Identifier, CodeLocation::new(1, 3)),
            )
        );
    }

    #[test]
    fn test_tokenize_comment() {
        let loc = CodeLocation::new(usize::MAX, usize::MAX);
        let result = tokenize("if   3 \n\n//Comment\n#Another\n\twhile //Comment2").unwrap();

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
        let loc = CodeLocation::new(usize::MAX, usize::MAX);
        let result = tokenize("var = 1 + 2").unwrap();

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
        let loc = CodeLocation::new(usize::MAX, usize::MAX);
        let result = tokenize("var 1 + - * 1/2 = == != < <= > >= 2 %").unwrap();

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
                Token::new("%", Operator, loc),
            )
        );
    }

    #[test]
    fn test_tokenize_punctuation_basic() {
        let loc = CodeLocation::new(usize::MAX, usize::MAX);
        let result = tokenize("{var = (1 + 2, 3);:}").unwrap();

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
                Token::new(":", Punctuation, loc),
                Token::new("}", Punctuation, loc),
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_tokenize_wrong_token() {
        tokenize("if 3\n  while @").unwrap();
    }
}
