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

struct Display<T>(T);

impl<T> Future for Display<T>
where
    T: Future,
    T::Item: fmt::Display,
{
    type Item = ();
    type Error = T::Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        let v = match self.0.poll() {
            Ok(Async::Ready(v)) => v,
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err(e) => return Err(e),
        };

        println!("{}", v);
        Ok(Async::Ready(()))
    }
}

fn main() {
    let hw = Display(HelloWorld);
    tokio::run(hw);
}