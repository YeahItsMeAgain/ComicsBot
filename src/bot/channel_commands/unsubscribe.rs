use crate::{
    bot::channel_commands::helpers::one_or_notify,
    db::{
        entities::{channel, channel_comics_provider_subscription, comics_provider},
        get_db,
    },
};
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use teloxide::{
    Bot,
    prelude::{Requester, ResponseResult},
    types::Message,
};

pub async fn handler(bot: Bot, msg: Message, comics_provider_name: &str) -> ResponseResult<()> {
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

    let Some(comics_provider) = one_or_notify(
        comics_provider::Entity::find()
            .filter(comics_provider::Column::Name.eq(comics_provider_name)),
        &bot,
        &msg,
        "Invalid comics provider, use /list to choose another one",
    )
    .await
    else {
        return Ok(());
    };

    let _ = channel_comics_provider_subscription::Entity::delete(
        channel_comics_provider_subscription::ActiveModel {
            channel_id: ActiveValue::Set(channel.id),
            provider_id: ActiveValue::Set(comics_provider.id),
        },
    )
    .exec(get_db())
    .await;
    bot.send_message(
        msg.chat.id,
        format!("Successfully unsubscribed from {}!", comics_provider.name),
    )
    .await?;
    Ok(())
}
