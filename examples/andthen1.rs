extern crate futures;
//extern crate tokio;

use futures::{Future, future::*};
use std::error::Error;

fn simple_func() -> Result<u32, Box<Error>> {
    Ok(100)
}

fn future_func() -> impl Future<Item = u32, Error = std::io::Error> {
    ok(3)
}

fn future_func_square(i: u32) -> impl Future<Item = u32, Error = std::io::Error> {
    ok(i * i)
}

fn main() {
    let a1 = simple_func().unwrap();
    dbg!(a1);

    let a2 = future_func()
        .and_then(|x| {
            future_func_square(x)
        })
        .and_then(|y| {
            dbg!(y);
            Ok(())
        })
        .map_err(|_| {});

    tokio::run(a2);
}
