extern crate futures;
extern crate tokio;

use futures::future;
use futures::Future;
use tokio::prelude::*;

fn fibonacci() -> impl Stream<Item=u32, Error=()> {
    stream::unfold((1, 1), |(curr, next)| {
        let a = curr + next;
        Some(Ok((curr, (next, a))))
    })
}

fn main() {
    let fu = fibonacci()
        .take(12)
        .for_each(|v|{
            println!("{}", v);
            Ok(())
        });

    tokio::run(fu);
}