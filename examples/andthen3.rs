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

    let future = TcpStream::connect(&addr)
        .and_then(|socket| {
            io::write_all(socket, b"hello world")
        })
        .and_then(|(socket, _)| {
            // read exactly 11 bytes
            io::read_exact(socket, vec![0; 11])
        })
        .and_then(|(socket, buf)| {
            println!("got {:?}", buf);
            Ok(())
        })
        .map_err(|_|{});

    tokio::run(future);
}