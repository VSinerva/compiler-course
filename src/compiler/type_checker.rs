use crate::compiler::{
    ast::{
        Expression::{self, *},
        TypeExpression,
    },
    symtab::SymTab,
    variable::Type,
};

pub fn type_check<'source>(ast: &Expression<'source>, symbols: &mut SymTab<'source, Type>) -> Type {
    match ast {
        EmptyLiteral(_) => Type::Unit,
        IntLiteral(_, _) => Type::Int,
        BoolLiteral(_, _) => Type::Bool,
        Identifier(_, name) => symbols.get(name).clone(),
        UnaryOp(_, op, expr) => match *op {
            "-" => {
                let expr_types = vec![type_check(expr, symbols)];

                let Type::Func(sig_arg_types, sig_ret_type) = symbols.get("neg") else {
                    panic!("Identifier {} does not correspond to an operator!", op);
                };

                if expr_types != *sig_arg_types {
                    panic!(
                        "Operator {} argument types {:?} don't match expected {:?}",
                        op, expr_types, *sig_arg_types
                    );
                }

                (**sig_ret_type).clone()
            }
            _ => {
                let expr_types = vec![type_check(expr, symbols)];

                let Type::Func(sig_arg_types, sig_ret_type) = symbols.get(op) else {
                    panic!("Identifier {} does not correspond to an operator!", op);
                };

                if expr_types != *sig_arg_types {
                    panic!(
                        "Operator {} argument types {:?} don't match expected {:?}",
                        op, expr_types, *sig_arg_types
                    );
                }

                (**sig_ret_type).clone()
            }
        },
        BinaryOp(_, left, op, right) => match *op {
            "==" | "!=" => {
                let left_type = type_check(left, symbols);
                let right_type = type_check(right, symbols);
                if left_type != right_type {
                    panic!("Mismatched types being compared with {op}");
                }
                Type::Bool
            }
            "=" => {
                if !matches!(**left, Identifier(_, _)) {
                    panic!("Non-variable on left side of assignment!");
                }

                let left_type = type_check(left, symbols);
                let right_type = type_check(right, symbols);
                if left_type != right_type {
                    panic!("Mismatched types in assignment!");
                }
                left_type
            }
            _ => {
                let left_type = type_check(left, symbols);
                let right_type = type_check(right, symbols);
                let arg_types = vec![left_type, right_type];

                let Type::Func(sig_arg_types, sig_ret_type) = symbols.get(op) else {
                    panic!("Identifier {} does not correspond to an operator!", op);
                };

                if arg_types != *sig_arg_types {
                    panic!(
                        "Operator {} argument types {:?} don't match expected {:?}",
                        op, arg_types, *sig_arg_types
                    );
                }

                (**sig_ret_type).clone()
            }
        },
        VarDeclaration(_, name, expr, type_expr) => {
            let type_var = type_check(expr, symbols);

            if let Some(type_expr) = type_expr {
                let expected_type = match type_expr {
                    TypeExpression::Int(_) => Type::Int,
                    TypeExpression::Bool(_) => Type::Bool,
                };

                if type_var != expected_type {
                    panic!(
                        "Expected type {:?} does not match actual type {:?} in var declaration",
                        expected_type, type_var
                    )
                }
            }

            symbols.insert(name, type_var);
            Type::Unit
        }
        Conditional(_, condition_expr, then_expr, else_expr) => {
            if !matches!(type_check(condition_expr, symbols), Type::Bool) {
                panic!("Non-bool as if-then-else condition!");
            }

            if let Some(else_expr) = else_expr {
                let then_type = type_check(then_expr, symbols);
                let else_type = type_check(else_expr, symbols);
                if then_type == else_type {
                    then_type
                } else {
                    panic!("Mismatched return values in if-then-else!");
                }
            } else {
                Type::Unit
            }
        }
        While(_, condition_expr, do_expr) => {
            if !matches!(type_check(condition_expr, symbols), Type::Bool) {
                panic!("Non-bool as while-do condition!");
            }
            type_check(do_expr, symbols);
            Type::Unit
        }
        FunCall(_, name, args) => {
            let mut arg_types = Vec::new();
            for arg in args {
                arg_types.push(type_check(arg, symbols));
            }

            let Type::Func(sig_arg_types, sig_ret_type) = symbols.get(name) else {
                panic!("Identifier {} does not correspond to a function!", name);
            };

            if arg_types != *sig_arg_types {
                panic!(
                    "Function {} argument types {:?} don't match expected {:?}",
                    name, arg_types, *sig_arg_types
                );
            }

            (**sig_ret_type).clone()
        }
        Block(_, expressions) => {
            symbols.push_level();

            let mut type_var = Type::Unit;
            for expression in expressions {
                type_var = type_check(expression, symbols);
            }

            symbols.remove_level();
            type_var
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::{parser::parse, tokenizer::tokenize};
    use Type::*;

    fn get_type(code: &str) -> Type {
        type_check(&parse(&tokenize(code)), &mut SymTab::new_type_table())
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
        let mut tokens = tokenize("foo(1)");
        let mut ast = parse(&tokens);
        let mut symtab = SymTab::new_type_table();
        symtab.insert("foo", Func(vec![Int], Box::new(Int)));
        let result = type_check(&ast, &mut symtab);
        assert_eq!(result, Int);

        tokens = tokenize("foo(1);");
        ast = parse(&tokens);
        symtab = SymTab::new_type_table();
        symtab.insert("foo", Func(vec![Int], Box::new(Int)));
        let result = type_check(&ast, &mut symtab);
        assert_eq!(result, Unit);
    }

    #[test]
    #[should_panic]
    fn test_function_wrong_arg() {
        let tokens = tokenize("foo(true)");
        let ast = parse(&tokens);
        let mut symtab = SymTab::new_type_table();
        symtab.insert("foo", Func(vec![Int], Box::new(Int)));
        type_check(&ast, &mut symtab);
    }
}
