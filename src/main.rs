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

static PHRASE: &'static [u8] = b"Hello World!";

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
        // Get long request value
        // I know I'm doing it wrong here, because this makes it synchonrous.
        // If I make 3 requests, it will take 9 seconds to serve all instead of 3
        let val = foo().wait().unwrap();
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
fn foo() -> Result<&'static [u8], i32> {
    let timer = Timer::default();
    // Set a timeout that expires in 500 milliseconds
    let sleep = timer.sleep(Duration::from_millis(3000));

    await!(sleep);
    Ok(PHRASE)
}
