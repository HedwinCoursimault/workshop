use warp::{Filter, Rejection, Reply};

use crate::{auth::Role, environment::Environment, filters, handlers};

pub fn healthz() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("healthz")
        .and(warp::get())
        .and_then(handlers::health)
}

pub fn upload(env: Environment) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(5_000_000))
        .and(filters::with_auth(Role::User))
        .and(filters::with_env(env))
        .and_then(handlers::upload)
}

pub fn file_list_by_uploader(
    env: Environment,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("files")
        .and(warp::get())
        .and(filters::with_auth(Role::User))
        .and(filters::with_env(env))
        .and_then(handlers::get_files_by_uploader)
}

pub fn download(env: Environment) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("files").and(
        warp::fs::dir("./files/")
            .and(filters::with_auth(Role::User))
            .and(filters::with_env(env))
            .and_then(handlers::download),
    )
}

pub fn login(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login")
        .and(warp::post())
        .and(filters::with_env(env))
        .and(filters::user_login_json_body())
        .and_then(handlers::login)
}

pub fn create_account(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(warp::post())
        .and(filters::with_env(env))
        .and(filters::user_json_body())
        .and_then(handlers::insert_user)
}
