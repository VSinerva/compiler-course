use crate::compiler::token::CodeLocation;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TypeExpression {
    Int(CodeLocation),
    Bool(CodeLocation),
}

#[derive(Debug, PartialEq)]
pub enum Expression<'source> {
    EmptyLiteral(CodeLocation),
    IntLiteral(CodeLocation, i64),
    BoolLiteral(CodeLocation, bool),
    Identifier(CodeLocation, &'source str),
    UnaryOp(CodeLocation, &'source str, Box<Expression<'source>>),
    BinaryOp(
        CodeLocation,
        Box<Expression<'source>>,
        &'source str,
        Box<Expression<'source>>,
    ),
    VarDeclaration(
        CodeLocation,
        &'source str,
        Box<Expression<'source>>,
        Option<TypeExpression>,
    ),
    Conditional(
        CodeLocation,
        Box<Expression<'source>>,
        Box<Expression<'source>>,
        Option<Box<Expression<'source>>>,
    ),
    While(
        CodeLocation,
        Box<Expression<'source>>,
        Box<Expression<'source>>,
    ),
    FunCall(CodeLocation, &'source str, Vec<Expression<'source>>),
    Block(CodeLocation, Vec<Expression<'source>>),
}

impl<'source> Expression<'source> {
    pub fn loc(&self) -> CodeLocation {
        match self {
            Expression::EmptyLiteral(loc) => *loc,
            Expression::IntLiteral(loc, _) => *loc,
            Expression::BoolLiteral(loc, _) => *loc,
            Expression::Identifier(loc, _) => *loc,
            Expression::UnaryOp(loc, _, _) => *loc,
            Expression::VarDeclaration(loc, _, _, _) => *loc,
            Expression::BinaryOp(loc, _, _, _) => *loc,
            Expression::Conditional(loc, _, _, _) => *loc,
            Expression::While(loc, _, _) => *loc,
            Expression::FunCall(loc, _, _) => *loc,
            Expression::Block(loc, _) => *loc,
        }
    }

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
            Expression::While(_, _, _) => "While loop",
            Expression::FunCall(..) => "Function call",
            Expression::Block(..) => "Block",
        }
    }

    fn val_string(&self) -> String {
        match self {
            Expression::EmptyLiteral(..) => "".to_string(),
            Expression::IntLiteral(_, val) => val.to_string(),
            Expression::BoolLiteral(_, val) => val.to_string(),
            Expression::Identifier(_, name) => name.to_string(),
            Expression::UnaryOp(_, op, _) => op.to_string(),
            Expression::VarDeclaration(_, name, _, _) => name.to_string(),
            Expression::BinaryOp(_, _, op, _) => op.to_string(),
            Expression::Conditional(_, condition, _, _) => format!("if {}", condition),
            Expression::While(_, condition, _) => format!("while {}", condition),
            Expression::FunCall(_, name, args) => format!("{} with {} args", name, args.len()),
            Expression::Block(_, expressions) => format!("with {} expressions", expressions.len()),
        }
    }
}

impl<'source> fmt::Display for Expression<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} at {}",
            self.expr_type_str(),
            self.val_string(),
            self.loc()
        )
    }
}
