extern crate futures;

use futures::{stream, Future, Stream};
use std::io;
use futures::future::ok;

fn get_data() -> impl Future<Item = u32, Error = io::Error> {
    ok(10)
}

fn get_ok_data() -> impl Future<Item = Vec<u32>, Error = io::Error> {
    let mut dst = vec![];

    // Start with an unbounded stream that uses unit values.
    stream::repeat(())
        // Only take 10. This is how the for loop is simulated using a functional
        // style.
        .take(5)
        // The `fold` combinator is used here because, in order to be
        // functional, the state must be moved into the combinator. In this
        // case, the state is the `dst` vector.
        .fold(dst, |mut dst, _| {
            // Once again, the `dst` vector must be moved into the nested
            // closure.
            get_data().and_then(|item| {
                dst.push(item);

                // The state must be included as part of the return value, so
                // `dst` is returned.
                Ok(dst)
            })
        })
}

fn main() {
    let fu = get_ok_data()
        .and_then(|v| {
            println!("{:#?}", v);
            Ok(())
        })
        .map(|_| {})
        .map_err(|_| {});

    tokio::run(fu);
}