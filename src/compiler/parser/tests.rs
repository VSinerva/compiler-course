use super::*;
use crate::compiler::{token::CodeLocation, tokenizer::tokenize};

macro_rules! bool_ast {
    ($x:expr) => {
        AstNode::new(CodeLocation::new(usize::MAX, usize::MAX), BoolLiteral($x))
    };
}

macro_rules! bool_ast_b {
    ($x:expr) => {
        Box::new(bool_ast!($x))
    };
}

macro_rules! int_ast {
    ($x:expr) => {
        AstNode::new(CodeLocation::new(usize::MAX, usize::MAX), IntLiteral($x))
    };
}

macro_rules! int_ast_b {
    ($x:expr) => {
        Box::new(int_ast!($x))
    };
}

macro_rules! id_ast {
    ($x:expr) => {
        AstNode::new(CodeLocation::new(usize::MAX, usize::MAX), Identifier($x))
    };
}

macro_rules! id_ast_b {
    ($x:expr) => {
        Box::new(id_ast!($x))
    };
}

macro_rules! un_ast {
    ($x:expr, $y:expr) => {
        AstNode::new(CodeLocation::new(usize::MAX, usize::MAX), UnaryOp($x, $y))
    };
}

macro_rules! un_ast_b {
    ($x:expr, $y:expr) => {
        Box::new(un_ast!($x, $y))
    };
}

macro_rules! bin_ast {
    ($x:expr, $y:expr, $z:expr) => {
        AstNode::new(
            CodeLocation::new(usize::MAX, usize::MAX),
            BinaryOp($x, $y, $z),
        )
    };
}

macro_rules! bin_ast_b {
    ($x:expr, $y:expr, $z:expr) => {
        Box::new(bin_ast!($x, $y, $z))
    };
}

macro_rules! con_ast {
    ($x:expr, $y:expr, $z:expr) => {
        AstNode::new(
            CodeLocation::new(usize::MAX, usize::MAX),
            Conditional($x, $y, $z),
        )
    };
}

macro_rules! con_ast_b {
    ($x:expr, $y:expr, $z:expr) => {
        Box::new(con_ast!($x, $y, $z))
    };
}

macro_rules! fun_ast {
    ($x:expr, $y:expr) => {
        AstNode::new(CodeLocation::new(usize::MAX, usize::MAX), FunCall($x, $y))
    };
}

macro_rules! fun_ast_b {
    ($x:expr, $y:expr) => {
        Box::new(fun_ast!($x, $y))
    };
}

macro_rules! block_ast {
    ($x:expr) => {
        AstNode::new(CodeLocation::new(usize::MAX, usize::MAX), Block($x))
    };
}

macro_rules! block_ast_b {
    ($x:expr) => {
        Box::new(block_ast!($x))
    };
}

macro_rules! empty_ast {
    () => {
        AstNode::new(CodeLocation::new(usize::MAX, usize::MAX), EmptyLiteral())
    };
}

macro_rules! var_ast {
    ($x:expr, $y:expr, $z:expr) => {
        AstNode::new(
            CodeLocation::new(usize::MAX, usize::MAX),
            VarDeclaration($x, $y, $z),
        )
    };
}

macro_rules! while_ast {
    ($x:expr, $y:expr) => {
        AstNode::new(CodeLocation::new(usize::MAX, usize::MAX), While($x, $y))
    };
}

macro_rules! while_ast_b {
    ($x:expr, $y:expr) => {
        Box::new(while_ast!($x, $y))
    };
}

#[test]
#[should_panic]
fn test_empty() {
    parse(&[]);
}

#[test]
#[should_panic]
fn test_invalid_start() {
    parse(&tokenize("1 2 + 3").unwrap());
}

#[test]
#[should_panic]
fn test_invalid_middle() {
    parse(&tokenize("1 + 2 2 + 3").unwrap());
}

#[test]
#[should_panic]
fn test_invalid_end() {
    parse(&tokenize("1 + 2 3").unwrap());
}

#[test]
fn test_binary_op_basic() {
    let result = parse(&tokenize("1 + 23").unwrap());
    assert_eq!(result, bin_ast!(int_ast_b!(1), "+", int_ast_b!(23)));

    let result = parse(&tokenize("4 - 56").unwrap());
    assert_eq!(result, bin_ast!(int_ast_b!(4), "-", int_ast_b!(56)));

    let result = parse(&tokenize("1 * 2").unwrap());
    assert_eq!(result, bin_ast!(int_ast_b!(1), "*", int_ast_b!(2)));

    let result = parse(&tokenize("1 / 2").unwrap());
    assert_eq!(result, bin_ast!(int_ast_b!(1), "/", int_ast_b!(2)));
}

#[test]
fn test_binary_op_all_levels() {
    let result = parse(&tokenize("1 * 2 + 3 < 4 == 5 and 6 or 7").unwrap());
    assert_eq!(
        result,
        bin_ast!(
            bin_ast_b!(
                bin_ast_b!(
                    bin_ast_b!(
                        bin_ast_b!(
                            bin_ast_b!(int_ast_b!(1), "*", int_ast_b!(2)),
                            "+",
                            int_ast_b!(3)
                        ),
                        "<",
                        int_ast_b!(4)
                    ),
                    "==",
                    int_ast_b!(5)
                ),
                "and",
                int_ast_b!(6)
            ),
            "or",
            int_ast_b!(7)
        )
    );
}

#[test]
fn test_binary_op_identifier() {
    let result = parse(&tokenize("a + 1").unwrap());
    assert_eq!(result, bin_ast!(id_ast_b!("a"), "+", int_ast_b!(1)));

    let result = parse(&tokenize("1 - a").unwrap());
    assert_eq!(result, bin_ast!(int_ast_b!(1), "-", id_ast_b!("a")));
}

#[test]
fn test_binary_op_multiple() {
    let result = parse(&tokenize("1 + 2 - 3").unwrap());
    assert_eq!(
        result,
        bin_ast!(
            bin_ast_b!(int_ast_b!(1), "+", int_ast_b!(2)),
            "-",
            int_ast_b!(3)
        )
    );
}

#[test]
fn test_binary_op_precedence() {
    let result = parse(&tokenize("1 + 2 * 3").unwrap());
    assert_eq!(
        result,
        bin_ast!(
            int_ast_b!(1),
            "+",
            bin_ast_b!(int_ast_b!(2), "*", int_ast_b!(3))
        )
    );

    let result = parse(&tokenize("1 - 2 / 3").unwrap());
    assert_eq!(
        result,
        bin_ast!(
            int_ast_b!(1),
            "-",
            bin_ast_b!(int_ast_b!(2), "/", int_ast_b!(3))
        )
    );
}

#[test]
fn test_assignment_basic() {
    let result = parse(&tokenize("a = 1 + 2").unwrap());
    assert_eq!(
        result,
        bin_ast!(
            id_ast_b!("a"),
            "=",
            bin_ast_b!(int_ast_b!(1), "+", int_ast_b!(2))
        )
    );
}

#[test]
fn test_assignment_chain() {
    let result = parse(&tokenize("a = b = 1 + 2").unwrap());
    assert_eq!(
        result,
        bin_ast!(
            id_ast_b!("a"),
            "=",
            bin_ast_b!(
                id_ast_b!("b"),
                "=",
                bin_ast_b!(int_ast_b!(1), "+", int_ast_b!(2))
            )
        )
    );
}

#[test]
#[should_panic]
fn test_assignment_invalid() {
    parse(&tokenize("a =").unwrap());
}

#[test]
fn test_unary_basic() {
    let result = parse(&tokenize("not x").unwrap());
    assert_eq!(result, un_ast!("not", id_ast_b!("x")));

    let result = parse(&tokenize("-x").unwrap());
    assert_eq!(result, un_ast!("-", id_ast_b!("x")));

    let result = parse(&tokenize("-1").unwrap());
    assert_eq!(result, un_ast!("-", int_ast_b!(1)));

    let result = parse(&tokenize("-1 + 2").unwrap());
    assert_eq!(
        result,
        bin_ast!(un_ast_b!("-", int_ast_b!(1)), "+", int_ast_b!(2))
    );
}

#[test]
fn test_unary_chain() {
    let result = parse(&tokenize("not not x").unwrap());
    assert_eq!(result, un_ast!("not", un_ast_b!("not", id_ast_b!("x"))));

    let result = parse(&tokenize("--x").unwrap());
    assert_eq!(result, un_ast!("-", un_ast_b!("-", id_ast_b!("x"))));

    let result = parse(&tokenize("--1").unwrap());
    assert_eq!(result, un_ast!("-", un_ast_b!("-", int_ast_b!(1))));

    let result = parse(&tokenize("--1 + 2").unwrap());
    assert_eq!(
        result,
        bin_ast!(
            un_ast_b!("-", un_ast_b!("-", int_ast_b!(1))),
            "+",
            int_ast_b!(2)
        )
    );
}

#[test]
fn test_parenthesized() {
    let result = parse(&tokenize("(1+2)*3").unwrap());
    assert_eq!(
        result,
        bin_ast!(
            bin_ast_b!(int_ast_b!(1), "+", int_ast_b!(2)),
            "*",
            int_ast_b!(3)
        )
    );
}

#[test]
fn test_parenthesized_nested() {
    let result = parse(&tokenize("((1 - 2))/3").unwrap());
    assert_eq!(
        result,
        bin_ast!(
            bin_ast_b!(int_ast_b!(1), "-", int_ast_b!(2)),
            "/",
            int_ast_b!(3)
        )
    );

    let result = parse(&tokenize("((1 + 2)*3) / 4").unwrap());
    assert_eq!(
        result,
        bin_ast!(
            bin_ast_b!(
                bin_ast_b!(int_ast_b!(1), "+", int_ast_b!(2)),
                "*",
                int_ast_b!(3)
            ),
            "/",
            int_ast_b!(4)
        )
    );
}

#[test]
#[should_panic]
fn test_parenthesized_mismatched() {
    parse(&tokenize("(1+2*3").unwrap());
}

#[test]
fn test_if_then() {
    let result = parse(&tokenize("if 1 + 2 then 3").unwrap());
    assert_eq!(
        result,
        con_ast!(
            bin_ast_b!(int_ast_b!(1), "+", int_ast_b!(2)),
            int_ast_b!(3),
            None
        )
    );
}

#[test]
fn test_if_then_else() {
    let result = parse(&tokenize("if a then b + c else 1 * 2").unwrap());
    assert_eq!(
        result,
        con_ast!(
            id_ast_b!("a"),
            bin_ast_b!(id_ast_b!("b"), "+", id_ast_b!("c")),
            Some(bin_ast_b!(int_ast_b!(1), "*", int_ast_b!(2)))
        )
    );
}

#[test]
fn test_if_then_else_embedded() {
    let result = parse(&tokenize("1 + if true then 2 else 3").unwrap());
    assert_eq!(
        result,
        bin_ast!(
            int_ast_b!(1),
            "+",
            con_ast_b!(bool_ast_b!(true), int_ast_b!(2), Some(int_ast_b!(3)))
        )
    );
}

#[test]
fn test_if_then_else_nested() {
    let result = parse(&tokenize("if true then if false then 1 else 2 else 3").unwrap());
    assert_eq!(
        result,
        con_ast!(
            bool_ast_b!(true),
            con_ast_b!(bool_ast_b!(false), int_ast_b!(1), Some(int_ast_b!(2))),
            Some(int_ast_b!(3))
        )
    );
}

#[test]
#[should_panic]
fn test_if_no_then() {
    parse(&tokenize("if true").unwrap());
}

#[test]
fn test_func_basic() {
    let result = parse(&tokenize("f(a, b)").unwrap());
    assert_eq!(result, fun_ast!("f", vec![id_ast!("a"), id_ast!("b"),]));

    let result = parse(&tokenize("f(a, 1 + 2)").unwrap());
    assert_eq!(
        result,
        fun_ast!(
            "f",
            vec![id_ast!("a"), bin_ast!(int_ast_b!(1), "+", int_ast_b!(2)),]
        )
    );

    let result = parse(&tokenize("f()").unwrap());
    assert_eq!(result, fun_ast!("f", vec![]));
}

#[test]
fn test_func_embedded() {
    let result = parse(&tokenize("1 + f(a)").unwrap());
    assert_eq!(
        result,
        bin_ast!(int_ast_b!(1), "+", fun_ast_b!("f", vec![id_ast!("a")]))
    );
}

#[test]
fn test_func_nested() {
    let result = parse(&tokenize("f(a, g(b))").unwrap());
    assert_eq!(
        result,
        fun_ast!("f", vec![id_ast!("a"), fun_ast!("g", vec![id_ast!("b")]),])
    );
}

#[test]
#[should_panic]
fn test_func_missing_comma() {
    parse(&tokenize("f(a b)").unwrap());
}

#[test]
#[should_panic]
fn test_func_missing_close() {
    parse(&tokenize("f(a").unwrap());
}

#[test]
fn test_block_basic() {
    let result = parse(&tokenize("{ a = 1; b; }").unwrap());
    assert_eq!(
        result,
        block_ast!(vec![
            bin_ast!(id_ast_b!("a"), "=", int_ast_b!(1)),
            id_ast!("b"),
            empty_ast!()
        ])
    );

    let result = parse(&tokenize("{ a = 1; b }").unwrap());
    assert_eq!(
        result,
        block_ast!(vec![
            bin_ast!(id_ast_b!("a"), "=", int_ast_b!(1)),
            id_ast!("b"),
        ])
    );
}

#[test]
fn test_block_embedded() {
    let result = parse(&tokenize("{ 1 + 2 } * 3").unwrap());
    assert_eq!(
        result,
        bin_ast!(
            block_ast_b!(vec![bin_ast!(int_ast_b!(1), "+", int_ast_b!(2))]),
            "*",
            int_ast_b!(3)
        )
    );
}

#[test]
fn test_block_nested() {
    let result = parse(&tokenize("{ a = { 1 + 2}}").unwrap());
    assert_eq!(
        result,
        block_ast!(vec![bin_ast!(
            id_ast_b!("a"),
            "=",
            block_ast_b!(vec![bin_ast!(int_ast_b!(1), "+", int_ast_b!(2))])
        )])
    );
}

#[test]
#[should_panic]
fn test_block_unmatched() {
    parse(&tokenize("{ a = 1 ").unwrap());
}

#[test]
#[should_panic]
fn test_block_missing_semicolon() {
    parse(&tokenize("{ a = 1\nb }").unwrap());
}

#[test]
fn test_var_basic() {
    let result = parse(&tokenize("var x = 1").unwrap());
    assert_eq!(result, var_ast!("x", int_ast_b!(1), None));

    let result = parse(&tokenize("{ var x = 1; x = 2; }").unwrap());
    assert_eq!(
        result,
        block_ast!(vec![
            var_ast!("x", int_ast_b!(1), None),
            bin_ast!(id_ast_b!("x"), "=", int_ast_b!(2)),
            empty_ast!()
        ])
    );
}

#[test]
fn test_var_typed() {
    let result = parse(&tokenize("var x: Int = 1").unwrap());
    assert_eq!(
        result,
        var_ast!(
            "x",
            int_ast_b!(1),
            Some(TypeExpression::Int(CodeLocation::new(
                usize::MAX,
                usize::MAX
            )))
        )
    );

    let result = parse(&tokenize("var x: Bool = true").unwrap());
    assert_eq!(
        result,
        var_ast!(
            "x",
            bool_ast_b!(true),
            Some(TypeExpression::Bool(CodeLocation::new(
                usize::MAX,
                usize::MAX
            )))
        )
    );
}

#[test]
#[should_panic]
fn test_var_chain() {
    parse(&tokenize("var x = var y = 1").unwrap());
}

#[test]
#[should_panic]
fn test_var_embedded() {
    parse(&tokenize("if true then var x = 3").unwrap());
}

#[test]
fn test_omitting_semicolons() {
    let result = parse(&tokenize("{ { a } { b } }").unwrap());
    assert_eq!(
        result,
        block_ast!(vec![
            block_ast!(vec![id_ast!("a")]),
            block_ast!(vec![id_ast!("b")])
        ])
    );

    let result = parse(&tokenize("{ if true then { a } b }").unwrap());
    assert_eq!(
        result,
        block_ast!(vec![
            con_ast!(bool_ast_b!(true), block_ast_b!(vec![id_ast!("a")]), None),
            id_ast!("b"),
        ])
    );

    let result = parse(&tokenize("{ if true then { a }; b }").unwrap());
    assert_eq!(
        result,
        block_ast!(vec![
            con_ast!(bool_ast_b!(true), block_ast_b!(vec![id_ast!("a")]), None),
            id_ast!("b"),
        ])
    );

    let result = parse(&tokenize("{ if true then { a } else { b } c }").unwrap());
    assert_eq!(
        result,
        block_ast!(vec![
            con_ast!(
                bool_ast_b!(true),
                block_ast_b!(vec![id_ast!("a")]),
                Some(block_ast_b!(vec![id_ast!("b")]))
            ),
            id_ast!("c"),
        ])
    );

    let result = parse(&tokenize("x = { { f(a) } { b } }").unwrap());
    assert_eq!(
        result,
        bin_ast!(
            id_ast_b!("x"),
            "=",
            block_ast_b!(vec![
                block_ast!(vec![fun_ast!("f", vec![id_ast!("a")])]),
                block_ast!(vec![id_ast!("b")]),
            ])
        )
    );
}

#[test]
#[should_panic]
fn test_omitting_semicolons_invalid() {
    parse(&tokenize("{ if true then { a } b c }").unwrap());
}

#[test]
fn test_while_do() {
    let result = parse(&tokenize("while 1 + 2 do 3").unwrap());
    assert_eq!(
        result,
        while_ast!(bin_ast_b!(int_ast_b!(1), "+", int_ast_b!(2)), int_ast_b!(3))
    );
}

#[test]
fn test_while_do_embedded() {
    let result = parse(&tokenize("1 + while true do 2").unwrap());
    assert_eq!(
        result,
        bin_ast!(
            int_ast_b!(1),
            "+",
            while_ast_b!(bool_ast_b!(true), int_ast_b!(2))
        )
    );
}

#[test]
fn test_while_do_nested() {
    let result = parse(&tokenize("while true do while false do 1").unwrap());
    assert_eq!(
        result,
        while_ast!(
            bool_ast_b!(true),
            while_ast_b!(bool_ast_b!(false), int_ast_b!(1))
        )
    );
}

#[test]
#[should_panic]
fn test_while_no_do() {
    parse(&tokenize("while true").unwrap());
}

#[test]
fn test_multiple_top_levels() {
    let result = parse(&tokenize("a;").unwrap());
    assert_eq!(result, block_ast!(vec![id_ast!("a"), empty_ast!()]));

    let result = parse(&tokenize("a; b").unwrap());
    assert_eq!(result, block_ast!(vec![id_ast!("a"), id_ast!("b")]));

    let result = parse(&tokenize("{}{}").unwrap());
    assert_eq!(
        result,
        block_ast!(vec![block_ast!(vec![]), block_ast!(vec![])])
    );
}

#[test]
fn test_large() {
    let result = parse(
        &tokenize(
            "
{
    while f() do {
        x = 10;
        y = if g(x) then {
            x = x + 1;
            x
        } else {
            g(x)
        }
        g(y);
    }
    123
}
",
        )
        .unwrap(),
    );

    assert_eq!(
        result,
        block_ast!(vec![
            while_ast!(
                fun_ast_b!("f", vec![]),
                block_ast_b!(vec![
                    bin_ast!(id_ast_b!("x"), "=", int_ast_b!(10)),
                    bin_ast!(
                        id_ast_b!("y"),
                        "=",
                        con_ast_b!(
                            fun_ast_b!("g", vec![id_ast!("x")]),
                            block_ast_b!(vec![
                                bin_ast!(
                                    id_ast_b!("x"),
                                    "=",
                                    bin_ast_b!(id_ast_b!("x"), "+", int_ast_b!(1))
                                ),
                                id_ast!("x")
                            ]),
                            Some(block_ast_b!(vec![fun_ast!("g", vec![id_ast!("x")])]))
                        )
                    ),
                    fun_ast!("g", vec![id_ast!("y")]),
                    empty_ast!()
                ])
            ),
            int_ast!(123),
        ])
    );
}
