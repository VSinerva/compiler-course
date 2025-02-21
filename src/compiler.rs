use std::io;

use assembly_generator::generate_assembly;
use interpreter::interpret;
use ir_generator::generate_ir;
use parser::parse;
use symtab::SymTab;
use tokenizer::tokenize;
use type_checker::type_check;

mod assembly_generator;
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

pub fn compile(code: &str) -> String {
    let tokens = tokenize(code);
    let mut ast = parse(&tokens);
    type_check(&mut ast, &mut SymTab::new_type_table());
    let ir = generate_ir(&ast);
    generate_assembly(&ir)
}

pub fn start_compiler() {
    let lines = io::stdin().lines();
    for line in lines.map_while(Result::ok) {
        println!();
        println!("{}", compile(&line));
        println!();
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
