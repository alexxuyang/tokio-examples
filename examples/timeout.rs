extern crate futures;
extern crate tokio;

use futures::future;
use futures::Future;
use tokio::timer::Delay;
use tokio::prelude::*;
use std::time::{Instant, Duration};

fn main() {
    let de = Instant::now() + Duration::from_millis(5000);
    let fu = Delay::new(de)
        .timeout(Duration::from_millis(1000))
        .and_then(|_| {
            println!("hello world!");
            Ok(())
        })
        .map_err(|e| {println!("{}", e)});

    tokio::run(fu);
}