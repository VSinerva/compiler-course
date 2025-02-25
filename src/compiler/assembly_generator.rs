use std::collections::{HashMap, HashSet};

use crate::compiler::ir::{IrInstruction, IrVar};

pub fn generate_assembly(instructions: &Vec<IrInstruction>) -> String {
    const ARG_REGISTERS: [&str; 6] = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];
    const INSTRINSICS: [&str; 13] = [
        "+",
        "*",
        "-",
        "/",
        "%",
        "<",
        "<=",
        ">",
        ">=",
        "==",
        "!=",
        "unary_not",
        "unary_-",
    ];
    let locals = Locals::new(instructions);

    let mut out = String::from(
        "\t.extern print_int
\t.extern print_bool
\t.extern read_int 
\t.global main
\t.type main, @function

\t.section .text

main:
\tpushq %rbp
\tmovq %rsp, %rbp
",
    );

    out.push_str(&format!("\tsubq ${}, %rsp\n", locals.stack_used()));

    for instruction in instructions {
        out.push_str(&format!("\n\t# {}\n", instruction.instruction));

        use crate::compiler::ir::IrInstructionType::*;
        match &instruction.instruction {
            LoadBoolConst(val, dest) => {
                let val = if *val { 1 } else { 0 };
                out.push_str(&format!("\tmovq ${val}, {}\n", locals.get_ref(dest)));
            }
            LoadIntConst(val, dest) => {
                // x86-64 weirdness with large integers
                if -2_i64.pow(31) <= *val && *val < 2_i64.pow(31) {
                    out.push_str(&format!("\tmovq ${val}, {}\n", locals.get_ref(dest)));
                } else {
                    out.push_str(&format!("\tmovq ${val}, %rax\n"));
                    out.push_str(&format!("\tmovq %rax, {}\n", locals.get_ref(dest)));
                }
            }
            Copy(src, dest) => {
                out.push_str(&format!("\tmovq {}, %rax\n", locals.get_ref(src)));
                out.push_str(&format!("\tmovq %rax, {}\n", locals.get_ref(dest)));
            }
            Call(op_var, arg_vec, output_var) => {
                assert!(
                    arg_vec.len() <= 6,
                    "More than 6 args to a function '{op_var}' !"
                );

                if INSTRINSICS.contains(&&*op_var.name) {
                    handle_intrinsics(&mut out, &locals, op_var, arg_vec, output_var);
                } else {
                    // Align stack according to spec
                    if locals.stack_used % 16 != 0 {
                        out.push_str("\tsubq $8, %rsp\n");
                    }

                    for (i, var) in arg_vec.iter().enumerate() {
                        out.push_str(&format!(
                            "\tmovq {}, {}\n",
                            locals.get_ref(var),
                            ARG_REGISTERS[i]
                        ));
                    }
                    out.push_str(&format!("\tcallq {op_var}\n"));
                    out.push_str(&format!("\tmovq %rax, {}\n", locals.get_ref(output_var)));

                    // Align stack according to spec
                    if locals.stack_used % 16 != 0 {
                        out.push_str("\taddq $8, %rsp\n");
                    }
                }
            }
            Jump(target) => {
                let Label(target_name) = &target.instruction else {
                    panic!("Tried to jump to non-label {target}")
                };
                out.push_str(&format!("\tjmp .L{target_name}\n"));
            }
            CondJump(cond, jmp_then, jmp_else) => {
                let Label(then_target) = &jmp_then.instruction else {
                    panic!("Tried to jump to non-label {jmp_then}")
                };
                let Label(else_target) = &jmp_else.instruction else {
                    panic!("Tried to jump to non-label {jmp_else}")
                };

                out.push_str(&format!("\tcmpq $0, {}\n", locals.get_ref(cond)));
                out.push_str(&format!("\tjne .L{then_target}\n"));
                out.push_str(&format!("\tjmp .L{else_target}\n"));
            }
            Label(name) => out.push_str(&format!(".L{name}:\n")),
        }
    }

    out.push_str(
        "
\tmovq $0, %rax
\tmovq %rbp, %rsp
\tpopq %rbp
\tret",
    );

    out
}

fn handle_intrinsics(
    out: &mut String,
    locals: &Locals,
    op_var: &IrVar,
    arg_vec: &[IrVar],
    output_var: &IrVar,
) {
    let res = "%rax";
    match &*op_var.name {
        "unary_not" => {
            let arg0 = locals.get_ref(&arg_vec[0]);
            out.push_str(&format!("\tmovq {arg0}, {res}\n"));
            out.push_str(&format!("\txorq $1, {res}\n"));
        }
        "unary_-" => {
            let arg0 = locals.get_ref(&arg_vec[0]);
            out.push_str(&format!("\tmovq {arg0}, {res}\n"));
            out.push_str(&format!("\tnegq {res}\n"));
        }
        _ => {
            // Must be binary intrinsic
            let arg0 = locals.get_ref(&arg_vec[0]);
            let arg1 = locals.get_ref(&arg_vec[1]);
            match &*op_var.name {
                "+" => {
                    if arg0 != res {
                        out.push_str(&format!("\tmovq {arg0}, {res}\n"));
                    }
                    out.push_str(&format!("\taddq {arg1}, {res}\n"));
                }
                "*" => {
                    if arg0 != res {
                        out.push_str(&format!("\tmovq {arg0}, {res}\n"));
                    }
                    out.push_str(&format!("\timulq {arg1}, {res}\n"));
                }
                "-" => {
                    if arg0 != res {
                        out.push_str(&format!("\tmovq {arg0}, {res}\n"));
                    }
                    out.push_str(&format!("\tsubq {arg1}, {res}\n"));
                }
                "/" => {
                    out.push_str(&format!("\tmovq {arg0}, %rax\n"));
                    out.push_str("\tcqto\n");
                    out.push_str(&format!("\tidivq {arg1}\n"));
                    if res != "%rax " {
                        out.push_str(&format!("\tmovq %rax, {res}\n"));
                    }
                }
                "%" => {
                    out.push_str(&format!("\tmovq {arg0}, %rax\n"));
                    out.push_str("\tcqto\n");
                    out.push_str(&format!("\tidivq {arg1}\n"));
                    if res != "%rdx " {
                        out.push_str(&format!("\tmovq %rdx, {res}\n"));
                    }
                }
                _ => {
                    let setcc_insn = match &*op_var.name {
                        "<" => "setl",
                        "<=" => "setle",
                        ">" => "setg",
                        ">=" => "setge",
                        "==" => "sete",
                        "!=" => "setne",
                        _ => panic!("Unknown intrinsic {op_var}!"),
                    };
                    // We use 'al' below, which means the lower bytes of 'rax'

                    // Clear all bits of rax
                    out.push_str("\txor %rax, %rax\n");

                    out.push_str(&format!("\tmovq {arg0}, %rdx\n"));
                    out.push_str(&format!("\tcmpq {arg1}, %rdx\n"));

                    // Set lowest byte of 'rax' to comparison result
                    out.push_str(&format!("\t{setcc_insn} %al\n"));
                    if res != "%rax" {
                        out.push_str(&format!("\tmovq %rax, {res}\n"));
                    }
                }
            }
        }
    }
    out.push_str(&format!("\tmovq {res}, {}\n", locals.get_ref(output_var)));
}

#[derive(Debug)]
struct Locals {
    stack_used: i64,                         // Bytes
    var_to_location: HashMap<IrVar, String>, // Assembly reference as string
}

impl Locals {
    pub fn new(instructions: &Vec<IrInstruction>) -> Self {
        let ir_vars = Self::get_all_ir_vars(instructions);

        let mut stack_used = 0;
        let mut var_to_location = HashMap::new();

        for var in ir_vars {
            stack_used += 8;
            var_to_location.insert(var, format!("-{stack_used}(%rbp)"));
        }

        Self {
            var_to_location,
            stack_used,
        }
    }

    pub fn get_ref(&self, var: &IrVar) -> &str {
        self.var_to_location
            .get(var)
            .expect("Tried to use non-existant var in assembly generation!")
    }

    pub fn stack_used(&self) -> i64 {
        self.stack_used
    }

    fn get_all_ir_vars(instructions: &Vec<IrInstruction>) -> Vec<IrVar> {
        let mut var_set = HashSet::new();
        let globals = IrVar::new_global_types()
            .into_keys()
            .collect::<HashSet<IrVar>>();

        for instruction in instructions {
            for var in instruction.get_vars() {
                if !globals.contains(&var) {
                    var_set.insert(var);
                }
            }
        }

        let mut var_vec = var_set.into_iter().collect::<Vec<IrVar>>();
        var_vec.sort();
        var_vec
    }
}
