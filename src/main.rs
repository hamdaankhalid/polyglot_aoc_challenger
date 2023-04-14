use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::net::SocketAddr;
use std::convert::Infallible;
use serde::{Serialize, Deserialize};

mod random_picker;

#[derive(Serialize, Deserialize, Debug)]
struct ChallengeResp {
    language: String,
    aoc_url: String
}

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let language = random_picker::pick_lang();
    let aoc = random_picker::pick_aoc_challenge_url();
    let resp = ChallengeResp{language: language.to_string(), aoc_url: aoc};    
    Ok(Response::new(Body::from(serde_json::to_string(&resp).unwrap())))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

}
