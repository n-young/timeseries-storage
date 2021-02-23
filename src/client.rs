use bincode::{deserialize_from, serialize_into};
use std::io::*;
use std::net::TcpStream;

pub fn from_stdin() {
    let mut stream = TcpStream::connect("127.0.0.1:12345").unwrap();
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        serialize_into(&mut stream, &line).unwrap();
        let responses: String = deserialize_from(&mut stream).unwrap();
        println!("{}", responses);
    }
}
