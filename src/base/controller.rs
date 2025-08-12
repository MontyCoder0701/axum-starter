use axum::{
    Router,
    routing::{MethodRouter, delete, get},
};
use deadpool_diesel::mysql::Pool;
use serde::de::DeserializeOwned;

use super::service::BaseService;

#[allow(dead_code)]
pub trait BaseController<T, Id>: Sized
where
    T: serde::Serialize + Send + Sync + 'static,
    Id: DeserializeOwned + Send + Sync + 'static,
{
    type Service: BaseService<T, Id>;

    fn path() -> &'static str;

    fn routes() -> Router<Pool> {
        Router::new().nest(
            Self::path(),
            Router::new()
                .route("/", Self::get_many())
                .route("/:id", Self::delete_one()),
        )
    }

    fn get_many() -> MethodRouter<Pool> {
        get(Self::Service::get_many)
    }

    fn delete_one() -> MethodRouter<Pool> {
        delete(Self::Service::delete_one)
    }
}
