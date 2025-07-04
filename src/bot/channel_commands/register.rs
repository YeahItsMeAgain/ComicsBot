use teloxide::{
    Bot,
    prelude::{Requester, ResponseResult},
    types::Message,
};

pub async fn handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    // TODO: save in the db or something idk.
    bot.send_message(
        msg.chat.id,
        "Done ✅
Please use /help to view all the available options",
    )
    .await?;
    Ok(())
}
