use crate::config::CONFIG;
use log::LevelFilter;
use persistent_kv::{Config, PersistentKeyValueStore};
use sea_orm::{ConnectOptions, Database};

mod bot;
mod comics_providers;
mod config;
mod db;
mod periodic;
mod store;

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();

    log::info!("Connecting to the db");
    let mut db_connection_option = ConnectOptions::new(CONFIG.database_url.clone());
    db_connection_option.sqlx_logging_level(log::LevelFilter::Debug);
    let db: sea_orm::DatabaseConnection = Database::connect(db_connection_option).await.unwrap();
    db::DB.set(db).unwrap();

    log::info!("Setting app persistent store");
    let store: PersistentKeyValueStore<String, String> =
        PersistentKeyValueStore::new(CONFIG.persistent_store_path.clone(), Config::default())
            .unwrap();
    store::STORE.set(store).unwrap();

    log::info!("Initializing the bot");
    let bot = bot::comics_bot::ComicsBot::default();

    log::info!("Starting schedules");
    periodic::schedule(bot.bot.clone()).await;

    log::info!("Starting the bot");
    bot.start().await;
}
