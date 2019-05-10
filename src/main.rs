extern crate futures;
extern crate hyper;

use futures::future;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::rt::Future;
use hyper::header::{HeaderValue, LOCATION};
use hyper::service::service_fn;

fn main() {
    hyper()
}

fn hyper() {
    let addr = ([0, 0, 0, 0], 3001).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(redirect))
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}

type BoxFut = Box<Future<Item = Response<Body>, Error=hyper::Error> + Send>;

fn redirect(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());
    
    *response.status_mut() = StatusCode::PERMANENT_REDIRECT;
    response.headers_mut().insert(LOCATION, HeaderValue::from_str(&req.uri().path()[1..]).unwrap());

    Box::new(future::ok(response))
}