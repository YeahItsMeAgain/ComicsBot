use crate::comics_providers::ComicsProviders;
use strum::IntoEnumIterator;
use teloxide::{
    Bot,
    payloads::SendMessageSetters,
    prelude::{Requester, ResponseResult},
    types::Message,
};

pub async fn handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let providers = ComicsProviders::iter()
        .map(|provider| format!("`{}`", provider))
        .collect::<Vec<String>>()
        .join("\n");

    bot.send_message(
        msg.chat.id,
        format!("The available providers:\n{}", providers),
    )
    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
    .await?;
    Ok(())
}
