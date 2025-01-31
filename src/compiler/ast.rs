#[derive(Debug, PartialEq)]
pub enum Expression<'source> {
    EmptyLiteral(),
    IntLiteral(u32),
    BoolLiteral(bool),
    Identifier(&'source str),
    UnaryOp(&'source str, Box<Expression<'source>>),
    VarDeclaration(&'source str, Box<Expression<'source>>),
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
    FunCall(&'source str, Vec<Expression<'source>>),
    Block(Vec<Expression<'source>>),
}
