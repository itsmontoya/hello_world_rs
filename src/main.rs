#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate hyper;
extern crate futures_await as futures;
extern crate tokio_timer;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use futures::prelude::*;
use std::time::*;
use tokio_timer::*;

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
    server.run().unwrap();
}

const PHRASE: &'static str = "Hello, World!";

struct HelloWorld;

impl Service for HelloWorld {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    #[async]
    fn call(&self, _req: Request) -> Self::Future {
        println!("Hello world!");
        let timer = Timer::default();

        // Pretend the request takes 500ms
        let sleep = timer.sleep(Duration::from_millis(3000));
        sleep.wait();

        println!("Sending!");

        // We're currently ignoring the Request
        // And returning an 'ok' Future, which means it's ready
        // immediately, and build a Response with the 'PHRASE' body.
        futures::future::ok(Response::new()
            .with_header(ContentLength(PHRASE.len() as u64))
            .with_body(PHRASE))
    }
}
