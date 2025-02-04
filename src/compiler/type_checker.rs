use crate::compiler::{
    ast::Expression::{self, *},
    symtab::SymTab,
    variable::Type,
};

pub fn type_check<'source>(ast: &Expression<'source>, symbols: &mut SymTab<'source, Type>) -> Type {
    match ast {
        EmptyLiteral(_) => Type::Unit,
        IntLiteral(_, _) => Type::Int,
        BoolLiteral(_, _) => Type::Bool,
        Identifier(_, _) => todo!(),
        UnaryOp(_, _, _) => todo!(),
        BinaryOp(_, _, _, _) => todo!(),
        VarDeclaration(_, _, _) => Type::Unit,
        Conditional(_, _, _, _) => todo!(),
        While(_, _, _) => todo!(),
        FunCall(_, _, _) => todo!(),
        Block(_, _) => todo!(),
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

        let result = get_type("+");
        assert_eq!(result, Func(vec![Int, Int], Box::new(Int)));
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
