use async_trait::async_trait;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use deadpool_diesel::mysql::Pool;

#[async_trait]
pub trait BaseService<T, Id>: Send + Sync + 'static {
    async fn get_many(pool: State<Pool>) -> Result<Json<Vec<T>>, (StatusCode, String)>;

    async fn delete_one(
        id: Path<Id>,
        pool: State<Pool>,
    ) -> Result<StatusCode, (StatusCode, String)>;
}
