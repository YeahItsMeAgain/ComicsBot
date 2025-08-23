use once_cell::sync::OnceCell;
use sea_orm::{DatabaseConnection, EntityTrait, sea_query::OnConflict};
use strum::IntoEnumIterator;

use crate::{comics_providers::ComicsProviders, db::entities::comics_provider};
pub mod entities;

pub static DB: OnceCell<DatabaseConnection> = OnceCell::new();

pub fn get_db() -> &'static DatabaseConnection {
    DB.get().unwrap()
}

pub async fn init() -> anyhow::Result<()> {
    create_comics_provider().await?;
    Ok(())
}

pub async fn create_comics_provider() -> anyhow::Result<()> {
    for provider in ComicsProviders::iter() {
        let provider_db = comics_provider::ActiveModel {
            name: sea_orm::ActiveValue::Set(provider.to_string()),
            ..Default::default()
        };
        comics_provider::Entity::insert(provider_db)
            .on_conflict(
                OnConflict::column(comics_provider::Column::Name)
                    .do_nothing()
                    .to_owned(),
            )
            .do_nothing()
            .exec(get_db())
            .await?;
    }
    Ok(())
}
