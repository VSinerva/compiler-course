use std::io;

use interpreter::Interpreter;
use parser::parse;
use tokenizer::tokenize;

mod ast;
mod interpreter;
mod parser;
mod symtab;
mod token;
mod tokenizer;
mod value;

pub fn compile(code: &str) {
    let tokens = tokenizer::tokenize(code);
    parser::parse(&tokens);
}

pub fn start_interpreter() {
    let lines = io::stdin().lines();
    #[allow(clippy::manual_flatten)]
    for line in lines {
        if let Ok(code) = line {
            let tokens = tokenize(&code);
            let ast = parse(&tokens);

            let mut interpreter = Interpreter::new();
            let val = interpreter.interpret(&ast);
            println!("{}", val);
        }
    }
}
