//mod predicate;
mod error;
mod server;
mod client;

use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::env;

use error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "client" {
        client::from_stdin()
    } else if args[1] == "server" {
        server::server()
    }
}

