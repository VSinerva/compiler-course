use super::*;
use crate::compiler::tokenizer::tokenize;

macro_rules! int_ast {
    ($x:expr) => {
        Box::new(IntLiteral($x))
    };
}

macro_rules! id_ast {
    ($x:expr) => {
        Box::new(Identifier($x))
    };
}

macro_rules! un_ast {
    ($x:expr, $y:expr) => {
        Box::new(UnaryOp($x, $y))
    };
}

macro_rules! bin_ast {
    ($x:expr, $y:expr, $z:expr) => {
        Box::new(BinaryOp($x, $y, $z))
    };
}

macro_rules! bool_ast {
    ($x:expr) => {
        Box::new(BoolLiteral($x))
    };
}

#[test]
#[should_panic]
fn test_empty() {
    parse(&vec![]);
}

#[test]
#[should_panic]
fn test_invalid_start() {
    parse(&tokenize("1 2 + 3"));
}

#[test]
#[should_panic]
fn test_invalid_middle() {
    parse(&tokenize("1 + 2 2 + 3"));
}

#[test]
#[should_panic]
fn test_invalid_end() {
    parse(&tokenize("1 + 2 3"));
}

#[test]
fn test_binary_op_basic() {
    let result = parse(&tokenize("1 + 23"));
    assert_eq!(result, BinaryOp(int_ast!(1), "+", int_ast!(23)));

    let result = parse(&tokenize("4 - 56"));
    assert_eq!(result, BinaryOp(int_ast!(4), "-", int_ast!(56)));

    let result = parse(&tokenize("1 * 2"));
    assert_eq!(result, BinaryOp(int_ast!(1), "*", int_ast!(2)));

    let result = parse(&tokenize("1 / 2"));
    assert_eq!(result, BinaryOp(int_ast!(1), "/", int_ast!(2)));
}

#[test]
fn test_binary_op_all_levels() {
    let result = parse(&tokenize("1 * 2 + 3 < 4 == 5 and 6 or 7"));
    assert_eq!(
        result,
        BinaryOp(
            bin_ast!(
                bin_ast!(
                    bin_ast!(
                        bin_ast!(bin_ast!(int_ast!(1), "*", int_ast!(2)), "+", int_ast!(3)),
                        "<",
                        int_ast!(4)
                    ),
                    "==",
                    int_ast!(5)
                ),
                "and",
                int_ast!(6)
            ),
            "or",
            int_ast!(7)
        )
    );
}

#[test]
fn test_binary_op_identifier() {
    let result = parse(&tokenize("a + 1"));
    assert_eq!(result, BinaryOp(id_ast!("a"), "+", int_ast!(1)));

    let result = parse(&tokenize("1 - a"));
    assert_eq!(result, BinaryOp(int_ast!(1), "-", id_ast!("a")));
}

#[test]
fn test_binary_op_multiple() {
    let result = parse(&tokenize("1 + 2 - 3"));
    assert_eq!(
        result,
        BinaryOp(bin_ast!(int_ast!(1), "+", int_ast!(2)), "-", int_ast!(3))
    );
}

#[test]
fn test_binary_op_precedence() {
    let result = parse(&tokenize("1 + 2 * 3"));
    assert_eq!(
        result,
        BinaryOp(int_ast!(1), "+", bin_ast!(int_ast!(2), "*", int_ast!(3)),)
    );

    let result = parse(&tokenize("1 - 2 / 3"));
    assert_eq!(
        result,
        BinaryOp(int_ast!(1), "-", bin_ast!(int_ast!(2), "/", int_ast!(3)),)
    );
}

#[test]
fn test_assignment_basic() {
    let result = parse(&tokenize("a = 1 + 2"));
    assert_eq!(
        result,
        BinaryOp(id_ast!("a"), "=", bin_ast!(int_ast!(1), "+", int_ast!(2)))
    );
}

#[test]
fn test_assignment_chain() {
    let result = parse(&tokenize("a = b = 1 + 2"));
    assert_eq!(
        result,
        BinaryOp(
            id_ast!("a"),
            "=",
            bin_ast!(id_ast!("b"), "=", bin_ast!(int_ast!(1), "+", int_ast!(2)))
        )
    );
}

#[test]
#[should_panic]
fn test_assignment_invalid() {
    parse(&tokenize("a ="));
}

#[test]
fn test_unary_basic() {
    let result = parse(&tokenize("not x"));
    assert_eq!(result, UnaryOp("not", id_ast!("x")));

    let result = parse(&tokenize("-x"));
    assert_eq!(result, UnaryOp("-", id_ast!("x")));

    let result = parse(&tokenize("-1"));
    assert_eq!(result, UnaryOp("-", int_ast!(1)));

    let result = parse(&tokenize("-1 + 2"));
    assert_eq!(
        result,
        BinaryOp(un_ast!("-", int_ast!(1)), "+", int_ast!(2))
    );
}

#[test]
fn test_unary_chain() {
    let result = parse(&tokenize("not not x"));
    assert_eq!(result, UnaryOp("not", un_ast!("not", id_ast!("x"))));

    let result = parse(&tokenize("--x"));
    assert_eq!(result, UnaryOp("-", un_ast!("-", id_ast!("x"))));

    let result = parse(&tokenize("--1"));
    assert_eq!(result, UnaryOp("-", un_ast!("-", int_ast!(1))));

    let result = parse(&tokenize("--1 + 2"));
    assert_eq!(
        result,
        BinaryOp(un_ast!("-", un_ast!("-", int_ast!(1))), "+", int_ast!(2))
    );
}

#[test]
fn test_parenthesized() {
    let result = parse(&tokenize("(1+2)*3"));
    assert_eq!(
        result,
        BinaryOp(bin_ast!(int_ast!(1), "+", int_ast!(2)), "*", int_ast!(3),)
    );
}

#[test]
fn test_parenthesized_nested() {
    let result = parse(&tokenize("((1 - 2))/3"));
    assert_eq!(
        result,
        BinaryOp(bin_ast!(int_ast!(1), "-", int_ast!(2)), "/", int_ast!(3),)
    );

    let result = parse(&tokenize("((1 + 2)*3) / 4"));
    assert_eq!(
        result,
        BinaryOp(
            bin_ast!(bin_ast!(int_ast!(1), "+", int_ast!(2)), "*", int_ast!(3)),
            "/",
            int_ast!(4)
        )
    );
}

#[test]
#[should_panic]
fn test_parenthesized_mismatched() {
    parse(&tokenize("(1+2*3"));
}

#[test]
fn test_if_then() {
    let result = parse(&tokenize("if 1 + 2 then 3"));
    assert_eq!(
        result,
        Conditional(bin_ast!(int_ast!(1), "+", int_ast!(2)), int_ast!(3), None,)
    );
}

#[test]
fn test_if_then_else() {
    let result = parse(&tokenize("if a then b + c else 1 * 2"));
    assert_eq!(
        result,
        Conditional(
            id_ast!("a"),
            bin_ast!(id_ast!("b"), "+", id_ast!("c")),
            Some(bin_ast!(int_ast!(1), "*", int_ast!(2)))
        )
    );
}

#[test]
fn test_if_then_else_embedded() {
    let result = parse(&tokenize("1 + if true then 2 else 3"));
    assert_eq!(
        result,
        BinaryOp(
            int_ast!(1),
            "+",
            Box::new(Conditional(bool_ast!(true), int_ast!(2), Some(int_ast!(3))))
        )
    );
}

#[test]
fn test_if_then_else_nested() {
    let result = parse(&tokenize("if true then if false then 1 else 2 else 3"));
    assert_eq!(
        result,
        Conditional(
            bool_ast!(true),
            Box::new(Conditional(
                bool_ast!(false),
                int_ast!(1),
                Some(int_ast!(2))
            )),
            Some(int_ast!(3))
        )
    );
}

#[test]
#[should_panic]
fn test_if_no_then() {
    parse(&tokenize("if true"));
}

#[test]
fn test_func_basic() {
    let result = parse(&tokenize("f(a, b)"));
    assert_eq!(
        result,
        FunCall("f", vec![Identifier("a"), Identifier("b"),])
    );

    let result = parse(&tokenize("f(a, 1 + 2)"));
    assert_eq!(
        result,
        FunCall(
            "f",
            vec![Identifier("a"), BinaryOp(int_ast!(1), "+", int_ast!(2),),]
        )
    );

    let result = parse(&tokenize("f()"));
    assert_eq!(result, FunCall("f", vec![]));
}

#[test]
fn test_func_embedded() {
    let result = parse(&tokenize("1 + f(a)"));
    assert_eq!(
        result,
        BinaryOp(
            int_ast!(1),
            "+",
            Box::new(FunCall("f", vec![Identifier("a")]))
        )
    );
}

#[test]
fn test_func_nested() {
    let result = parse(&tokenize("f(a, g(b))"));
    assert_eq!(
        result,
        FunCall(
            "f",
            vec![Identifier("a"), FunCall("g", vec![Identifier("b")]),]
        )
    );
}

#[test]
#[should_panic]
fn test_func_missing_comma() {
    parse(&tokenize("f(a b)"));
}

#[test]
#[should_panic]
fn test_func_missing_close() {
    parse(&tokenize("f(a"));
}
