use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::model::group;

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(title = "UserModel", as = user::Model)]
pub struct Model {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WithGroup {
    model: Model,
    group: group::Model,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WithGroups {
    model: Model,
    groups: Vec<group::Model>,
}
