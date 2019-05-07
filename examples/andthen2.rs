extern crate futures;
extern crate tokio;
extern crate bytes;

use tokio::net::TcpStream;
use tokio::io;
use tokio::prelude::*;
use futures::Future;
use futures::future;

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let conn = TcpStream::connect(&addr)
        .and_then(|socket| io::write_all(socket, b"hello world!"))
        .map(|_| println!("write complete"))
        .map_err(|_| println!("failed"));

    tokio::run(conn);
}
