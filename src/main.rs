extern crate futures;
extern crate hyper;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader, BufRead};
use futures::future;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::rt::Future;
use hyper::header::{HeaderValue, LOCATION};
use hyper::service::service_fn;

fn main() {
    tcpstreams()
}

fn tcpstreams() {
    let listener = TcpListener::bind("0.0.0.0:3001").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("handling connection");
    
    let mut buffer = [0; 512];

    let mut reader = BufReader::new(stream);
    
    let mut line = String::new();
    for rline in reader.by_ref().lines() {
        line = rline.unwrap();
        break;
    }

    let len = line.len();
    let path: &str = if len > 14 {
        line[5..len-9].as_ref()
    } else {
        "http://www.google.com/"
    };

    //let response = format!("HTTP/1.1 308 Permanent Redirect\r\nLocation: {}", path);
    let response = "HTTP/1.1 308 Permanent Redirect\r\nLocation: http://www.google.com/";


    stream = reader.into_inner();

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

pub fn hyper() {
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