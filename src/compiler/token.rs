use std::fmt;

#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct CodeLocation {
    line: usize,
    char: usize,
}

impl CodeLocation {
    pub fn new(line: usize, char: usize) -> Self {
        Self { line, char }
    }
}

impl fmt::Display for CodeLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.char)
    }
}

impl PartialEq for CodeLocation {
    fn eq(&self, other: &Self) -> bool {
        let true_match = self.line == other.line && self.char == other.char;

        // For testing purposes
        let simulated_match = self.line == usize::MAX
            || self.char == usize::MAX
            || other.line == usize::MAX
            || other.char == usize::MAX;

        true_match || simulated_match
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Comment,
    Integer,
    Identifier,
    Operator,
    Punctuation,
    Whitespace,
    End,
}

impl TokenType {
    pub fn ignore(&self) -> bool {
        use TokenType::*;
        matches!(self, Whitespace | Comment)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token<'source> {
    pub text: &'source str,
    pub token_type: TokenType,
    pub loc: CodeLocation,
}

impl<'source> Token<'source> {
    pub fn new(text: &'source str, token_type: TokenType, loc: CodeLocation) -> Self {
        Self {
            text,
            token_type,
            loc,
        }
    }
}

impl<'source> fmt::Display for Token<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ``{}`` at {}", self.token_type, self.text, self.loc)
    }
}
