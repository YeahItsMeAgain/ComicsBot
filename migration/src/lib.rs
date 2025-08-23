pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_channel_table;
mod m20250823_124126_create_channel_provider_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_channel_table::Migration),
            Box::new(m20250823_124126_create_channel_provider_tables::Migration),
        ]
    }
}
