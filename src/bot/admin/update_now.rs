use crate::comics_providers::cyanide_and_happiness;
use teloxide::{
    Bot,
    prelude::{Requester, ResponseResult},
    sugar::request::RequestReplyExt,
    types::Message,
};

pub async fn handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Starting update")
        .reply_to(msg.id)
        .await?;

    if let Err(err) = cyanide_and_happiness::notify_changes(bot.clone()).await {
        bot.send_message(msg.chat.id, format!("Update failed!: {err}"))
            .reply_to(msg.id)
            .await?;
    }

    bot.send_message(msg.chat.id, "Finished update")
        .reply_to(msg.id)
        .await?;
    Ok(())
}
