#![allow(unused)]

pub mod group;
pub mod user;

macro_rules! impl_model_alias {
    ($($mod:ident),*) => {
        paste::paste! {
            $(
                // pub type [<$mod:camel Model>] = $mod::Model;

                #[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
                pub struct [<$mod:camel Model>](pub $mod::Model);

                impl std::ops::Deref for [<$mod:camel Model>] {
                    type Target = $mod::Model;
                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }

                impl std::ops::DerefMut for [<$mod:camel Model>] {
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        &mut self.0
                    }
                }
            )*
        }
    };
}

// impl_model_alias!(group, user);

// #[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
// pub struct UserModel(pub user::Model);

// #[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
// pub struct GroupModel(pub group::Model);
