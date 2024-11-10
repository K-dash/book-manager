use shared::config::DatabaseConfig;
use sqlx::{postgres::PgConnectOptions, PgPool};

// DatabaseConfig から PgConnectOptions に変換する
fn make_pg_connect_options(cfg: &DatabaseConfig) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .username(&cfg.username)
        .password(&cfg.password)
        .database(&cfg.database)
}

// sqlx::PgPool をラップする（PgPoolを露出させないため）
// 後にDIコンテナをregistryに作る際、sqlxへの依存を registryワークスペースに追加しなくて済む
#[derive(Clone)]
pub struct ConnectionPool(PgPool);

impl ConnectionPool {
    // sqlx::PgPool への参照を取得する
    pub fn inner_ref(&self) -> &PgPool {
        &self.0
    }
}

// 返り値を ConnectionPool に変更する
pub fn connect_database_with(cfg: &DatabaseConfig) -> ConnectionPool {
    ConnectionPool(PgPool::connect_lazy_with(make_pg_connect_options(cfg)))
}
