use crate::compiler::{
    ast::Expression::{self, *},
    value::Value,
};

#[expect(unused_variables)]
pub fn interpret(ast: &Expression) -> Value {
    match ast {
        EmptyLiteral(_) => Value::None(),
        IntLiteral(_, val) => Value::Int(*val),
        BoolLiteral(_, val) => Value::Bool(*val),
        Identifier(_, _) => todo!(), // Variables are TODO
        UnaryOp(_, op, expr) => match *op {
            "-" => -interpret(expr),
            "not" => !interpret(expr),
            _ => panic!("Unrecognized unary op {}", op),
        },
        BinaryOp(_, left, op, right) => match *op {
            "+" => interpret(left) + interpret(right),
            "*" => interpret(left) * interpret(right),
            "-" => interpret(left) - interpret(right),
            "/" => interpret(left) / interpret(right),
            "%" => interpret(left) % interpret(right),
            "==" => Value::Bool(interpret(left) == interpret(right)),
            "!=" => Value::Bool(interpret(left) != interpret(right)),
            "<" => Value::Bool(interpret(left) < interpret(right)),
            "<=" => Value::Bool(interpret(left) <= interpret(right)),
            ">" => Value::Bool(interpret(left) > interpret(right)),
            ">=" => Value::Bool(interpret(left) >= interpret(right)),
            "and" => interpret(left).and(&interpret(right)),
            "or" => interpret(left).or(&interpret(right)),
            "=" => todo!(), // Variables are TODO
            _ => panic!("Unrecognized binary op {}", op),
        },
        VarDeclaration(_, name, expr) => todo!(), // Variables are TODO
        Conditional(_, condition_expr, then_expr, else_expr) => {
            if let Value::Bool(condition) = interpret(condition_expr) {
                if condition {
                    interpret(then_expr)
                } else if let Some(expr) = else_expr {
                    interpret(expr)
                } else {
                    Value::None()
                }
            } else {
                panic!("Non-bool as if-then-else condition!");
            }
        }
        While(_, condition, do_expr) => {
            let mut val = Value::None();
            loop {
                let condition = interpret(condition);
                if let Value::Bool(cond) = condition {
                    if cond {
                        val = interpret(do_expr);
                    } else {
                        break;
                    }
                } else {
                    panic!("Non-boon as while-do condition!");
                }
            }
            val
        }
        FunCall(_, name, args) => todo!(), // Functions are TODO
        Block(_, expressions) => {
            let mut val = Value::None();
            for expression in expressions {
                val = interpret(expression);
            }
            val
        }
    }
}
