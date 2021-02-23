use crate::server::operators::Op;
use crate::{error::Error, server::record::Record};
use std::sync::mpsc::Sender;

pub fn execute(operation: Op, tx: &Sender<Record>) -> Result<Option<Vec<Record>>, Error> {
    match operation {
        Op::Write(record) => execute_write(record, tx),
        _ => Ok(None),
    }
}

fn execute_write(record: Record, tx: &Sender<Record>) -> Result<Option<Vec<Record>>, Error> {
    let record_dup = record.clone();
    tx.send(record).unwrap();
    Ok(Some(vec![record_dup]))
}
