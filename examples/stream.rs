#[macro_use]
extern crate futures;
extern crate tokio;

use futures::future;
use futures::Future;
use tokio::prelude::*;
use tokio::timer::Interval;
use std::fmt;
use std::time::Duration;

struct Fibonacci {
    interval: Interval,
    curr: u32,
    next: u32,
}

impl Fibonacci {
    fn new(itv: Interval) -> Self {
        Fibonacci {
            interval: itv,
            curr: 1,
            next: 1,
        }
    }
}

impl Stream for Fibonacci {
    type Item = u32;
    type Error = ();

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        try_ready!(self.interval.poll().map_err(|_|{}));

        let a = self.curr + self.next;
        self.curr = self.next;
        self.next = a;

        Ok(Async::Ready(Some(a)))
    }
}

struct Display10<T> {
    stream: T,
    curr: usize,
}

impl<T> Display10<T> {
    fn new(stream: T, curr: usize) -> Self {
        Display10 {
            stream,
            curr,
        }
    }
}

impl<T> Future for Display10<T>
where
    T: Stream,
    T::Item: fmt::Display,
{
    type Item = ();
    type Error = T::Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        while self.curr < 10 {
            let v = match try_ready!(self.stream.poll()) {
                Some(v) => v,
                None => break,
            };

            println!("#{}: {}", self.curr, v);
            self.curr += 1;
        }

        Ok(Async::Ready(()))
    }
}

fn main() {
    let fu = Display10::new(Fibonacci::new(Interval::new_interval(Duration::from_secs(1))), 0);
    tokio::run(fu);
}