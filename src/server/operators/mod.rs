mod select;

use serde::{Deserialize, Serialize};

use crate::server::record::Record;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Op {
    Select(select::Select),
    Write(Record),
}
