use std::sync::mpsc::Receiver;

use crate::server::record::Record;

pub fn db_open(write_rx: Receiver<Record>) {
    // Create an in-memory storage structure
    let mut storage: Vec<Record> = Vec::new();

    // Receive write operations from the server
    for received in write_rx {
        storage.push(received);
        println!("Total records in storage: {}", storage.len());
    }
}
