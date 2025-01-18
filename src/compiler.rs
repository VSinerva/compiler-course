mod token;
mod tokenizer;

pub fn compile(code: &str) {
    tokenizer::tokenize(code);
}
