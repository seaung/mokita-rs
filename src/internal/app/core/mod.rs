pub mod cfg;
pub mod code;
pub mod db;

use anyhow::{Context, Ok};

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
