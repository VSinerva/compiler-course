use super::*;
use crate::compiler::token::CodeLocation;

fn int_tok(text: &str) -> Token {
    Token::new(
        text,
        TokenType::Integer,
        CodeLocation::new(usize::MAX, usize::MAX),
    )
}

macro_rules! int_ast {
    ($x:expr) => {
        Box::new(IntLiteral($x))
    };
}

fn id_tok(text: &str) -> Token {
    Token::new(
        text,
        TokenType::Identifier,
        CodeLocation::new(usize::MAX, usize::MAX),
    )
}

macro_rules! id_ast {
    ($x:expr) => {
        Box::new(Identifier($x))
    };
}

fn punc_tok(text: &str) -> Token {
    Token::new(
        text,
        TokenType::Punctuation,
        CodeLocation::new(usize::MAX, usize::MAX),
    )
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
fn test_binary_op_basic() {
    let result = parse(&vec![int_tok("1"), id_tok("+"), int_tok("23")]);
    assert_eq!(result, BinaryOp(int_ast!(1), "+", int_ast!(23)));

    let result = parse(&vec![int_tok("4"), id_tok("-"), int_tok("56")]);
    assert_eq!(result, BinaryOp(int_ast!(4), "-", int_ast!(56)));

    let result = parse(&vec![int_tok("1"), id_tok("*"), int_tok("2")]);
    assert_eq!(result, BinaryOp(int_ast!(1), "*", int_ast!(2)));

    let result = parse(&vec![int_tok("1"), id_tok("/"), int_tok("2")]);
    assert_eq!(result, BinaryOp(int_ast!(1), "/", int_ast!(2)));
}

#[test]
fn test_binary_op_identifier() {
    let result = parse(&vec![id_tok("a"), id_tok("+"), int_tok("1")]);
    assert_eq!(result, BinaryOp(id_ast!("a"), "+", int_ast!(1)));

    let result = parse(&vec![int_tok("1"), id_tok("-"), id_tok("a")]);
    assert_eq!(result, BinaryOp(int_ast!(1), "-", id_ast!("a")));
}

#[test]
fn test_binary_op_multiple() {
    let result = parse(&vec![
        int_tok("1"),
        id_tok("+"),
        int_tok("2"),
        id_tok("-"),
        int_tok("3"),
    ]);
    assert_eq!(
        result,
        BinaryOp(bin_ast!(int_ast!(1), "+", int_ast!(2)), "-", int_ast!(3))
    );
}

#[test]
fn test_binary_op_precedence() {
    let result = parse(&vec![
        int_tok("1"),
        id_tok("+"),
        int_tok("2"),
        id_tok("*"),
        int_tok("3"),
    ]);
    assert_eq!(
        result,
        BinaryOp(int_ast!(1), "+", bin_ast!(int_ast!(2), "*", int_ast!(3)),)
    );

    let result = parse(&vec![
        int_tok("1"),
        id_tok("-"),
        int_tok("2"),
        id_tok("/"),
        int_tok("3"),
    ]);
    assert_eq!(
        result,
        BinaryOp(int_ast!(1), "-", bin_ast!(int_ast!(2), "/", int_ast!(3)),)
    );
}

#[test]
fn test_parenthesized() {
    let result = parse(&vec![
        id_tok("("),
        int_tok("1"),
        id_tok("+"),
        int_tok("2"),
        id_tok(")"),
        id_tok("*"),
        int_tok("3"),
    ]);
    assert_eq!(
        result,
        BinaryOp(bin_ast!(int_ast!(1), "+", int_ast!(2)), "*", int_ast!(3),)
    );

    let result = parse(&vec![
        id_tok("("),
        id_tok("("),
        int_tok("1"),
        id_tok("-"),
        int_tok("2"),
        id_tok(")"),
        id_tok(")"),
        id_tok("/"),
        int_tok("3"),
    ]);
    assert_eq!(
        result,
        BinaryOp(bin_ast!(int_ast!(1), "-", int_ast!(2)), "/", int_ast!(3),)
    );
}

#[test]
fn test_if_then() {
    let result = parse(&vec![
        id_tok("if"),
        int_tok("1"),
        id_tok("+"),
        int_tok("2"),
        id_tok("then"),
        int_tok("3"),
    ]);
    assert_eq!(
        result,
        Conditional(bin_ast!(int_ast!(1), "+", int_ast!(2)), int_ast!(3), None,)
    );
}

#[test]
fn test_if_then_else() {
    let result = parse(&vec![
        id_tok("if"),
        id_tok("a"),
        id_tok("then"),
        id_tok("b"),
        id_tok("+"),
        id_tok("c"),
        id_tok("else"),
        int_tok("1"),
        id_tok("*"),
        int_tok("2"),
    ]);
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
fn test_embedded_if_then_else() {
    let result = parse(&vec![
        int_tok("1"),
        id_tok("+"),
        id_tok("if"),
        id_tok("true"),
        id_tok("then"),
        int_tok("2"),
        id_tok("else"),
        int_tok("3"),
    ]);
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
fn test_nested_if_then_else() {
    let result = parse(&vec![
        id_tok("if"),
        id_tok("true"),
        id_tok("then"),
        id_tok("if"),
        id_tok("false"),
        id_tok("then"),
        int_tok("1"),
        id_tok("else"),
        int_tok("2"),
        id_tok("else"),
        int_tok("3"),
    ]);
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
fn test_func_basic() {
    let result = parse(&vec![
        id_tok("f"),
        punc_tok("("),
        id_tok("a"),
        punc_tok(","),
        id_tok("b"),
        punc_tok(")"),
    ]);
    assert_eq!(
        result,
        FunCall("f", vec![Identifier("a"), Identifier("b"),])
    );

    let result = parse(&vec![
        id_tok("f"),
        punc_tok("("),
        id_tok("a"),
        punc_tok(","),
        int_tok("1"),
        id_tok("+"),
        int_tok("2"),
        punc_tok(")"),
    ]);
    assert_eq!(
        result,
        FunCall(
            "f",
            vec![Identifier("a"), BinaryOp(int_ast!(1), "+", int_ast!(2),),]
        )
    );

    let result = parse(&vec![id_tok("f"), punc_tok("("), punc_tok(")")]);
    assert_eq!(result, FunCall("f", vec![]));
}

#[test]
fn test_func_nested() {
    let result = parse(&vec![
        id_tok("f"),
        punc_tok("("),
        id_tok("a"),
        punc_tok(","),
        id_tok("g"),
        punc_tok("("),
        id_tok("b"),
        punc_tok(")"),
        punc_tok(")"),
    ]);
    assert_eq!(
        result,
        FunCall(
            "f",
            vec![Identifier("a"), FunCall("g", vec![Identifier("b")]),]
        )
    );
}

#[test]
fn test_func_embedded() {
    let result = parse(&vec![
        int_tok("1"),
        id_tok("+"),
        id_tok("f"),
        punc_tok("("),
        id_tok("a"),
        punc_tok(")"),
    ]);
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
#[should_panic]
fn test_parenthesized_mismatched() {
    parse(&vec![
        id_tok("("),
        int_tok("1"),
        id_tok("+"),
        int_tok("2"),
        id_tok("*"),
        int_tok("3"),
    ]);
}

#[test]
#[should_panic]
fn test_func_missing_comma() {
    parse(&vec![
        id_tok("f"),
        punc_tok("("),
        id_tok("a"),
        id_tok("b"),
        punc_tok(")"),
    ]);
}

#[test]
#[should_panic]
fn test_func_missing_close() {
    parse(&vec![id_tok("f"), punc_tok("("), id_tok("a")]);
}

#[test]
#[should_panic]
fn test_if_no_then() {
    parse(&vec![id_tok("if"), id_tok("true")]);
}

#[test]
#[should_panic]
fn test_empty() {
    parse(&vec![]);
}

#[test]
#[should_panic]
fn test_invalid_start() {
    parse(&vec![int_tok("1"), int_tok("2"), id_tok("+"), int_tok("3")]);
}

#[test]
#[should_panic]
fn test_invalid_middle() {
    parse(&vec![
        int_tok("1"),
        id_tok("+"),
        int_tok("2"),
        int_tok("2"),
        id_tok("+"),
        int_tok("3"),
    ]);
}

#[test]
#[should_panic]
fn test_invalid_end() {
    parse(&vec![
        int_tok("1"),
        id_tok("+"),
        int_tok("2"),
        int_tok("2"),
        id_tok("+"),
        int_tok("3"),
    ]);
}
