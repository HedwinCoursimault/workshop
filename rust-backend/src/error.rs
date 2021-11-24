#[derive(thiserror::Error, Debug)]
pub enum EnvironmentError {
    #[error("The required environment variable {0} is not defined")]
    EnvVariableMissing(String),
    #[error("Connection to database failed with :\n{0}")]
    DbConnectionFailed(sqlx::Error),
}

#[derive(serde::Serialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
    pub status: String,
}
