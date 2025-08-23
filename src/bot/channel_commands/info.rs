use crate::{
    bot::channel_commands::helpers::one_or_notify,
    db::{
        entities::{channel, channel_comics_provider_subscription, comics_provider},
        get_db,
    },
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use teloxide::{
    payloads::SendMessageSetters, prelude::{Requester, ResponseResult}, sugar::request::RequestReplyExt, types::Message, Bot
};

pub async fn handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let Some(channel) = one_or_notify(
        channel::Entity::find().filter(channel::Column::Tgid.eq(msg.chat.id.0)),
        &bot,
        &msg,
        "Don't forget to register using /register!",
    )
    .await
    else {
        return Ok(());
    };

    let Ok(subscriptions) = channel_comics_provider_subscription::Entity::find()
        .filter(channel_comics_provider_subscription::Column::ChannelId.eq(channel.id))
        .all(get_db())
        .await
    else {
        return Ok(());
    };

    let mut info_msg: String = "Subscribed to:\n".to_owned();
    for subscription in subscriptions {
        let Some(comic_provider) = comics_provider::Entity::find()
            .filter(comics_provider::Column::Id.eq(subscription.provider_id))
            .one(get_db())
            .await
            .unwrap()
        else {
            continue;
        };
        info_msg.push_str(&format!("`{}`\n", comic_provider.name));
    }

    bot.send_message(msg.chat.id, info_msg)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_to(msg.id)
        .await?;
    Ok(())
}
