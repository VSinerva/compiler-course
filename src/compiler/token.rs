#[derive(Debug, Copy, Clone)]
pub struct CodeLocation {
    line: usize,
    char: usize,
}

impl CodeLocation {
    pub fn new(line: usize, char: usize) -> Self {
        Self { line, char }
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
}

impl TokenType {
    pub fn ignore(&self) -> bool {
        use TokenType::*;
        match self {
            Whitespace | Comment => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token<'source> {
    text: &'source str,
    token_type: TokenType,
    loc: CodeLocation,
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
