#[derive(thiserror::Error, Debug)]
pub enum EnvironmentError {
    #[error("The required environment variable {0} is not defined")]
    EnvVariableMissing(String),
    #[error("Connection to database failed with :\n{0}")]
    DbConnectionFailed(sqlx::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("no permission")]
    NoPermissionError,
}

impl warp::reject::Reject for AuthError{}

#[derive(serde::Serialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
    pub status: String,
}
