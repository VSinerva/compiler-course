use std::io;

use interpreter::interpret;
use parser::parse;
use tokenizer::tokenize;

mod ast;
mod interpreter;
mod parser;
mod token;
mod tokenizer;
mod value;

pub fn compile(code: &str) {
    let tokens = tokenizer::tokenize(code);
    parser::parse(&tokens);
}

pub fn start_interpreter() {
    let lines = io::stdin().lines();

    for line in lines {
        if let Ok(code) = line {
            println!("{}", interpret(&parse(&tokenize(&code))));
        }
    }
}
