use json;
use std::{
    io::prelude::*,
    net::{IpAddr, TcpListener, TcpStream},
    thread,
};

pub fn start(address: IpAddr, port: u16) {
    let address_string = format!("{address}:{port}");

    let listener = TcpListener::bind(&address_string).unwrap_or_else(|error| {
        panic!("Problem starting listener: {error:?}");
    });

    println!("Listening for connections on {address_string}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = String::new();
    stream
        .read_to_string(&mut buffer)
        .expect("Unable to read from buffer!");

    let json_request = json::parse(&buffer).expect("Malformed JSON!");

    match json_request["command"].as_str().unwrap() {
        "ping" => println!("ping"),
        "compile" => {
            let program = &json_request["code"].as_str().unwrap();
            println!("compile code:\n\n{program}\n");
        }
        _ => panic!("Unexpected command!"),
    }
}
