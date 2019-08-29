extern crate futures;
extern crate hyper;
extern crate redis;

use redis::Commands;
use futures::future;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::rt::Future;
use hyper::header::{HeaderValue, LOCATION};
use hyper::service::{make_service_fn, service_fn};
use std::sync::Arc;

fn main() {

    hyper()
}

fn get_url(key: &str) -> redis::RedisResult<String> {
    let client = redis::Client::open("redis://redis-service:6379/")?;
    let mut con = client.get_connection()?;
    //let _ : () = con.set(key, "http://google.com/")?;

    con.get(key)
}

fn hyper() {
    let addr = ([0, 0, 0, 0], 3001).into();

    let redirect_service = || service_fn(move |req| -> BoxFut {
        let mut response = Response::new(Body::empty());

        *response.status_mut() = StatusCode::PERMANENT_REDIRECT;
        
        let key = &req.uri().path()[1..];
        let url = get_url(key);

        let _ = match url {
            Ok(u) => response.headers_mut().insert(LOCATION, HeaderValue::from_str(u.as_ref()).unwrap()),
            Err(e) =>  {
                eprintln!("{}", e);
                //response.headers_mut().insert(LOCATION, HeaderValue::from_str(String::from(e).as_ref()).unwrap())
                response.headers_mut().insert(LOCATION, HeaderValue::from_str(("/app/unknown/".to_string() + key).as_ref()).unwrap())
            },
        };

        Box::new(future::ok(response))
    });

    let server = Server::bind(&addr)
        .serve(redirect_service)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}

type BoxFut = Box<Future<Item = Response<Body>, Error=hyper::Error> + Send>;

// struct Redirect {
//     client: redis::Client
// }

// impl Service for Redirect {
//     type ReqBody = Body;
//     type ResBody = Body;
//     type Error = hyper::Error;
//     type Future = Box<Future<Item = Response<Body>, Error=hyper::Error> + Send>;

//     fn call(&mut self, req: Request<Body>) -> Self::Future {
//         let mut response = Response::new(Body::empty());

//         *response.status_mut() = StatusCode::PERMANENT_REDIRECT;
        
//         let key = &req.uri().path()[1..];
//         let url = get_url(key, self.client);

//         let _ = match url {
//             Ok(u) => response.headers_mut().insert(LOCATION, HeaderValue::from_str(u.as_ref()).unwrap()),
//             Err(_) => response.headers_mut().insert(LOCATION, HeaderValue::from_str(("/app/unknown/".to_string() + key).as_ref()).unwrap()),
//         };

//         Box::new(future::ok(response))
//     }
// }