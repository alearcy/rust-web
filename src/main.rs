#![allow(dead_code)] //omit warning in the compiler
use server::Server;

mod http;
mod server;

fn main() {
    let server = Server::new("127.0.0.1:8000".to_string());
    server.run();

    // notes about strings
    // let string = String::from("alessandro");
    // let string_slice = &string[5...] doesnt't mean 'give me from the fifth character to the end' but 'give me the fifth byte'
}
