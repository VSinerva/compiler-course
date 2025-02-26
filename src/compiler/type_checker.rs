use std::{error::Error, fmt::Display};

use crate::compiler::{
    ast::{AstNode, Expression::*, TypeExpression},
    symtab::SymTab,
    variable::Type,
};

#[derive(Debug)]
pub struct TypeCheckerError {
    msg: String,
}

impl Display for TypeCheckerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeCheckerError: {}", self.msg)
    }
}

impl Error for TypeCheckerError {}

pub fn type_check<'source>(
    ast: &mut AstNode<'source>,
    symbols: &mut SymTab<'source, Type>,
) -> Result<Type, Box<dyn Error>> {
    let node_type = get_type(ast, symbols)?;
    ast.node_type = node_type.clone();
    Ok(node_type)
}

fn get_type<'source>(
    ast: &mut AstNode<'source>,
    symbols: &mut SymTab<'source, Type>,
) -> Result<Type, Box<dyn Error>> {
    match &mut ast.expr {
        EmptyLiteral() => Ok(Type::Unit),
        IntLiteral(_) => Ok(Type::Int),
        BoolLiteral(_) => Ok(Type::Bool),
        Identifier(name) => Ok(symbols.get(name)?.clone()),
        UnaryOp(op, ref mut expr) => {
            let expr_types = vec![type_check(expr, symbols)?];

            let Type::Func(sig_arg_types, sig_ret_type) = symbols.get(&format!("unary_{op}"))?
            else {
                return Err(Box::new(TypeCheckerError {
                    msg: format!("Identifier {} does not correspond to an operator!", op),
                }));
            };

            if expr_types != *sig_arg_types {
                return Err(Box::new(TypeCheckerError {
                    msg: format!(
                        "Operator {} argument types {:?} don't match expected {:?}",
                        op, expr_types, *sig_arg_types
                    ),
                }));
            }

            Ok((**sig_ret_type).clone())
        }
        BinaryOp(ref mut left, op, ref mut right) => match *op {
            "==" | "!=" => {
                let left_type = type_check(left, symbols)?;
                let right_type = type_check(right, symbols)?;
                if left_type != right_type {
                    return Err(Box::new(TypeCheckerError {
                        msg: format!("Mismatched types being compared with {op}"),
                    }));
                }
                Ok(Type::Bool)
            }
            "=" => {
                if !matches!(left.expr, Identifier(_)) {
                    return Err(Box::new(TypeCheckerError {
                        msg: String::from("Non-variable on left side of assignment!"),
                    }));
                }

                let left_type = type_check(left, symbols)?;
                let right_type = type_check(right, symbols)?;
                if left_type != right_type {
                    return Err(Box::new(TypeCheckerError {
                        msg: String::from("Mismatched types in assignment!"),
                    }));
                }
                Ok(left_type)
            }
            _ => {
                let left_type = type_check(left, symbols)?;
                let right_type = type_check(right, symbols)?;
                let arg_types = vec![left_type, right_type];

                let Type::Func(sig_arg_types, sig_ret_type) = symbols.get(op).unwrap() else {
                    return Err(Box::new(TypeCheckerError {
                        msg: format!("Identifier {} does not correspond to an operator!", op),
                    }));
                };

                if arg_types != *sig_arg_types {
                    return Err(Box::new(TypeCheckerError {
                        msg: format!(
                            "Operator {} argument types {:?} don't match expected {:?}",
                            op, arg_types, *sig_arg_types
                        ),
                    }));
                }

                Ok((**sig_ret_type).clone())
            }
        },
        VarDeclaration(name, ref mut expr, ref mut type_expr) => {
            let type_var = type_check(expr, symbols)?;

            if let Some(type_expr) = type_expr {
                let expected_type = match type_expr {
                    TypeExpression::Int(_) => Type::Int,
                    TypeExpression::Bool(_) => Type::Bool,
                };

                if type_var != expected_type {
                    return Err(Box::new(TypeCheckerError {
                        msg: format!(
                            "Expected type {:?} does not match actual type {:?} in var declaration",
                            expected_type, type_var
                        ),
                    }));
                }
            }

            symbols.insert(name, type_var)?;
            Ok(Type::Unit)
        }
        Conditional(ref mut condition_expr, ref mut then_expr, ref mut else_expr) => {
            if !matches!(type_check(condition_expr, symbols)?, Type::Bool) {
                return Err(Box::new(TypeCheckerError {
                    msg: String::from("Non-bool as if-then-else condition!"),
                }));
            }

            if let Some(ref mut else_expr) = else_expr {
                let then_type = type_check(then_expr, symbols)?;
                let else_type = type_check(else_expr, symbols)?;
                if then_type == else_type {
                    Ok(then_type)
                } else {
                    Err(Box::new(TypeCheckerError {
                        msg: String::from("Mismatched return types in if-then-else!"),
                    }))
                }
            } else {
                Ok(Type::Unit)
            }
        }
        While(ref mut condition_expr, ref mut do_expr) => {
            if !matches!(type_check(condition_expr, symbols)?, Type::Bool) {
                return Err(Box::new(TypeCheckerError {
                    msg: String::from("Non-bool as while-do condition!"),
                }));
            }
            type_check(do_expr, symbols)?;
            Ok(Type::Unit)
        }
        FunCall(name, args) => {
            let mut arg_types = Vec::new();
            for arg in args {
                arg_types.push(type_check(arg, symbols)?);
            }

            let Type::Func(sig_arg_types, sig_ret_type) = symbols.get(name).unwrap() else {
                return Err(Box::new(TypeCheckerError {
                    msg: format!("Identifier {} does not correspond to a function!", name),
                }));
            };

            if arg_types != *sig_arg_types {
                return Err(Box::new(TypeCheckerError {
                    msg: format!(
                        "Function {} argument types {:?} don't match expected {:?}",
                        name, arg_types, *sig_arg_types
                    ),
                }));
            }

            Ok((**sig_ret_type).clone())
        }
        Block(ref mut expressions) => {
            symbols.push_level();

            let mut type_var = Type::Unit;
            for expression in expressions {
                type_var = type_check(expression, symbols)?;
            }

            symbols.remove_level();
            Ok(type_var)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::{parser::parse, tokenizer::tokenize};
    use Type::*;

    fn get_type(code: &str) -> Type {
        type_check(
            &mut parse(&tokenize(code).unwrap()).unwrap(),
            &mut SymTab::new_type_table(),
        )
        .unwrap()
    }

    #[test]
    fn test_individual() {
        let result = get_type("1");
        assert_eq!(result, Int);

        let result = get_type("true");
        assert_eq!(result, Bool);

        let result = get_type("var a = true; a");
        assert_eq!(result, Bool);

        let result = get_type("var a = true;");
        assert_eq!(result, Unit);
    }

    #[test]
    fn test_var_untyped() {
        let result = get_type("var a = 1");
        assert_eq!(result, Unit);

        let result = get_type("var a = 1; a");
        assert_eq!(result, Int);
    }

    #[test]
    fn test_var_typed() {
        let result = get_type("var a: Int = 1");
        assert_eq!(result, Unit);

        let result = get_type("var a = 1; a");
        assert_eq!(result, Int);
    }

    #[test]
    #[should_panic]
    fn test_var_typed_mismatch() {
        get_type("var a: Int = true");
    }

    #[test]
    fn test_assign() {
        let result = get_type("var a = 1; a = 2;");
        assert_eq!(result, Unit);

        let result = get_type("var a = 1; a = 2");
        assert_eq!(result, Int);
    }

    #[test]
    #[should_panic]
    fn test_assign_mismatch() {
        get_type("var a = 1; a = true");
    }

    #[test]
    #[should_panic]
    fn test_assign_non_var() {
        get_type("1 = 2");
    }

    #[test]
    fn test_operators() {
        let result = get_type("true or false");
        assert_eq!(result, Bool);
        let result = get_type("true and false");
        assert_eq!(result, Bool);
        let result = get_type("true == false");
        assert_eq!(result, Bool);
        let result = get_type("1 == 2");
        assert_eq!(result, Bool);
        let result = get_type("true != false");
        assert_eq!(result, Bool);
        let result = get_type("1 != 2");
        assert_eq!(result, Bool);
        let result = get_type("1 < 2");
        assert_eq!(result, Bool);
        let result = get_type("1 <= 2");
        assert_eq!(result, Bool);
        let result = get_type("1 > 2");
        assert_eq!(result, Bool);
        let result = get_type("1 >= 2");
        assert_eq!(result, Bool);
        let result = get_type("1 + 2");
        assert_eq!(result, Int);
        let result = get_type("1 - 2");
        assert_eq!(result, Int);
        let result = get_type("1 * 2");
        assert_eq!(result, Int);
        let result = get_type("1 / 2");
        assert_eq!(result, Int);
        let result = get_type("1 % 2");
        assert_eq!(result, Int);
        let result = get_type("not false");
        assert_eq!(result, Bool);
        let result = get_type("-1");
        assert_eq!(result, Int);
    }

    #[test]
    #[should_panic]
    fn test_operators_mismatch() {
        get_type("1 == true");
    }

    #[test]
    #[should_panic]
    fn test_operators_wrong_type() {
        get_type("1 and 2");
    }

    #[test]
    fn test_conditional() {
        let result = get_type("if true then 1");
        assert_eq!(result, Unit);

        let result = get_type("if true then 1 else 2");
        assert_eq!(result, Int);
    }

    #[test]
    #[should_panic]
    fn test_conditional_non_bool() {
        get_type("if 1 then 2");
    }

    #[test]
    #[should_panic]
    fn test_conditional_type_mismatch() {
        get_type("if true then 2 else false");
    }

    #[test]
    fn test_while() {
        let result = get_type("while true do 1");
        assert_eq!(result, Unit);
    }

    #[test]
    #[should_panic]
    fn test_while_non_bool() {
        get_type("while 1 do 2");
    }

    #[test]
    fn test_block() {
        let result = get_type("{1; 2}");
        assert_eq!(result, Int);

        let result = get_type("{1; 2;}");
        assert_eq!(result, Unit);
    }

    #[test]
    fn test_function() {
        let mut tokens = tokenize("foo(1)").unwrap();
        let mut ast = parse(&tokens).unwrap();
        let mut symtab = SymTab::new_type_table();
        symtab
            .insert("foo", Func(vec![Int], Box::new(Int)))
            .unwrap();
        let result = type_check(&mut ast, &mut symtab).unwrap();
        assert_eq!(result, Int);

        tokens = tokenize("foo(1);").unwrap();
        ast = parse(&tokens).unwrap();
        symtab = SymTab::new_type_table();
        symtab
            .insert("foo", Func(vec![Int], Box::new(Int)))
            .unwrap();
        let result = type_check(&mut ast, &mut symtab).unwrap();
        assert_eq!(result, Unit);
    }

    #[test]
    #[should_panic]
    fn test_function_wrong_arg() {
        let tokens = tokenize("foo(true)").unwrap();
        let mut ast = parse(&tokens).unwrap();
        let mut symtab = SymTab::new_type_table();
        symtab
            .insert("foo", Func(vec![Int], Box::new(Int)))
            .unwrap();
        type_check(&mut ast, &mut symtab).unwrap();
    }

    #[test]
    fn test_node_type() {
        let tokens = tokenize("1").unwrap();
        let mut ast = parse(&tokens).unwrap();
        let mut symtab = SymTab::new_type_table();

        assert_eq!(ast.node_type, Unit);
        type_check(&mut ast, &mut symtab).unwrap();
        assert_eq!(ast.node_type, Int);
    }
}
