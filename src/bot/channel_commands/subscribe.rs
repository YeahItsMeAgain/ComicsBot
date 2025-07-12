use crate::comics_providers::ComicsProviders;
use strum::IntoEnumIterator;
use teloxide::{
    Bot,
    prelude::{Requester, ResponseResult},
    sugar::request::RequestReplyExt,
    types::Message,
};

pub async fn handler(bot: Bot, msg: Message, comics_provider: &str) -> ResponseResult<()> {
    if !ComicsProviders::iter().any(|provider| provider.to_string() == comics_provider) {
        bot.send_message(
            msg.chat.id,
            "Invalid comics provider, use /list to choose another one",
        )
        .reply_to(msg)
        .await?;
        return Ok(());
    }

    // TODO: save in db.
    bot.send_message(
        msg.chat.id,
        format!("Successfully subscribed to {}!", comics_provider),
    )
    .await?;
    Ok(())
}
