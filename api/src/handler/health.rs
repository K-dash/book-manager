use axum::{extract::State, http::StatusCode};
use registry::AppRegistry;

// ヘルスチェック用のハンドラ
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

// Stateに登録されているAppRegistryを取得し、ヘルスチェックを行う
pub async fn health_check_db(State(registry): State<AppRegistry>) -> StatusCode {
    // health_check_repositoryメソッドを呼び出し、HealthCheckRepositoryを取得
    if registry.health_check_repository().check_db().await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let status_code = health_check().await;
        assert_eq!(status_code, StatusCode::OK);
    }
}
