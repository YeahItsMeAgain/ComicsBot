use log::LevelFilter;

mod bot;
mod config;

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();

    log::info!("Initializing the bot");
    let bot = bot::comics_bot::ComicsBot::default();

    log::info!("Starting the bot");
    bot.start().await;
}
