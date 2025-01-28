#[derive(Debug, PartialEq)]
pub enum Expression<'source> {
    IntLiteral(u32),
    BoolLiteral(bool),
    Identifier(&'source str),
    BinaryOp(
        Box<Expression<'source>>,
        &'source str,
        Box<Expression<'source>>,
    ),
    Conditional(
        Box<Expression<'source>>,
        Box<Expression<'source>>,
        Option<Box<Expression<'source>>>,
    ),
}
