#[macro_use]
extern crate futures;
extern crate tokio;

use futures::future;
use futures::Stream;
use tokio::timer::Delay;
use tokio::prelude::*;
use std::time::{Duration, Instant};
use std::ops::Add;

fn main() {
    let fu = future::lazy(|| {
        for i in 0..4 {
            tokio::spawn(future::lazy(move || {
                println!("hello world from {}", i);
                Ok(())
            }));
        }

        Ok(Async::Ready(()))
    });

    tokio::run(fu.map(|_|{}));
}