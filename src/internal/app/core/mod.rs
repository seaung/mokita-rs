pub mod cfg;
pub mod code;
pub mod db;
pub mod errors;
pub mod logger;

use crate::internal::app::core::errors::{HttpResponseError, IsResponseOK};
use anyhow::{Context, Ok};
use serde::Serialize;

#[derive(Clone, Default)]
pub struct AppState {
    db: Option<sqlx::PgPool>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let default_db = db::init_dbpool("db")
            .await
            .context("default database init failed.")?;
        Ok(Self {
            db: Some(default_db),
        })
    }

    pub fn db_pool(&self) -> &sqlx::PgPool {
        self.db
            .as_ref()
            .expect("default database is None (forgotten initalize?)")
    }
}

#[derive(Serialize)]
pub struct ResponseBody<T>
where
    T: Serialize + Send,
{
    pub code: i32,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

pub type HttpResult<T> = anyhow::Result<IsResponseOK<T>, HttpResponseError>;
