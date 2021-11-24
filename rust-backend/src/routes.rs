use warp::{Filter, Rejection, Reply};

use crate::{environment::Environment, filters, handlers};

pub fn healthz() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("healthz")
        .and(warp::get())
        .and_then(handlers::health)
}

pub fn upload(env: Environment) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(5_000_000))
        .and(filters::with_env(env))
        .and_then(handlers::upload)
}

pub fn download(env: Environment) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("files")
        .and(warp::fs::dir("./files/")
        .and(filters::with_env(env))
        .and_then(handlers::download))
}
