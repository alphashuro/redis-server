use std::net::TcpStream;
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpListener,
};
use threadpool::ThreadPool;

use crate::redis::run;

mod redis;
mod threadpool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let thread_pool = ThreadPool::new(16);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // TODO: track handles so that they can be joined on shutdown
                let _handle = thread_pool.execute(|| handle_client(stream));
            }
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
    writer.write("+OK".to_string().as_bytes()).unwrap();
    writer.flush().unwrap();

    let mut msg: String = Default::default();

    loop {
        msg.clear();

        // TODO: read byte counts instead of lines to make faster
        match reader.read_line(&mut msg) {
            Ok(_) => {
                // note: type of message is always Array<BulkString>
                msg = msg.trim().to_owned();
                println!("msg: {:?}", msg);

                if msg.is_empty() {
                    // TODO: confirm what this means, for now just ignore it
                    break;
                }

                let arr_size = msg[1..].parse::<usize>().unwrap();

                let strings = read_bulk_strings(arr_size, &mut reader);
                let response = run(&strings);

                println!("response: {:?}", response);

                writer.write(response.as_bytes()).unwrap();
                writer.flush().unwrap();
            }
            Err(e) => {
                println!("couldn't read line: {}", e);
                break;
            }
        }
    }
}

/// Reads n bulk strings from the reader
/// TODO: figure out what BufReader's type arg should be
fn read_bulk_strings(n: usize, reader: &mut BufReader<&TcpStream>) -> Vec<String> {
    let mut strings: Vec<String> = Vec::with_capacity(n);

    let mut buf = String::new();

    // each bulk string spans 2 lines
    // so we iterate every two lines
    // and only process the odd ones
    for i in 0..n * 2 {
        buf.clear();
        reader.read_line(&mut buf).unwrap();

        if i % 2 != 0 {
            strings.push(buf.trim().to_string());
        }
    }

    strings
}
