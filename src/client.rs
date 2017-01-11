extern crate hyper;

use hyper::client::Body;
use hyper::Client;

fn run () -> Result<hyper::client::Response,hyper::Error> {
    let a = vec![1];
    let b = Body::BufBody(&a,1);
    let client = Client::new();
    let mut builder = client.get("d");
    builder = builder.body(b);
    builder.send()
    // builder.body(b).send()
}

fn main() {
    let x = run().unwrap();
    println!("{:?}",x);
}