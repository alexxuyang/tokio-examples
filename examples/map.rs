extern crate futures;
extern crate tokio;

use futures::Future;
use futures::future;
use tokio::prelude::*;
use std::fmt;

struct HelloWorld;

impl Future for HelloWorld {
    type Item = String;
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        Ok(Async::Ready((String::from("hello world!"))))
    }
}

fn main() {
    let hw = HelloWorld.map(|v| println!("{}", v));
    tokio::run(hw);
}
