use crate::compiler::{
    ast::Expression::{self, *},
    symtab::SymTab,
    value::Value,
};

pub struct Interpreter<'source> {
    symbols: SymTab<'source>,
}

impl<'source> Interpreter<'source> {
    pub fn new() -> Self {
        Interpreter {
            symbols: SymTab::new(),
        }
    }

    pub fn interpret(&mut self, ast: &Expression<'source>) -> Value {
        match ast {
            EmptyLiteral(_) => Value::None(),
            IntLiteral(_, val) => Value::Int(*val),
            BoolLiteral(_, val) => Value::Bool(*val),
            Identifier(_, name) => *self.symbols.get(name),
            UnaryOp(_, op, expr) => match *op {
                "-" => {
                    let Value::Func(op_fn) = self.symbols.get("neg") else {
                        panic!("Operator {} does not correspond to a function!", op);
                    };
                    op_fn(&[self.interpret(expr)])
                }
                _ => {
                    let Value::Func(op_fn) = self.symbols.get(op) else {
                        panic!("Operator {} does not correspond to a function!", op);
                    };
                    op_fn(&[self.interpret(expr)])
                }
            },
            BinaryOp(_, left, op, right) => match *op {
                "and" => {
                    let left_val = self.interpret(left);
                    if let Value::Bool(val_l) = left_val {
                        if !val_l {
                            Value::Bool(false)
                        } else {
                            let right_val = self.interpret(right);
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
                    let left_val = self.interpret(left);
                    if let Value::Bool(val_l) = left_val {
                        if val_l {
                            Value::Bool(true)
                        } else {
                            let right_val = self.interpret(right);
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
                        let val = self.interpret(right);
                        *self.symbols.get(name) = val;
                        val
                    } else {
                        panic!("Assignment must have identifier as left expr!");
                    }
                }
                _ => {
                    let Value::Func(op_fn) = self.symbols.get(op) else {
                        panic!("Operator {} does not correspond to a function!", op);
                    };
                    op_fn(&[self.interpret(left), self.interpret(right)])
                }
            },
            VarDeclaration(_, name, expr) => {
                let val = self.interpret(expr);
                self.symbols.insert(name, val);
                Value::None()
            }
            Conditional(_, condition_expr, then_expr, else_expr) => {
                if let Value::Bool(condition) = self.interpret(condition_expr) {
                    if condition {
                        self.interpret(then_expr)
                    } else if let Some(expr) = else_expr {
                        self.interpret(expr)
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
                    let condition = self.interpret(condition);
                    if let Value::Bool(cond) = condition {
                        if cond {
                            val = self.interpret(do_expr);
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
                    arg_values.push(self.interpret(arg));
                }

                let Value::Func(function) = self.symbols.get(name) else {
                    panic!("Identifier {} does not correspond to a function!", name);
                };

                function(&arg_values)
            }
            Block(_, expressions) => {
                self.symbols.push_level();

                let mut val = Value::None();
                for expression in expressions {
                    val = self.interpret(expression);
                }

                self.symbols.remove_level();
                val
            }
        }
    }
}
