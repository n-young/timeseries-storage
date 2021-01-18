use crate::server::operators::Op;
use crate::{
    error::Error,
    server::record::Record,
};

pub fn execute(operation: Op) -> Result<Option<Vec<Record>>, Error> {
    let _ = operation;
    Ok(None)
}

