use std::convert::Infallible;
use warp::{
    http::StatusCode,
    Filter, Rejection, Reply,
};

use crate::environment::Environment;

mod db;
mod environment;
mod error;
mod filters;
mod handlers;
mod model;
mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let env = match Environment::from_env().await {
        Ok(env) => env,
        Err(e) => panic!("{:?}", e),
    };

    let port = env.app_port();

    let router = routes::upload(env.clone())
        .or(routes::download(env.clone()))
        .or(routes::healthz())
        .recover(handle_rejection);
    println!("Server started at localhost:8080");
    warp::serve(router).run(([0, 0, 0, 0], port)).await;
}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}
