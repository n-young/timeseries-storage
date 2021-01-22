use std::{
    net::{TcpListener, TcpStream, Shutdown},
    thread,
    io
};

use bincode::{serialize_into, deserialize_from};

use crate::{
    server::{
        execute::execute,
        record::Record,
    },
};

fn postprocess(result: Vec<Record>) -> String {
    let _ = result;
    String::from("Processed some records")
}

fn handle_tcp_connection(mut stream: TcpStream) {
    let addr = stream.peer_addr().unwrap();

    while match deserialize_from::<_, String>(&mut stream) {
        Ok(data) => {
            if let Ok(op) = serde_json::from_str(&data) {
                let response = match execute(op) {
                    Ok(Some(result)) => postprocess(result),
                    Ok(None) => String::from("Operation completed"),
                    Err(error) => format!("Error: {}", error),
                };
                serialize_into(&mut stream, &response).unwrap();
                true
            } else {
                let response = format!("Unrecognized input: {}", &data);
                serialize_into(&mut stream, &response).unwrap();
                true
            }
        },
        Err(_) => false,
    } {}

    println!("Terminating connection with {}", addr);
    match stream.shutdown(Shutdown::Both) {
        Ok(_) => println!("Connection terminated"),
        Err(err) => match err.kind() {
            io::ErrorKind::NotConnected => println!("Connection already terminated"),
            _ => panic!("Shutdown problem"),
        }
    }
}

pub fn server() {
    let listener = TcpListener::bind("127.0.0.1:12345").unwrap();
    for stream in listener.incoming() {
        match stream {
            Err(e) => println!("failed: {}", e),
            Ok(stream) => {
                thread::spawn(move || handle_tcp_connection(stream));
            }
        }
    }
}
