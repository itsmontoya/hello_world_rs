#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate hyper;
extern crate pretty_env_logger;
extern crate futures_await as futures;
extern crate tokio_timer;

use tokio_timer::*;
use std::time::*;
use futures::prelude::*;
use futures::future::FutureResult;
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Service, Request, Response};

static __SPACE: &'static [u8] = b" ";
static __GREETING: &'static [u8] = b"Hello";
static __NAME: &'static [u8] = b"World";
static __PUNCTUATION: &'static [u8] = b"!";

fn main() {
    pretty_env_logger::init().unwrap();
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Hello)).unwrap();
    println!("Listening on http://{} with 1 thread.",
             server.local_addr().unwrap());
    server.run().unwrap();
}

struct Hello;

impl Service for Hello {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, _req: Request) -> Self::Future {
        let val = fooResp();
        // Create response from value
        let resp = Response::new()
            .with_header(ContentLength(val.len() as u64))
            .with_header(ContentType::plaintext())
            .with_body(val);
        // Return an OK future with the response
        futures::future::ok(resp)
    }
}

#[async]
fn fooResp() -> Result<String, i32> {
    // We'll create a set to add a bunch of recievers to.
    //  let mut rx_set = Vec::new();
    //rx_set.push(greeting())
    //rx_set.push(name())

    let mut rx_set = Vec::new();
    rx_set.push(greeting());
    rx_set.push(greeting());
    rx_set.push(greeting());

    let result = await!(futures::future::join_all(rx_set)).unwrap();
    let val = result[0] + result[1] + result[2];
    Ok(val.to_string())
}

type DBResult = Result<i32, i32>;

#[async]
fn greeting() -> DBResult {
    let timer = Timer::default();
    // Set a timeout that expires in 500 milliseconds
    let sleep = timer.sleep(Duration::from_millis(3000));

    await!(sleep);
    Ok(1)
}


#[async]
fn name() -> DBResult {
    let timer = Timer::default();
    // Set a timeout that expires in 500 milliseconds
    let sleep = timer.sleep(Duration::from_millis(3000));

    await!(sleep);
    Ok(2)
}
