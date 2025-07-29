use axum::{Json, extract::State, http::StatusCode};
use deadpool_diesel::mysql::Pool;
use diesel::prelude::*;

use super::{model::User, schema::user};
use crate::common::utils::internal_error;

pub async fn get_users(State(pool): State<Pool>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let users = conn
        .interact(|conn| user::table.select(User::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    return Ok(Json(users));
}
