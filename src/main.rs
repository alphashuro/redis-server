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

enum RespType {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(usize),
    Array(Vec<String>),
}

fn parse_msg(str: &str) -> Option<RespType> {
    let mut chars = str.chars();

    let resp_type_indicator = chars.next();

    match resp_type_indicator {
        Some('+') => Some(RespType::SimpleString(chars.collect())),
        Some('-') => Some(RespType::Error(chars.collect())),
        Some(':') => Some(RespType::Integer(
            (chars.collect::<String>().trim()).parse::<i64>().unwrap(),
        )),
        Some('$') => {
            let count_chars = chars.collect::<String>();
            let count = count_chars.trim().parse::<usize>().unwrap();

            Some(RespType::BulkString(count))
        }
        Some('*') => Some(RespType::Array(vec![])),
        Some(_) | None => None,
    }
}

fn handle_client(stream: TcpStream) {
    println!("accepted new connection");

    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    loop {
        let mut msg: String = Default::default();

        reader.read_line(&mut msg).unwrap();

        println!("msg: {}", msg);

        let resp_type = parse_msg(&msg).unwrap();

        match resp_type {
            RespType::BulkString(_len) => {
                let mut cmd: String = Default::default();

                reader.read_line(&mut cmd).unwrap();

                println!("cmd: {:?}", cmd);

                let res = match cmd.as_str().trim() {
                    "ping" => "+PONG",
                    _ => "+OK",
                };

                println!("res: {}\n", res);

                writer
                    .write(format!("{}\r\n", res).as_bytes())
                    .expect("Thought I could write back!?");
                writer.flush().expect("Couldn't flush");
            }
            _ => {}
        }
    }
}
