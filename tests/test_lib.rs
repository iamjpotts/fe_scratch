use std::future::Future;
use std::io::Error;
use std::path::PathBuf;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;
use std::task::{Context, Poll};
use hyper::{Body, Client, Request, Uri};
use hyper::client::connect::{Connected, Connection};
use hyper::rt::Executor;
use hyper::service::Service;
use pin_project_lite::pin_project;
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use fe_scratch::add;

#[derive(Clone, Default)]
struct PipeConnector;

impl Unpin for PipeConnector {}

impl Service<Uri> for PipeConnector {
    type Response = PipeStream;
    type Error = std::io::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Uri) -> Self::Future {
        println!("Call {}", req);

        let pipe_name_b64= req.host()
            .unwrap();

        let pipe_name_bytes = base64::decode(pipe_name_b64)
            .unwrap();

        let pipe_name = String::from_utf8(pipe_name_bytes)
            .unwrap();

        println!("Call pipe {}", pipe_name);

        let path = PathBuf::from(pipe_name);

        let future = async move {
            Ok(PipeStream {
                file: File::open(path).await?
            })
        };

        Box::pin(future)
    }

}

pin_project! {
    struct PipeStream {
        #[pin]
        file: File
    }
}

impl AsyncRead for PipeStream {

    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
        self.project().file.poll_read(cx, buf)
    }

}

impl AsyncWrite for PipeStream {

    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize, Error>> {
        self.project().file.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        self.project().file.poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        self.project().file.poll_shutdown(cx)
    }

}

impl Connection for PipeStream {
    fn connected(&self) -> Connected {
        Connected::new()
    }
}


#[test]
fn adds_together() {
    assert_eq!(13, add(9, 4));
}

#[test]
#[ignore]
fn fails() {
    assert_eq!(11, 13);
}

#[tokio::test]
async fn sewer() {
    let pipe_name = "\\\\.\\pipe\\docker_engine";

    println!("Opening {}", pipe_name);

    let pipe = File::open(&pipe_name)
        .await
        .unwrap();

    drop(pipe);

    println!("Done.");
}

struct Foo {

}

type BoxSendFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

impl Executor<BoxSendFuture> for Foo {
    fn execute(&self, fut: BoxSendFuture) {
        tokio::task::spawn(fut);
    }
}

#[tokio::test]
async fn bar() {
    let pipe_name = "\\\\.\\pipe\\docker_engine";

    let pipe_name_b64 = base64::encode(pipe_name);

    let pipe_url = url::Url::from_str(&format!("x-pipe://{}", pipe_name_b64))
        .unwrap();

    let pipe_url_str = pipe_url.to_string();

    println!("Pipe url: {}", pipe_url_str);

    let client: Client<PipeConnector, Body> = Client::builder()
        .executor(Foo {})
        .pool_max_idle_per_host(123)
        .build::<_, Body>(PipeConnector::default());

    let request = Request::get(format!("{}/version", pipe_url_str))
        .body(hyper::Body::empty())
        .unwrap();

    let response = client.request(request)
        .await
        .unwrap();

    let body = hyper::body::to_bytes(response.into_body())
        .await
        .unwrap();

    let body_text = String::from_utf8(body.to_vec())
        .unwrap();

    println!("{}", body_text);
}