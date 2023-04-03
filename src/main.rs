use std::net::TcpStream;
use std::thread;
use std::time::Duration;
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
            Ok(stream) => {
                // TODO: track handles so that they can be joined on shutdown
                let _handle = thread::spawn(|| handle_client(stream));
            }
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

        match reader.read_line(&mut msg) {
            Ok(_) => {
                msg = msg.trim().to_owned();

                println!("msg: {:?}", msg);
                if msg.is_empty() {
                    break;
                }

                match parse_msg(&msg.trim()) {
                    Some(resp_type) => match resp_type {
                        RespType::BulkString(_len) => {
                            println!("type: bulk string");

                            let mut cmd: String = Default::default();
                            reader.read_line(&mut cmd).unwrap();

                            cmd = cmd.trim().to_owned();

                            println!("cmd: {:?}", cmd);

                            let res = match cmd.as_str() {
                                "ping" => "+PONG",
                                _ => "+OK",
                            };

                            println!("res: {}", res);

                            writer
                                .write_all(format!("{}\r\n", res).as_bytes())
                                .expect("Thought I could write back!?");
                            writer.flush().expect("Couldn't flush");
                        }
                        _ => {
                            println!("resp: {:?} not supported yet", msg)
                        }
                    },
                    None => {
                        println!("resp: {:?} not recognized", msg)
                    }
                };

                println!("\n");
            }
            Err(e) => {
                println!("couldn't read line: {}", e);
                break;
            }
        }
    }
}
