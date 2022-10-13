use std::future::Future;
use std::io::Error;
use std::path::PathBuf;
use std::pin::Pin;
use std::str::FromStr;
use std::task::{Context, Poll};
use hyper::{Body, Client, Request, Uri};
use hyper::client::connect::{Connected, Connection};
use hyper::rt::Executor;
use hyper::service::Service;
use pin_project_lite::pin_project;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use fe_scratch::add;


#[test]
fn adds_together() {
    assert_eq!(13, add(9, 4));
}

#[test]
#[ignore]
fn fails() {
    assert_eq!(11, 13);
}

