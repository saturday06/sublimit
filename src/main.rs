#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;

use std::io;
use futures::future::FutureResult;
use hyper::{Get, Post, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};

struct Echo;

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        futures::future::ok(match req.method() {
            &Get => {
                let mut res = Response::new();
                if let Some(len) = req.headers().get::<ContentLength>() {
                    res.headers_mut().set(len.clone());
                }
                res.with_body(req.body())
            },
            _ => {
                Response::new().with_status(StatusCode::NotFound)
            }
        })
    }
}

fn start() -> Result<i32, String> {
    pretty_env_logger::init().map_err(|e|
        e.to_string()
    )?;
    let addr = "127.0.0.1:1337".parse().map_err(|e|
        format!("failed to parse address: {}", e)
    )?;
    let server = Http::new().bind(&addr, || Ok(Echo)).map_err(|e|
        e.to_string()
    )?;
    let local_addr = server.local_addr().map_err(|e|
        e.to_string()
    )?;
    println!("Listening on http://{} with 1 thread.", local_addr);
    server.run().map_err(|_|
        "なんやて工藤"
    )?;
    return Ok(0)
}

fn main() {
    match start() {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {}", err),
    }
}
