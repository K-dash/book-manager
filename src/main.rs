use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{extract::State, http::StatusCode, routing::get, Router};
use sqlx::{postgres::PgConnectOptions, PgPool};
use tokio::net::TcpListener;

// データベース接続情報
struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

// アプリケーション用のデータベース設定構造体から、Postgres接続用の構造体へ変換する
impl From<DatabaseConfig> for PgConnectOptions {
    fn from(cfg: DatabaseConfig) -> Self {
        Self::new()
            .host(&cfg.host)
            .port(cfg.port)
            .username(&cfg.username)
            .password(&cfg.password)
            .database(&cfg.database)
    }
}

// Postgres 専用のコネクションプールを作成する
fn connect_database_with(cfg: DatabaseConfig) -> PgPool {
    PgPool::connect_lazy_with(cfg.into())
}

// データベースのヘルスチェック用のハンドラ
async fn health_check_db(State(db): State<PgPool>) -> StatusCode {
    let connection_result = sqlx::query("SELECT 1").fetch_one(&db).await;
    match connection_result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// ヘルスチェック用のハンドラ
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() -> Result<()> {
    // データベース接続情報
    let database_cfg = DatabaseConfig {
        host: "localhost".to_string(),
        port: 5432,
        username: "app".to_string(),
        password: "passwd".to_string(),
        database: "app".to_string(),
    };
    // コネクションプールを作成
    let conn_pool = connect_database_with(database_cfg);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/health/db", get(health_check_db))
        // データベースのコネクションプールをステートに登録しておき、リクエストハンドラで利用できるようにする
        .with_state(conn_pool);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;

    println!("Server running on: {}", addr);

    Ok(axum::serve(listener, app).await?)
}

// test
#[tokio::test]
async fn test_health_check() {
    let status_code = health_check().await;
    assert_eq!(status_code, StatusCode::OK);
}

#[sqlx::test]
async fn health_check_db_works(pool: sqlx::PgPool) {
    let status_code = health_check_db(State(pool)).await;
    assert_eq!(status_code, StatusCode::OK);
}
