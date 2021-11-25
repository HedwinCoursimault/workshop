use std::convert::Infallible;

use crate::{auth, environment::Environment, model::user::{UserInsertable, UserLogin}};
use uuid::Uuid;
use warp::{Filter, http::HeaderValue, hyper::HeaderMap};

pub fn with_env(env: Environment) -> impl Filter<Extract = (Environment,), Error = Infallible> + Clone {
    warp::any().map(move || env.clone())
}

pub fn user_json_body() -> impl Filter<Extract = (UserInsertable,), Error = warp::Rejection> + Clone {
    warp::body::json()
}

pub fn user_login_json_body() -> impl Filter<Extract = (UserLogin,), Error = warp::Rejection> + Clone {
    warp::body::json()
}

pub fn with_auth(role: auth::Role) -> impl Filter<Extract = (Uuid,), Error = warp::Rejection> + Clone {
    warp::header::headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (role.clone(), headers))
        .and_then(auth::authorize)
}