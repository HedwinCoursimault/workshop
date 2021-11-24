use std::convert::Infallible;

use crate::environment::Environment;
use warp::Filter;

pub fn with_env(env: Environment) -> impl Filter<Extract = (Environment,), Error = Infallible> + Clone {
    warp::any().map(move || env.clone())
}
