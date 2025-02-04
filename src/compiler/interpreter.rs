use crate::compiler::{
    ast::Expression::{self, *},
    symtab::SymTab,
    variable::Value,
};

pub fn interpret<'source>(ast: &Expression<'source>, symbols: &mut SymTab<'source>) -> Value {
    match ast {
        EmptyLiteral(_) => Value::None(),
        IntLiteral(_, val) => Value::Int(*val),
        BoolLiteral(_, val) => Value::Bool(*val),
        Identifier(_, name) => *symbols.get(name),
        UnaryOp(_, op, expr) => match *op {
            "-" => {
                let Value::Func(op_fn) = symbols.get("neg") else {
                    panic!("Operator {} does not correspond to a function!", op);
                };
                op_fn(&[interpret(expr, symbols)])
            }
            _ => {
                let Value::Func(op_fn) = symbols.get(op) else {
                    panic!("Operator {} does not correspond to a function!", op);
                };
                op_fn(&[interpret(expr, symbols)])
            }
        },
        BinaryOp(_, left, op, right) => match *op {
            "and" => {
                let left_val = interpret(left, symbols);
                if let Value::Bool(val_l) = left_val {
                    if !val_l {
                        Value::Bool(false)
                    } else {
                        let right_val = interpret(right, symbols);
                        if let Value::Bool(val_r) = right_val {
                            Value::Bool(val_r)
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
                if let Value::Bool(val_l) = left_val {
                    if val_l {
                        Value::Bool(true)
                    } else {
                        let right_val = interpret(right, symbols);
                        if let Value::Bool(val_r) = right_val {
                            Value::Bool(val_r)
                        } else {
                            panic!("Non-bool with and operator");
                        }
                    }
                } else {
                    panic!("Non-bool with and operator");
                }
            }
            "=" => {
                if let Expression::Identifier(_, name) = **left {
                    let val = interpret(right, symbols);
                    *symbols.get(name) = val;
                    val
                } else {
                    panic!("Assignment must have identifier as left expr!");
                }
            }
            _ => {
                let Value::Func(op_fn) = symbols.get(op) else {
                    panic!("Operator {} does not correspond to a function!", op);
                };
                op_fn(&[interpret(left, symbols), interpret(right, symbols)])
            }
        },
        VarDeclaration(_, name, expr) => {
            let val = interpret(expr, symbols);
            symbols.insert(name, val);
            Value::None()
        }
        Conditional(_, condition_expr, then_expr, else_expr) => {
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
        While(_, condition, do_expr) => {
            let mut val = Value::None();
            loop {
                let condition = interpret(condition, symbols);
                if let Value::Bool(cond) = condition {
                    if cond {
                        val = interpret(do_expr, symbols);
                    } else {
                        break;
                    }
                } else {
                    panic!("Non-boon as while-do condition!");
                }
            }
            val
        }
        FunCall(_, name, args) => {
            let mut arg_values = Vec::new();
            for arg in args {
                arg_values.push(interpret(arg, symbols));
            }

            let Value::Func(function) = symbols.get(name) else {
                panic!("Identifier {} does not correspond to a function!", name);
            };

            function(&arg_values)
        }
        Block(_, expressions) => {
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
