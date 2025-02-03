use std::env;

mod compiler;
mod server;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(flag) = args.get(1) {
        if flag == "-i" {
            compiler::start_interpreter();
        }
    } else {
        server::start("::".parse().unwrap(), 3000);
    }
}
