use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Record {
    name: String,
    labels: HashMap<String, String>,
    variables: HashMap<String, f64>,
    timestamp: DateTime<Utc>,
}

