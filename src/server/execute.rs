use crate::server::operators::{Op, Select};
use crate::{error::Error, server::record::Record};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct SelectRequest {
    pub statement: Select,
    result_tx: Sender<Vec<Record>>,
}

impl SelectRequest {
    fn new(s: Select) -> (Self, Receiver<Vec<Record>>) {
        let (tx, rx): (Sender<Vec<Record>>, Receiver<Vec<Record>>) = channel();
        (
            SelectRequest {
                statement: s,
                result_tx: tx,
            },
            rx,
        )
    }

    pub fn reply(&self, r: Vec<Record>) {
        self.result_tx.send(r).unwrap();
    }
}

pub fn execute(
    operation: Op,
    read_tx: &Sender<SelectRequest>,
    write_tx: &Sender<Record>,
) -> Result<Option<Vec<Record>>, Error> {
    match operation {
        Op::Write(record) => execute_write(record, write_tx),
        Op::Select(statement) => execute_select(statement, read_tx),
    }
}

fn execute_select(
    statement: Select,
    tx: &Sender<SelectRequest>,
) -> Result<Option<Vec<Record>>, Error> {
    let (request, rx) = SelectRequest::new(statement);
    tx.send(request).unwrap();
    let result = rx.recv().unwrap();
    println!("Received result: {:?}", result);
    Ok(Some(result))
}

fn execute_write(record: Record, tx: &Sender<Record>) -> Result<Option<Vec<Record>>, Error> {
    let record_dup = record.clone();
    tx.send(record).unwrap();
    Ok(Some(vec![record_dup]))
}
