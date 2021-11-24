use crate::db;
use crate::error::EnvironmentError;

use std::env;

use sqlx::postgres::PgPool;

#[derive(Debug, Clone)]
pub struct Environment {
    db_pool: PgPool,
    app_port: u16
}

impl Environment {
    pub async fn from_env() -> Result<Self, EnvironmentError> {
        let db_url = env::var("DATABASE_URL")
            .map_err(|_| EnvironmentError::EnvVariableMissing("DATABASE_URL".to_owned()))?;

        let app_port = env::var("APP_PORT")
            .map_err(|_| EnvironmentError::EnvVariableMissing("APP_PORT".to_owned()))?;
        
        let app_port: u16 = app_port.parse().map_err(|_| EnvironmentError::EnvVariableMissing("KEBAB_PORT is malformed".to_owned()))?;

        let db_pool = db::init_pool(&db_url)
            .await
            .map_err(EnvironmentError::DbConnectionFailed)?;

        Ok(Self {
            db_pool,
            app_port
        })
    }

    pub fn db_pool(&self) -> &PgPool {
        &self.db_pool
    }

    pub fn app_port(&self) -> u16 {
        self.app_port
    }
}
