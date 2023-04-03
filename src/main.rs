use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpListener,
};
use threadpool::ThreadPool;

use crate::redis::{format_response, parse_command, RespType};
use crate::utils::*;

mod redis;
mod threadpool;
mod utils;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:6379").unwrap();
    let thread_pool = ThreadPool::new(8);
    let server = Arc::new(Mutex::new(redis::Server::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // TODO: track handles so that they can be joined to do a safe shutdown
                let server = Arc::clone(&server);
                let _handle = thread_pool.execute(move || handle_client(stream, server));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(stream: TcpStream, server: Arc<Mutex<redis::Server>>) {
    println!("accepted new connection");

    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    let mut msg: String = Default::default();

    loop {
        msg.clear();

        // TODO: read byte counts instead of lines to make faster
        match reader.read_line(&mut msg) {
            Ok(_) => match handle_message(&msg, &server, &mut reader) {
                Ok(response) => {
                    writer.write(response.as_bytes()).unwrap();
                    writer.flush().unwrap();
                }
                Err(e) => {
                    println!("error: {}", e);
                    break;
                }
            },
            Err(e) => {
                println!("couldn't read line: {}", e);
                break;
            }
        }
    }
}

fn handle_message(
    msg: &str,
    server: &Arc<Mutex<redis::Server>>,
    reader: &mut BufReader<&TcpStream>,
) -> Result<String, String> {
    // note: type of message is always Array<BulkString>
    let msg = msg.trim().to_owned();
    if msg.is_empty() {
        // TODO: confirm what this means, for now just ignore it
        return Err("empty message".to_string());
    }

    println!("msg: {:?}", msg);

    let arr_size = msg[1..].parse::<usize>().unwrap();
    let strings = read_bulk_strings(arr_size, reader);

    match &parse_command(&strings) {
        Ok(command) => {
            let result = server
                // TODO: consider only locking if store is actually used
                .lock()
                .unwrap()
                .exec(command);

            Ok(format_response(&result))
        }

        Err(e) => Ok(format_response(&RespType::Error(e.to_string()))),
    }
}
