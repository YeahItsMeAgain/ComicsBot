use crate::{
    bot::channel_commands::helpers::one_or_notify,
    db::{entities::channel, get_db},
};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use teloxide::{
    Bot,
    prelude::{Requester, ResponseResult},
    sugar::request::RequestReplyExt,
    types::Message,
};

pub async fn handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let Some(channel) = one_or_notify(
        channel::Entity::find().filter(channel::Column::Tgid.eq(msg.chat.id.0)),
        &bot,
        &msg,
        "You must /register before unregistering :Z",
    )
    .await
    else {
        return Ok(());
    };

    if let Err(err) = channel.delete(get_db()).await {
        log::error!("Failed to delete channel: {err}");
        bot.send_message(
            msg.chat.id,
            "Failed to unregister the channel, try again later!",
        )
        .reply_to(msg)
        .await?;
        return Ok(());
    }

    bot.send_message(msg.chat.id, "Done ✅").await?;
    Ok(())
}
