use crate::db::{
    entities::{channel, channel_comics_provider_subscription, comics_provider},
    get_db,
};
use sea_orm::EntityTrait;
use teloxide::{
    Bot,
    payloads::SendMessageSetters,
    prelude::{Requester, ResponseResult},
    sugar::request::RequestReplyExt,
    types::Message,
};

pub async fn handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let Ok(subscriptions) = channel_comics_provider_subscription::Entity::find()
        .find_also_related(channel::Entity)
        .find_also_related(comics_provider::Entity)
        .all(get_db())
        .await
    else {
        return Ok(());
    };

    let mut subscriptions_msg: String = "Subscriptions:\n".to_owned();
    for (_subscription, channel, provider) in subscriptions {
        let channel = channel.unwrap();
        let provider = provider.unwrap();
        subscriptions_msg.push_str(&format!(
            "\\<`{}`:`{}`\\> \\- `{}`\n",
            channel.tgid, channel.title, provider.name
        ));
    }

    bot.send_message(msg.chat.id, subscriptions_msg)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_to(msg.id)
        .await?;
    Ok(())
}
