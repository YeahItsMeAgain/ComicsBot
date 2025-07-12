use crate::{comics_providers::cyanide_and_happiness, config::CONFIG};
use teloxide::Bot;
use tokio::spawn;
use tokio_schedule::{Job, every};

pub async fn schedule(bot: Bot) {
    let cyanide_and_happiness_bot = bot.clone();
    spawn(
        every(CONFIG.cyanide_and_happiness.update_schedule_hours)
            .hours()
            .perform(move || {
                let bot = cyanide_and_happiness_bot.clone();
                async move {
                    if let Err(err) = cyanide_and_happiness::notify_changes(bot).await {
                        log::error!("Error checking for notifications: {}", err);
                    }
                }
            }),
    );
}
