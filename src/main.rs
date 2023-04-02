use std::net::TcpStream;

use std::{
    io::{BufRead, BufReader, BufWriter, Read, Write},
    net::TcpListener,
};

enum Command {
    PING,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(stream: TcpStream) {
    println!("accepted new connection");

    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    for msg in reader.lines() {
        match msg {
            Ok(str) => {
                let response = match str.to_lowercase().as_str() {
                    "ping" => "+PONG",
                    _ => "+OK",
                };

                println!("msg: {}\nres: {}\n", str, response);

                writer
                    .write(format!("{}\r\n", response).as_bytes())
                    .expect("Thought I could write back!?");
                writer.flush().expect("Couldn't flush");
            }
            Err(e) => {
                println!("error receiving: {}", e);
            }
        }
    }
}
