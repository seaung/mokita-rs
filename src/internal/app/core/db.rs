use crate::internal::app::core::cfg::config;
use anyhow::Ok;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub async fn init_dbpool(cfg_key: &str) -> anyhow::Result<PgPool> {
    let cfg = config();

    let dsn = cfg.get_string(&format!("{}.dsn", cfg_key))?;
    let min_conns = cfg.get_int(&format!("{}.min_conns", cfg_key)).unwrap_or(10) as u32;
    let max_conns = cfg.get_int(&format!("{}.max_conns", cfg_key)).unwrap_or(20) as u32;
    let conn_timeout = cfg
        .get_int(&format!("{}.conn_timeout", cfg_key))
        .unwrap_or(10) as u64;
    let idel_timeout = cfg
        .get_int(&format!("{}.idel_timeout", cfg_key))
        .unwrap_or(20) as u64;
    let max_lifetime = cfg
        .get_int(&format!("{}.max_lifetime", cfg_key))
        .unwrap_or(10) as u64;

    let pool = PgPoolOptions::new()
        .min_connections(min_conns)
        .max_connections(max_conns)
        .idle_timeout(Duration::from_secs(idel_timeout))
        .max_lifetime(Duration::from_secs(max_lifetime))
        .acquire_timeout(Duration::from_secs(conn_timeout))
        .connect(&dsn)
        .await?;

    Ok(pool)
}
