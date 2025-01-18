#[derive(Debug, Copy, Clone)]
pub struct CodeLocation {
    row: i32,
    col: i32,
}

impl CodeLocation {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

impl PartialEq for CodeLocation {
    fn eq(&self, other: &Self) -> bool {
        let true_match = self.row == other.row && self.col == other.col;

        // For testing purposes
        let simulated_match = self.row < 0 || self.col < 0 || other.row < 0 || other.col < 0;

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
