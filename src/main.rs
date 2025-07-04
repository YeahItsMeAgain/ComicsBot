use crate::config::CONFIG;
use log::LevelFilter;
use sea_orm::{ConnectOptions, Database};

mod bot;
mod config;
mod db;

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

    log::info!("Initializing the bot");
    let bot = bot::comics_bot::ComicsBot::default();

    log::info!("Starting the bot");
    bot.start().await;
}
