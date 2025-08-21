use super::{model::User, service::UserService};
use crate::base::controller::BaseController;

pub struct UserController;

impl BaseController<User, i32> for UserController {
    type Service = UserService;

    fn path() -> &'static str {
        "/users"
    }
}
