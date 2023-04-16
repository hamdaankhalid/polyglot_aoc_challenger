use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::net::SocketAddr;

use rand::Rng;

pub fn pick_lang() -> &'static str {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..LANGUAGES.len());
    LANGUAGES[random_number]
}

pub fn pick_aoc_challenge_url() -> String {
    let mut rng = rand::thread_rng();
    let rand_year = rng.gen_range(2015..2022);
    let rand_day = rng.gen_range(1..25);
    format!("https://adventofcode.com/{}/day/{}", rand_year, rand_day)
}

const LANGUAGES: [&str; 15] = [
    "Golang",
    "Python",
    "Java",
    "Kotlin",
    "C#",
    "Ruby",
    "Scala",
    "C++",
    "C",
    "F#",
    "OCaml",
    "Haskell",
    "Rust",
    "Javascript",
    "Typescript",
];

#[derive(Serialize, Deserialize, Debug)]
struct ChallengeResp {
    language: String,
    aoc_url: String,
}

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let language = pick_lang();
    let aoc = pick_aoc_challenge_url();
    let resp = ChallengeResp {
        language: language.to_string(),
        aoc_url: aoc,
    };
    Ok(Response::new(Body::from(
        serde_json::to_string(&resp).unwrap(),
    )))
}

#[tokio::main]
async fn main() {
    
    const PORT: u16 = 3000;

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));

    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);
    
    println!("Starting server on port: {}", PORT);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
