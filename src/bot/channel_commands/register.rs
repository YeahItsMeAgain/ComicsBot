use crate::db::{entities::channel, get_db};
use sea_orm::{ActiveValue, EntityTrait, sea_query::OnConflict};
use teloxide::{
    prelude::{Requester, ResponseResult}, sugar::request::RequestReplyExt, types::Message, Bot
};

pub async fn handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let channel = channel::ActiveModel {
        tgid: ActiveValue::Set(msg.chat.id.0),
        title: ActiveValue::Set(msg.chat.title().unwrap_or("<Unknown title>").to_owned()),
        ..Default::default()
    };
    if let Err(err) = channel::Entity::insert(channel)
        .on_conflict(
            OnConflict::column(channel::Column::Tgid)
                .update_column(channel::Column::Title)
                .to_owned(),
        )
        .exec(get_db())
        .await
    {
        log::error!("Failed to save channel: {err}");
        bot.send_message(msg.chat.id, "Failed to save the channel, try again later!").reply_to(msg)
            .await?;
        return Ok(());
    }

    bot.send_message(
        msg.chat.id,
        "Done ✅
Please use /help to view all the available options",
    )
    .await?;
    Ok(())
}
