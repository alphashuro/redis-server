use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

/// Reads n bulk strings from the reader
/// TODO: figure out what BufReader's type arg should be
pub fn read_bulk_strings(n: usize, reader: &mut BufReader<&TcpStream>) -> Vec<String> {
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
