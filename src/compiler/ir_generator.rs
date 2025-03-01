use std::collections::{HashMap, HashSet};

use crate::compiler::{
    ast::{AstNode, Expression::*},
    ir::{
        IrInstruction,
        IrInstructionType::{self, *},
        IrVar,
    },
    symtab::SymTab,
    token::CodeLocation,
    variable::Type,
};

pub fn generate_ir(ast: &AstNode) -> Vec<IrInstruction> {
    let mut instructions = Vec::new();

    let mut symbols = SymTab::new();
    let mut labels = HashSet::new();
    let global_types = IrVar::new_global_types();
    let mut types = global_types.clone();
    for var in global_types.keys() {
        symbols.insert(&var.name, var.clone()).unwrap();
    }

    let result = visit_ast_node(
        ast,
        &mut types,
        &mut symbols,
        &mut instructions,
        &mut labels,
    );

    match types.get(&result) {
        Some(Type::Int) => {
            let loc = instructions.last().unwrap().loc;
            let fn_var = symbols.get("print_int").unwrap().clone();
            let result_var = add_var(&Type::Bool, &mut types);

            instructions.push(IrInstruction::new(
                loc,
                Call(fn_var, vec![result], result_var),
            ));
        }
        Some(Type::Bool) => {
            let loc = instructions.last().unwrap().loc;
            let fn_var = symbols.get("print_bool").unwrap().clone();
            let result_var = add_var(&Type::Bool, &mut types);

            instructions.push(IrInstruction::new(
                loc,
                Call(fn_var, vec![result], result_var),
            ));
        }
        _ => (),
    }

    instructions
}

fn add_var(var_type: &Type, types: &mut HashMap<IrVar, Type>) -> IrVar {
    match var_type {
        Type::Unit => {
            let var = IrVar::new("unit");
            types.insert(var.clone(), var_type.clone());
            var
        }
        _ => {
            let mut i = 1;
            let mut var = IrVar::new(&format!("x{}", i));

            while types.contains_key(&var) {
                i += 1;
                var = IrVar::new(&format!("x{}", i));
            }

            types.insert(var.clone(), var_type.clone());
            var
        }
    }
}

fn add_label(
    label: &str,
    loc: CodeLocation,
    labels: &mut HashSet<IrInstructionType>,
) -> IrInstruction {
    let mut i = 1;

    let mut instruction = IrInstructionType::Label(format!("{}{}", label, i));

    while labels.contains(&instruction) {
        i += 1;
        instruction = IrInstructionType::Label(format!("{}{}", label, i));
    }

    labels.insert(instruction.clone());
    IrInstruction::new(loc, instruction)
}

fn visit_ast_node<'source>(
    ast: &AstNode<'source>,
    types: &mut HashMap<IrVar, Type>,
    symbols: &mut SymTab<'source, IrVar>,
    instructions: &mut Vec<IrInstruction>,
    labels: &mut HashSet<IrInstructionType>,
) -> IrVar {
    match &ast.expr {
        EmptyLiteral() => add_var(&Type::Unit, types),
        IntLiteral(val) => {
            let var = add_var(&Type::Int, types);
            let val = *val as i64;
            instructions.push(IrInstruction::new(ast.loc, LoadIntConst(val, var.clone())));
            var
        }
        BoolLiteral(val) => {
            let var = add_var(&Type::Bool, types);
            instructions.push(IrInstruction {
                loc: ast.loc,
                instruction: LoadBoolConst(*val, var.clone()),
            });
            var
        }
        Identifier(name) => symbols.get(name).unwrap().clone(),
        UnaryOp(op, expr) => {
            let op_var = symbols.get(&format!("unary_{op}")).unwrap().clone();
            let expr_var = visit_ast_node(expr, types, symbols, instructions, labels);
            let result_var = add_var(&ast.node_type, types);

            instructions.push(IrInstruction::new(
                ast.loc,
                Call(op_var, vec![expr_var], result_var.clone()),
            ));

            result_var
        }
        BinaryOp(left, op, right) => match *op {
            "=" => {
                let right_var = visit_ast_node(right, types, symbols, instructions, labels);
                let Identifier(var_name) = left.expr else {
                    panic!("Tried to assign to non-variable!");
                };
                let var = symbols.get(var_name).unwrap().clone();

                instructions.push(IrInstruction::new(right.loc, Copy(right_var, var.clone())));

                var
            }
            "and" => {
                let l_right = add_label("and_right", right.loc, labels);
                let l_skip = add_label("and_skip", ast.loc, labels);
                let l_end = add_label("and_end", ast.loc, labels);

                let left_var = visit_ast_node(left, types, symbols, instructions, labels);
                instructions.push(IrInstruction::new(
                    left.loc,
                    CondJump(
                        left_var,
                        Box::new(l_right.clone()),
                        Box::new(l_skip.clone()),
                    ),
                ));

                instructions.push(l_right);
                let right_var = visit_ast_node(right, types, symbols, instructions, labels);
                let result_var = add_var(&ast.node_type, types);
                instructions.push(IrInstruction::new(
                    right.loc,
                    Copy(right_var, result_var.clone()),
                ));
                instructions.push(IrInstruction::new(right.loc, Jump(Box::new(l_end.clone()))));

                instructions.push(l_skip);
                instructions.push(IrInstruction::new(
                    right.loc,
                    LoadBoolConst(false, result_var.clone()),
                ));
                instructions.push(IrInstruction::new(right.loc, Jump(Box::new(l_end.clone()))));

                instructions.push(l_end);
                result_var
            }
            "or" => {
                let l_right = add_label("or_right", right.loc, labels);
                let l_skip = add_label("or_skip", ast.loc, labels);
                let l_end = add_label("or_end", ast.loc, labels);

                let left_var = visit_ast_node(left, types, symbols, instructions, labels);
                instructions.push(IrInstruction::new(
                    left.loc,
                    CondJump(
                        left_var,
                        Box::new(l_skip.clone()),
                        Box::new(l_right.clone()),
                    ),
                ));

                instructions.push(l_right);
                let right_var = visit_ast_node(right, types, symbols, instructions, labels);
                let result_var = add_var(&ast.node_type, types);
                instructions.push(IrInstruction::new(
                    right.loc,
                    Copy(right_var, result_var.clone()),
                ));
                instructions.push(IrInstruction::new(right.loc, Jump(Box::new(l_end.clone()))));

                instructions.push(l_skip);
                instructions.push(IrInstruction::new(
                    right.loc,
                    LoadBoolConst(true, result_var.clone()),
                ));
                instructions.push(IrInstruction::new(right.loc, Jump(Box::new(l_end.clone()))));

                instructions.push(l_end);
                result_var
            }
            _ => {
                let op_var = symbols.get(op).unwrap().clone();
                let left_var = visit_ast_node(left, types, symbols, instructions, labels);
                let right_var = visit_ast_node(right, types, symbols, instructions, labels);
                let result_var = add_var(&ast.node_type, types);

                instructions.push(IrInstruction::new(
                    ast.loc,
                    Call(op_var, vec![left_var, right_var], result_var.clone()),
                ));

                result_var
            }
        },
        VarDeclaration(name, expr, _) => {
            let expr_var = visit_ast_node(expr, types, symbols, instructions, labels);
            let result_var = add_var(&expr.node_type, types);
            symbols.insert(name, result_var.clone()).unwrap();
            instructions.push(IrInstruction::new(expr.loc, Copy(expr_var, result_var)));
            add_var(&Type::Unit, types)
        }
        Conditional(condition_expr, then_expr, else_expr) => match else_expr {
            Some(else_expr) => {
                let l_then = add_label("then", then_expr.loc, labels);
                let l_else = add_label("else", else_expr.loc, labels);
                let l_end = add_label("if_end", else_expr.loc, labels);

                let cond_var = visit_ast_node(condition_expr, types, symbols, instructions, labels);
                let result_var = add_var(&ast.node_type, types);

                instructions.push(IrInstruction::new(
                    condition_expr.loc,
                    CondJump(cond_var, Box::new(l_then.clone()), Box::new(l_else.clone())),
                ));

                instructions.push(l_then);
                let then_var = visit_ast_node(then_expr, types, symbols, instructions, labels);
                instructions.push(IrInstruction::new(
                    else_expr.loc,
                    Copy(then_var, result_var.clone()),
                ));
                instructions.push(IrInstruction::new(
                    else_expr.loc,
                    Jump(Box::new(l_end.clone())),
                ));

                instructions.push(l_else);
                let else_var = visit_ast_node(else_expr, types, symbols, instructions, labels);
                instructions.push(IrInstruction::new(
                    else_expr.loc,
                    Copy(else_var, result_var.clone()),
                ));
                instructions.push(l_end);

                result_var
            }
            None => {
                let l_then = add_label("then", then_expr.loc, labels);
                let l_end = add_label("if_end", then_expr.loc, labels);

                let cond_var = visit_ast_node(condition_expr, types, symbols, instructions, labels);

                instructions.push(IrInstruction::new(
                    condition_expr.loc,
                    CondJump(cond_var, Box::new(l_then.clone()), Box::new(l_end.clone())),
                ));

                instructions.push(l_then);
                visit_ast_node(then_expr, types, symbols, instructions, labels);
                instructions.push(l_end);

                add_var(&Type::Unit, types)
            }
        },
        While(condition_expr, do_expr) => {
            let l_start = add_label("while_start", condition_expr.loc, labels);
            let l_body = add_label("while_body", do_expr.loc, labels);
            let l_end = add_label("while_end", do_expr.loc, labels);

            instructions.push(l_start.clone());
            let cond_var = visit_ast_node(condition_expr, types, symbols, instructions, labels);
            instructions.push(IrInstruction::new(
                condition_expr.loc,
                CondJump(cond_var, Box::new(l_body.clone()), Box::new(l_end.clone())),
            ));
            instructions.push(l_body);
            visit_ast_node(do_expr, types, symbols, instructions, labels);
            instructions.push(IrInstruction::new(do_expr.loc, Jump(Box::new(l_start))));
            instructions.push(l_end);

            add_var(&Type::Unit, types)
        }
        FunCall(name, expressions) => {
            let fn_var = symbols.get(name).unwrap().clone();
            let Type::Func(_, result_type) = types.get(&fn_var).unwrap().clone() else {
                panic!("Function call does not have entry in types table!");
            };

            let mut args = Vec::new();
            for expression in expressions {
                args.push(visit_ast_node(
                    expression,
                    types,
                    symbols,
                    instructions,
                    labels,
                ));
            }
            let result_var = add_var(&result_type, types);

            instructions.push(IrInstruction::new(
                ast.loc,
                Call(fn_var, args, result_var.clone()),
            ));

            result_var
        }
        Block(expressions) => {
            let mut result_var = add_var(&Type::Unit, types);
            symbols.push_level();
            for expression in expressions {
                result_var = visit_ast_node(expression, types, symbols, instructions, labels);
            }
            symbols.remove_level();
            result_var
        }
    }
}
