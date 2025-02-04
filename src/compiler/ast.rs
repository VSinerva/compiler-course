use crate::compiler::token::CodeLocation;
use crate::compiler::variable::Type;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TypeExpression {
    Int(CodeLocation),
    Bool(CodeLocation),
}

#[derive(Debug, PartialEq)]
pub struct AstNode<'source> {
    pub loc: CodeLocation,
    pub node_type: Type,
    pub expr: Expression<'source>,
}

impl<'source> AstNode<'source> {
    pub fn new(loc: CodeLocation, expr: Expression<'source>) -> AstNode<'source> {
        AstNode {
            loc,
            expr,
            node_type: Type::Unit,
        }
    }
}

impl<'source> fmt::Display for AstNode<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} at {}",
            self.expr.expr_type_str(),
            self.expr.val_string(),
            self.loc
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression<'source> {
    EmptyLiteral(),
    IntLiteral(i64),
    BoolLiteral(bool),
    Identifier(&'source str),
    UnaryOp(&'source str, Box<AstNode<'source>>),
    BinaryOp(Box<AstNode<'source>>, &'source str, Box<AstNode<'source>>),
    VarDeclaration(&'source str, Box<AstNode<'source>>, Option<TypeExpression>),
    Conditional(
        Box<AstNode<'source>>,
        Box<AstNode<'source>>,
        Option<Box<AstNode<'source>>>,
    ),
    While(Box<AstNode<'source>>, Box<AstNode<'source>>),
    FunCall(&'source str, Vec<AstNode<'source>>),
    Block(Vec<AstNode<'source>>),
}

impl<'source> Expression<'source> {
    fn expr_type_str(&self) -> &str {
        match self {
            Expression::EmptyLiteral(..) => "Empty literal",
            Expression::IntLiteral(..) => "Integer literal",
            Expression::BoolLiteral(..) => "Boolen literal",
            Expression::Identifier(..) => "Identifier",
            Expression::UnaryOp(..) => "Unary operation",
            Expression::VarDeclaration(..) => "Variable declaration",
            Expression::BinaryOp(..) => "Binary operation",
            Expression::Conditional(..) => "Conditional",
            Expression::While(..) => "While loop",
            Expression::FunCall(..) => "Function call",
            Expression::Block(..) => "Block",
        }
    }

    fn val_string(&self) -> String {
        match self {
            Expression::EmptyLiteral(..) => "".to_string(),
            Expression::IntLiteral(val) => val.to_string(),
            Expression::BoolLiteral(val) => val.to_string(),
            Expression::Identifier(name) => name.to_string(),
            Expression::UnaryOp(op, _) => op.to_string(),
            Expression::VarDeclaration(name, _, _) => name.to_string(),
            Expression::BinaryOp(_, op, _) => op.to_string(),
            Expression::Conditional(condition, _, _) => format!("if {:?}", condition),
            Expression::While(condition, _) => format!("while {:?}", condition),
            Expression::FunCall(name, args) => format!("{} with {} args", name, args.len()),
            Expression::Block(expressions) => format!("with {} expressions", expressions.len()),
        }
    }
}
