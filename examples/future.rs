extern crate futures;
extern crate tokio;

use futures::Future;
use futures::future;

fn main() {
    let a = future::ok::<u32, u32>(1);
    let b = a.wait().unwrap();
    dbg!(b);

    let c = future::err::<u32, u32>(10);
    let d = c.wait().err().unwrap();

    dbg!(d);
}