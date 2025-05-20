pub use sea_orm_migration::prelude::*;

mod m20220120_000001_create_post_table;
mod m20250517_010325_add_kvs;
mod m20250520_103407_add_parent_id;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220120_000001_create_post_table::Migration),
            Box::new(m20250517_010325_add_kvs::Migration),
            Box::new(m20250520_103407_add_parent_id::Migration),
        ]
    }
}
