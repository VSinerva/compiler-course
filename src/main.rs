mod compiler;
mod server;

fn main() {
    server::start("::".parse().unwrap(), 3000);
}
