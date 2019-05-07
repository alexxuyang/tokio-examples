#[macro_use]
extern crate futures;
extern crate tokio;

use futures::Future;
use futures::future;
use tokio::net::TcpStream;
use tokio::net::tcp::ConnectFuture;
use tokio::prelude::*;

struct GetPeerAddr {
    connect: ConnectFuture,
}

impl Future for GetPeerAddr {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        match self.connect.poll() {
            Ok(Async::Ready(v)) => {
                println!("connect to peer: {}", v.peer_addr().unwrap());
                Ok(Async::Ready(()))
            },
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err(e) => {
                println!("fail to connect: {}", e);
                Ok(Async::Ready(()))
            },
        }
    }
}

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let conn = TcpStream::connect(&addr);
    let get_peer_addr = GetPeerAddr {
        connect: conn,
    };
    tokio::run(get_peer_addr);
}