use crate::compiler::{
    ast::{
        AstNode,
        Expression::{self, *},
    },
    symtab::SymTab,
    variable::Value,
};

// Function was made as an exercise mid-way through the project and has been left mostly as-is
// since!

pub fn interpret<'source>(ast: &AstNode<'source>, symbols: &mut SymTab<'source, Value>) -> Value {
    match &ast.expr {
        EmptyLiteral() => Value::None(),
        IntLiteral(val) => Value::Int(*val),
        BoolLiteral(val) => Value::Bool(*val),
        Identifier(name) => *symbols.get(name).unwrap(),
        UnaryOp(op, expr) => {
            let Value::Func(op_fn) = symbols.get(&format!("unary_{op}")).unwrap() else {
                panic!("Operator {} does not correspond to a function!", op);
            };
            op_fn(&[interpret(expr, symbols)])
        }
        BinaryOp(left, op, right) => match *op {
            "and" => {
                let left_val = interpret(left, symbols);
                if let Value::Bool(left_val) = left_val {
                    if !left_val {
                        Value::Bool(false)
                    } else {
                        let right_val = interpret(right, symbols);
                        if let Value::Bool(right_val) = right_val {
                            Value::Bool(right_val)
                        } else {
                            panic!("Non-bool with and operator");
                        }
                    }
                } else {
                    panic!("Non-bool with and operator");
                }
            }
            "or" => {
                let left_val = interpret(left, symbols);
                if let Value::Bool(left_val) = left_val {
                    if left_val {
                        Value::Bool(true)
                    } else {
                        let right_val = interpret(right, symbols);
                        if let Value::Bool(right_val) = right_val {
                            Value::Bool(right_val)
                        } else {
                            panic!("Non-bool with or operator");
                        }
                    }
                } else {
                    panic!("Non-bool with or operator");
                }
            }
            "=" => {
                if let Expression::Identifier(name) = left.expr {
                    let val = interpret(right, symbols);
                    *symbols.get(name).unwrap() = val;
                    val
                } else {
                    panic!("Assignment must have identifier as left expr!");
                }
            }
            _ => {
                let Value::Func(op_fn) = symbols.get(op).unwrap() else {
                    panic!("Operator {} does not correspond to a function!", op);
                };
                op_fn(&[interpret(left, symbols), interpret(right, symbols)])
            }
        },
        VarDeclaration(name, expr, _) => {
            let val = interpret(expr, symbols);
            symbols.insert(name, val).unwrap();
            Value::None()
        }
        Conditional(condition_expr, then_expr, else_expr) => {
            let Value::Bool(condition) = interpret(condition_expr, symbols) else {
                panic!("Non-bool as if-then-else condition!");
            };

            if let Some(else_expr) = else_expr {
                if condition {
                    interpret(then_expr, symbols)
                } else {
                    interpret(else_expr, symbols)
                }
            } else {
                if condition {
                    interpret(then_expr, symbols);
                }
                Value::None()
            }
        }
        While(condition, do_expr) => {
            loop {
                let condition = interpret(condition, symbols);
                if let Value::Bool(cond) = condition {
                    if cond {
                        interpret(do_expr, symbols);
                    } else {
                        break;
                    }
                } else {
                    panic!("Non-boon as while-do condition!");
                }
            }
            Value::None()
        }
        FunCall(name, args) => {
            let mut arg_values = Vec::new();
            for arg in args {
                arg_values.push(interpret(arg, symbols));
            }

            let Value::Func(function) = symbols.get(name).unwrap() else {
                panic!("Identifier {} does not correspond to a function!", name);
            };

            function(&arg_values)
        }
        Block(expressions) => {
            symbols.push_level();

            let mut val = Value::None();
            for expression in expressions {
                val = interpret(expression, symbols);
            }

            symbols.remove_level();
            val
        }
    }
}
