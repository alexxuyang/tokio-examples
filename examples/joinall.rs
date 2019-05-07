extern crate futures;

use futures::{future, Future};
use std::io;
use std::sync::Arc;

fn get_message() -> impl Future<Item = String, Error = io::Error> {
    future::ok(String::from("kaka"))
}

fn print_multi() -> impl Future<Item = (), Error = io::Error> {
    let name = Arc::new("carl".to_string());

    let futures: Vec<_> = (0..10).map(|_| {
        // Clone the `name` handle, this allows multiple concurrent futures
        // to access the name to print.
        let name = name.clone();

        get_message()
            .and_then(move |message| {
                println!("Hello {}, {}", name, message);
                Ok(())
            })
    })
    .collect();

    future::join_all(futures)
        .map(|_| ())
}

fn main() {
    let fu = print_multi();
    tokio::run(fu.map_err(|_| {}));
}