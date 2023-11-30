use std::io::{empty, Empty, Read};
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

// curl -XGET localhost:3000/delay/30
async fn delay(req: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    // get delay numeric from path
    let path: Vec<&str> = req.uri().path().split("/").collect();
    let uri_n1 = path.get(1).expect("unable to get path");
    let delay_time = path.get(2).unwrap().parse::<u64>().expect("unable to parse int from delay_time");

    // apply sleep delay and make request
    println!("{}", uri_n1);
    let _ = tokio::time::sleep(Duration::new(delay_time, 0)).await;
    let success = format!("successfully delayed request by {} seconds", delay_time);


    match (req.method(), uri_n1) {
        (&Method::GET, &"delay") => Ok(Response::new(full(success))),
        _ => Ok(Response::new(full("404")))
    }
    // Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(delay))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
