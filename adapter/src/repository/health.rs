use async_trait::async_trait;
use derive_new::new;
use kernel::repository::health::HealthCheckRepository;

use crate::database::ConnectionPool;

// コンストラクタを自動生成
#[derive(new)]
pub struct HealthCheckRepositoryImpl {
    // 構造体にConnectionPoolを持たせる
    db: ConnectionPool,
}

#[async_trait]
// HealthCheckRepositoryを実装
impl HealthCheckRepository for HealthCheckRepositoryImpl {
    async fn check_db(&self) -> bool {
        // データベースへの接続を確認
        sqlx::query("SELECT 1")
            .fetch_one(self.db.inner_ref())
            .await
            .is_ok()
    }
}
