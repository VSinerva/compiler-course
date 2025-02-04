use std::io;

use interpreter::interpret;
use parser::parse;
use symtab::SymTab;
use tokenizer::tokenize;
use type_checker::type_check;

mod ast;
mod interpreter;
mod parser;
mod symtab;
mod token;
mod tokenizer;
mod type_checker;
mod variable;

pub fn compile(code: &str) {
    let tokens = tokenize(code);
    let ast = parse(&tokens);
    type_check(&ast, &mut SymTab::new_type_table());
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
