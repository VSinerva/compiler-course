#[derive(Debug, Copy, Clone)]
pub struct CodeLocation {
    start: usize,
    end: usize,
}

impl CodeLocation {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl PartialEq for CodeLocation {
    fn eq(&self, other: &Self) -> bool {
        let true_match = self.start == other.start && self.end == other.end;

        // For testing purposes
        let simulated_match = self.start == usize::MAX
            || self.end == usize::MAX
            || other.start == usize::MAX
            || other.end == usize::MAX;

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
pub struct Token {
    text: String,
    token_type: TokenType,
    loc: CodeLocation,
}

impl Token {
    pub fn new(text: &str, token_type: TokenType, loc: CodeLocation) -> Self {
        Self {
            text: text.to_string(),
            token_type,
            loc,
        }
    }
}
