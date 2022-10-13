use std::str::FromStr;
use hyper::{Body, Client, Request};
use hyper::client::HttpConnector;
use tokio;
use fe_scratch::foo::{Foo, PipeConnector};

#[tokio::main]
async fn main() {
    println!("Hello, world.");

    let client: Client<HttpConnector, Body> = Client::builder()
        .pool_max_idle_per_host(123)
        .build::<_, Body>(HttpConnector::default());

    println!("Building request");

    let request = Request::get("http://localhost:2375/version")
        .body(hyper::Body::empty())
        .unwrap();

    println!("Getting response");

    let response = client.request(request)
        .await
        .unwrap();

    println!("Reading body");

    let body = hyper::body::to_bytes(response.into_body())
        .await
        .unwrap();

    println!("Converting body to a string");

    let body_text = String::from_utf8(body.to_vec())
        .unwrap();

    println!("{}", body_text);

    println!("Goodbye, world.");
}