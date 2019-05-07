extern crate futures;
extern crate tokio;

use futures::Future;
use futures::future;
use std::thread;
use std::time;
use std::error::Error;

fn main() {
    let future1 = future::lazy(|| {
        thread::sleep(time::Duration::from_secs(5));
        future::ok::<char, ()>('a')
    });

    let future2 = future::lazy(|| {
        thread::sleep(time::Duration::from_secs(3));
        future::ok::<char, ()>('b')
    });

    let (value, last_future) = future1.select(future2).wait().ok().unwrap();

    let a = value;
    let b = last_future.wait().unwrap();

    assert_eq!(a, 'a');
    assert_eq!(b, 'b');

    dbg!(a);
    dbg!(b);
}