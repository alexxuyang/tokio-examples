extern crate futures;
//extern crate tokio;

use futures::{Future, future::*};
use std::error::Error;
use std::thread;
use std::time::Duration;

fn simple_func() -> Result<u32, Box<Error>> {
    Ok(100)
}

fn future_func() -> impl Future<Item = u32, Error = std::io::Error> {
    dbg!("future_func");
    thread::sleep(Duration::from_secs(1));
    ok(3)
}

fn future_func_square(i: u32) -> impl Future<Item = u32, Error = std::io::Error> {
    dbg!("future_func_square");
    thread::sleep(Duration::from_secs(5));
    ok(i * i)
}

fn main() {
    let a1 = simple_func().unwrap();
    dbg!(a1);

    let a2 = future_func()
        .then(|r| {
            let r = r.unwrap();
            future_func_square(r)
        })
        .map(|_| {
            dbg!("map");
        })
        .map_err(|_| {
            dbg!("map_err");
        });

    tokio::run(a2);
}
