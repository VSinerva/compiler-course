use std::io;

use interpreter::interpret;
use ir::IrInstruction;
use ir_generator::generate_ir;
use parser::parse;
use symtab::SymTab;
use tokenizer::tokenize;
use type_checker::type_check;

mod ast;
mod interpreter;
mod ir;
mod ir_generator;
mod parser;
mod symtab;
mod token;
mod tokenizer;
mod type_checker;
mod variable;

pub fn compile(code: &str) -> Vec<IrInstruction> {
    let tokens = tokenize(code);
    let mut ast = parse(&tokens);
    type_check(&mut ast, &mut SymTab::new_type_table());
    generate_ir(&ast)
}

pub fn start_compiler() {
    let lines = io::stdin().lines();
    for line in lines.map_while(Result::ok) {
        for instruction in compile(&line) {
            println!("{instruction}");
        }
    }
}

pub fn start_interpreter() {
    let lines = io::stdin().lines();
    #[allow(clippy::manual_flatten)]
    for line in lines {
        if let Ok(code) = line {
            let tokens = tokenize(&code);
            let ast = parse(&tokens);

            let val = interpret(&ast, &mut SymTab::new_val_table());
            println!("{}", val);
        }
    }
}
