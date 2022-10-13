use std::future::Future;
use std::io::Error;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};
use hyper::Uri;
use hyper::client::connect::{Connected, Connection};
use hyper::rt::Executor;
use hyper::service::Service;
use pin_project_lite::pin_project;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

#[derive(Clone, Default)]
pub struct PipeConnector;

impl Unpin for PipeConnector {}

impl Service<Uri> for PipeConnector {
    type Response = PipeStream;
    type Error = Error;

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
            let path_str = path.to_str().unwrap();
            println!("Opening {}", path_str);

            let file_result = OpenOptions::new()
                .create(false)
                .read(true)
                .write(true)
                .open(&path_str)
                .await;

            if let Err(e) = &file_result {
                println!("Failed to open {:?}: {}", path_str, e);
            }
            else {
                println!("Opened {:?}", path_str);
            }

            Ok(PipeStream {
                file: file_result?
            })
        };

        Box::pin(future)
    }

}

pin_project! {
    pub struct PipeStream {
        #[pin]
        file: File
    }
}

impl AsyncRead for PipeStream {

    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
        println!("Poll read");
        self.project().file.poll_read(cx, buf)
    }

}

impl AsyncWrite for PipeStream {

    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize, Error>> {
        println!("Poll write");
        self.project().file.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        println!("Poll flush");
        self.project().file.poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        println!("Poll shutdown");
        self.project().file.poll_shutdown(cx)
    }

}

impl Connection for PipeStream {
    fn connected(&self) -> Connected {
        println!("Connected");
        Connected::new()
    }
}

pub struct Foo {

}

type BoxSendFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

impl Executor<BoxSendFuture> for Foo {
    fn execute(&self, fut: BoxSendFuture) {
        tokio::task::spawn(fut);
    }
}
