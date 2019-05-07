#[macro_use]
extern crate futures;
extern crate tokio;
extern crate bytes;

use futures::{Future, future};
use tokio::prelude::*;
use bytes::{Bytes, Buf};
use std::io::{self, Cursor};
use tokio::net::tcp::ConnectFuture;
use tokio::net::TcpStream;

enum HelloWorld {
    Connecting(ConnectFuture),
    Connected(TcpStream, Cursor<Bytes>),
}

impl Future for HelloWorld {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        use self::HelloWorld::*;

        loop {
            match self {
                Connecting(conn) => {
                    let socket = try_ready!(conn.poll());
                    let data = Cursor::new(Bytes::from_static(b"hello world!"));
                    *self = Connected(socket, data);
                },
                Connected(ref mut socket, ref mut data) => {
                    while data.has_remaining() {
                        socket.write_buf(data);
                    }
                    return Ok(Async::Ready(()));
                },
            }
        }
    }
}

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let conn = TcpStream::connect(&addr);
    let hw = HelloWorld::Connecting(conn);
    tokio::run(hw.map_err(|e| {println!("{}", e)}));
}