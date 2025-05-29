use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::user;

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(title = "GroupModel", as = group::Model)]
pub struct Model {
    id: u64,
    name: String,
    parent_id: u64,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WithUser {
    model: Model,
    user: user::Model,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WithUsers {
    model: Model,
    users: Vec<user::Model>,
}
