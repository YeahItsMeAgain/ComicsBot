use crate::bot::admin::{LIST_SUBSCRIPTIONS_COMMAND, helpers::is_from_admin};
use teloxide::{
    Bot,
    payloads::SendMessageSetters,
    prelude::{Requester, ResponseResult},
    types::{KeyboardButton, KeyboardMarkup, Message},
};

pub async fn handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    if is_from_admin(msg.clone()) {
        bot.send_message(msg.chat.id, "👋 Welcome Oh Great Admin")
            .reply_markup(
                KeyboardMarkup::new([[KeyboardButton::new(LIST_SUBSCRIPTIONS_COMMAND)]])
                    .resize_keyboard()
                    .persistent(),
            )
            .await?;
        return Ok(());
    }

    bot.send_message(
        msg.chat.id,
        format!(
            "👋 Welcome {}!
Add me to a channel with the following permissions:
- Post messages.

And send /register in that channel.
            ",
            msg.chat.first_name().unwrap_or("Mystery man")
        ),
    )
    .await?;
    Ok(())
}
