use async_trait::async_trait;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use deadpool_diesel::mysql::Pool;
use diesel::prelude::*;

use super::{model::User, schema::user};
use crate::base::service::BaseService;
use crate::common::utils::internal_error;

pub struct UserService;

#[async_trait]
impl BaseService<User, i32> for UserService {
    async fn get_many(State(pool): State<Pool>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
        let conn = pool.get().await.map_err(internal_error)?;

        let users = conn
            .interact(|conn| user::table.select(User::as_select()).load(conn))
            .await
            .map_err(internal_error)?
            .map_err(internal_error)?;

        Ok(Json(users))
    }

    async fn delete_one(
        Path(id): Path<i32>,
        State(pool): State<Pool>,
    ) -> Result<StatusCode, (StatusCode, String)> {
        let conn = pool.get().await.map_err(internal_error)?;

        let deleted_rows = conn
            .interact(move |conn| diesel::delete(user::table.filter(user::id.eq(id))).execute(conn))
            .await
            .map_err(internal_error)?
            .map_err(internal_error)?;

        if deleted_rows == 0 {
            return Err((StatusCode::NOT_FOUND, "User not found".to_string()));
        }

        Ok(StatusCode::NO_CONTENT)
    }
}
