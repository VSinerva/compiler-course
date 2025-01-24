mod ast;
mod parser;
mod token;
mod tokenizer;

pub fn compile(code: &str) {
    let tokens = tokenizer::tokenize(code);
    parser::parse(&tokens);
}
