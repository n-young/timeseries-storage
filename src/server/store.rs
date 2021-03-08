use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use crate::server::record::Record;

pub fn db_open(write_rx: Receiver<Record>) {
    // Create an in-memory storage structure
    let mut index: HashMap<String, Vec<Record>> = HashMap::new();

    // Receive write operations from the server
    for received in write_rx {
        let key: String = received.get_key();
        match index.get_mut(&key) {
            Some(v) => {
                println!("Received a familiar key!");
                v.push(received);
            }
            None => {
                println!("First time seeing this key.");
                index.insert(key, vec![received]);
            }
        }
    }
}
