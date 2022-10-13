use std::str::FromStr;
use hyper::{Body, Client, Request};
use tokio;
use fe_scratch::foo::{Foo, PipeConnector};

#[tokio::main]
async fn main() {
    println!("Hello, world.");

    let pipe_name = "\\\\.\\pipe\\docker_engine";

    let pipe_name_b64 = base64::encode(pipe_name);

    let pipe_url = url::Url::from_str(&format!("x-pipe://{}", pipe_name_b64))
        .unwrap();

    let pipe_url_str = pipe_url.to_string();

    println!("Pipe url: {}", pipe_url_str);

    let client: Client<PipeConnector, Body> = Client::builder()
        //.executor(Foo {})
        .pool_max_idle_per_host(123)
        .build::<_, Body>(PipeConnector::default());

    println!("Building request");

    let request = Request::get(format!("{}/version", pipe_url_str))
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