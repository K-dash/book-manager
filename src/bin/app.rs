use std::net::{Ipv4Addr, SocketAddr};

use adapter::database::connect_database_with;
use anyhow::{Error, Result};
use api::route::health::build_health_check_routers;
use axum::Router;
use registry::AppRegistry;
use shared::config::AppConfig;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    bootstrap().await
}

async fn bootstrap() -> Result<()> {
    // アプリケーションの設定を読み込む
    let app_confiig = AppConfig::new()?;
    // データベースへの接続を確立する
    let pool = connect_database_with(&app_confiig.database);
    // アプリケーションの状態を保持するレジストリを作成する
    let registry = AppRegistry::new(pool);
    // ルーターを構築する
    let app = Router::new()
        .merge(build_health_check_routers())
        .with_state(registry);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.map_err(Error::from)
}

// // test
// #[tokio::test]
// async fn test_health_check() {
//     let status_code = health_check().await;
//     assert_eq!(status_code, StatusCode::OK);
// }
