//mod predicate;
mod client;
mod error;
mod server;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "client" {
        client::from_stdin()
    } else if args[1] == "server" {
        server::server()
    }
}
