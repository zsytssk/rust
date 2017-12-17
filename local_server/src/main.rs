extern crate futures;
extern crate hyper;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};

use futures::future::Future;

use std::ascii::AsciiExt;
use futures::Stream;
use hyper::{Body, Chunk};

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    server.run().unwrap();
}

struct HelloWorld;

const PHRASE: &'static str = "Hello, world!";
impl Service for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_body(PHRASE),
        ))
    }
}

struct Echo;

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Method::Get, "/echo") => {
                // let mapping = req.body().map(to_uppercase as fn(Chunk) -> Chunk);
                // let body: Box<Stream<Item=_, Error=_>>=Box::new(mapping);
                // response.set_body((req.body));

                Box::new(req.body().concat2().map(reverse))
            }
            (&Method::Get, "/") => Box::new(futures::future::ok(
                Response::new().with_body("Try POSTing data to /echo"),
            )),
            _ => Box::new(futures::future::ok(
                Response::new().with_status(StatusCode::NotFound),
            )),
        }
    }
}

fn to_uppercase(chunk: Chunk) -> Chunk {
    let uppered = chunk
        .iter()
        .map(|byte| byte.to_ascii_uppercase())
        .collect::<Vec<u8>>();
    Chunk::from(uppered)
}

fn reverse(chunk: Chunk) -> Response {
    let reversed = chunk.iter().rev().cloned().collect::<Vec<u8>>();

    Response::new().with_body(reversed)
}
