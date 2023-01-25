use std::{
    io::{BufRead, BufReader, BufWriter, Read, Write},
    net::TcpListener,
};

enum Command {
    PING,
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");

                let reader = BufReader::new(&stream);
                let mut writer = BufWriter::new(&stream);

                for msg in reader.lines() {
                    match msg {
                        Ok(str) => {
                            let response = match str.to_lowercase().as_str() {
                                "+ping" => "+PONG",
                                _ => "+PONG",
                            };

                            println!("msg: {}\nres: {}\n", str, response);

                            writer
                                .write_all(format!("{}\r\n", response).as_bytes())
                                .expect("Thought I could write back!?");
                            writer.flush().expect("Couldn't flush");
                        }
                        Err(e) => {
                            println!("error receiving: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
