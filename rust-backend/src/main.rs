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
mod auth;

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
        .or(routes::file_list_by_uploader(env.clone()))
        .or(routes::healthz())
        .or(routes::login(env.clone()))
        .or(routes::create_account(env))
        .recover(handle_rejection)
        .with(get_cors());
    println!("Server started at localhost:{}", &port);
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

pub fn get_cors() -> warp::cors::Builder {
    warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "Accept",
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "access-control-allow-origin",
            "Content-Type",
            "Authorization",
        ])
        .allow_methods(vec!["POST", "GET"])
}
