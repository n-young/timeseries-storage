use std::net::{TcpStream};
use std::io::*;
use bincode::{serialize_into, deserialize_from};

pub fn from_stdin() {
    let mut stream = TcpStream::connect("127.0.0.1:12345").unwrap();
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        serialize_into(&mut stream, &line).unwrap();
        let responses: String = deserialize_from(&mut stream).unwrap();
        println!("{}", responses);
    }
}
