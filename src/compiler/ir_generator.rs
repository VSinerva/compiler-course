use std::collections::HashMap;

use crate::compiler::{
    ast::{AstNode, Expression::*},
    ir::{IrInstruction, IrInstructionType::*, IrVar},
    symtab::SymTab,
    variable::Type,
};

pub fn generate_ir(ast: &AstNode) -> Vec<IrInstruction> {
    let mut instructions = Vec::new();

    let mut symbols = SymTab::new();
    let global_types = IrVar::new_global_types();
    let mut types = global_types.clone();
    for var in global_types.keys() {
        symbols.insert(&var.name, var.clone());
    }

    let result = visit_ast_node(ast, &mut types, &mut symbols, &mut instructions);

    match types.get(&result) {
        Some(Type::Int) => {
            let loc = instructions.last().unwrap().loc;
            let fn_var = symbols.get("print_int").clone();

            instructions.push(IrInstruction::new(
                loc,
                Call(fn_var, vec![result], symbols.get("unit").clone()),
            ));
        }
        Some(Type::Bool) => {
            let loc = instructions.last().unwrap().loc;
            let fn_var = symbols.get("print_bool").clone();

            instructions.push(IrInstruction::new(
                loc,
                Call(fn_var, vec![result], symbols.get("unit").clone()),
            ));
        }
        _ => (),
    }

    instructions
}

fn add_var(var_type: &Type, types: &mut HashMap<IrVar, Type>) -> IrVar {
    let mut i = 1;
    //let type_str = match var_type {
    //    Type::Int => "i",
    //    Type::Bool => "b",
    //    Type::Func(_, _) => "f",
    //    Type::Unit => "u",
    //};
    let type_str = "x";

    let mut var = IrVar::new(&format!("{}{}", type_str, i));

    while types.contains_key(&var) {
        i += 1;
        var = IrVar::new(&format!("{}{}", type_str, i));
    }

    types.insert(var.clone(), var_type.clone());
    var
}

fn visit_ast_node(
    ast: &AstNode,
    types: &mut HashMap<IrVar, Type>,
    symbols: &mut SymTab<IrVar>,
    instructions: &mut Vec<IrInstruction>,
) -> IrVar {
    match &ast.expr {
        EmptyLiteral() => symbols.get("unit").clone(),
        IntLiteral(val) => {
            let var = add_var(&Type::Int, types);
            instructions.push(IrInstruction::new(ast.loc, LoadIntConst(*val, var.clone())));
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
        Identifier(name) => symbols.get(name).clone(),
        UnaryOp(_, _) => todo!(),
        BinaryOp(left, op, right) => match *op {
            "=" => todo!(),   // TODO Special handling
            "and" => todo!(), // TODO Special handling
            "or" => todo!(),  // TODO Special handling
            _ => {
                let op_var = symbols.get(op).clone();
                let left_var = visit_ast_node(left, types, symbols, instructions);
                let right_var = visit_ast_node(right, types, symbols, instructions);
                let result_var = add_var(&ast.node_type, types);

                instructions.push(IrInstruction::new(
                    ast.loc,
                    Call(op_var, vec![left_var, right_var], result_var.clone()),
                ));

                result_var
            }
        },
        VarDeclaration(_, _, _) => todo!(),
        Conditional(_, _, _) => todo!(),
        While(_, _) => todo!(),
        FunCall(_, _) => todo!(),
        Block(_) => todo!(),
    }
}
