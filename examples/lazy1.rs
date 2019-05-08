use futures::future;

fn main() {
    let fu = future::lazy(||{
        println!("hello world!");
        Ok(())
    });

    tokio::run(fu);
}