extern crate fastly;
mod game;
mod moves;

use std::convert::TryInto;
use fastly::http::{header, Method, StatusCode};
use fastly::{Error, Request, Response};
use crate::game::{Game, Player};

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Log service version
    println!(
        "FASTLY_SERVICE_VERSION: {}",
        std::env::var("FASTLY_SERVICE_VERSION").unwrap_or_else(|_| String::new())
    );

    // Filter request methods...
    match req.get_method() {
        // Block requests with unexpected methods
        &Method::POST | &Method::PUT | &Method::PATCH | &Method::DELETE => {
            let x: Game = req.into_body_str().try_into().unwrap();
            println!("{:?}", x.longest_road(Player::White));
            Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD, PURGE")
                .with_body_text_plain("This method is not allowed\n"))
        }

        // Let any other requests through
        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested not be found\n")),
    }
}
