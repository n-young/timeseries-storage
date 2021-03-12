use std::{
    collections::HashMap,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread,
};

use crate::server::{execute::SelectRequest, operators::Select, record::Record};

struct Series {
    records: Mutex<Vec<Record>>,
}

impl Series {
    fn new(record: Record) -> Self {
        Series {
            records: Mutex::new(vec![record]),
        }
    }

    fn insert(&self, record: Record) {
        let mut v = self.records.lock().unwrap();
        v.push(record);
    }
}

fn db_read(read_rx: Receiver<SelectRequest>, index: Arc<Mutex<HashMap<String, Series>>>) {
    // Receive read operations from the server
    for request in read_rx {
        let statement = request.statement.clone();
        println!("Received statement: {:?}", statement);
        request.reply(Vec::new());
    }
}

fn db_write(write_rx: Receiver<Record>, index: Arc<Mutex<HashMap<String, Series>>>) {
    // Receive write operations from the server
    for received in write_rx {
        let key: String = received.get_key();
        let mut db = index.lock().unwrap();
        match db.get_mut(&key) {
            Some(v) => {
                println!("Received a familiar key!");
                v.insert(received);
            }
            None => {
                println!("First time seeing this key.");
                db.insert(key, Series::new(received));
            }
        }
    }
}

pub fn db_open(read_rx: Receiver<SelectRequest>, write_rx: Receiver<Record>) {
    // Create an in-memory storage structure
    let index = Arc::new(Mutex::new(HashMap::new()));

    // Set up separate r/w threads so that read operations don't block writes
    let read_index = Arc::clone(&index);
    let read_thr = thread::spawn(move || db_read(read_rx, read_index));
    let write_index = Arc::clone(&index);
    let write_thr = thread::spawn(move || db_write(write_rx, write_index));
    read_thr.join().unwrap();
    write_thr.join().unwrap();
}
