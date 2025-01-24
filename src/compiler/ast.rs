#[expect(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Expression<'source> {
    IntLiteral(u32),
    BoolLiteral(bool),
    Indentifier(String),
    BinaryOp(
        Box<Expression<'source>>,
        &'source str,
        Box<Expression<'source>>,
    ),
}
