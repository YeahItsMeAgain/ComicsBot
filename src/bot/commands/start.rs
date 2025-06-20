use teloxide::{
    Bot,
    prelude::{Requester, ResponseResult},
    types::Message,
};

pub async fn handler(bot: Bot, msg: Message) -> ResponseResult<()> {
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
