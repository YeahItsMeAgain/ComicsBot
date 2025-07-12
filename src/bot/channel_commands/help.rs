use teloxide::{
    Bot,
    payloads::SendMessageSetters,
    prelude::{Requester, ResponseResult},
    sugar::request::RequestReplyExt,
    types::Message,
};

const AVAILABLE_CMDS_HELP: &str = r#"
The available commands are:

`/help` This help message
`/register` Register your channel to the comics bot system
`/list` List all available providers
`/info` Display info such as your subscriptions
`/subscribe <comics_provider>` Subscribe to a comics provider
`/unsubscribe <comics_provider>` Unsubscribe from a comics provider
"#;

pub async fn handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, AVAILABLE_CMDS_HELP)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_to(msg)
        .await?;
    Ok(())
}
