use crate::bot::channel_commands::ChannelCommand;
use sea_orm::Iterable;
use teloxide::{
    Bot,
    payloads::SendMessageSetters,
    prelude::{Requester, ResponseResult},
    sugar::request::RequestReplyExt,
    types::Message,
};

pub async fn handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let available_cmds = ChannelCommand::iter()
        .map(|cmd| format!("`{}`", cmd.as_cmd_str()))
        .collect::<Vec<String>>()
        .join("\n");

    bot.send_message(
        msg.chat.id,
        format!("The available commands are: \n\n {available_cmds}"),
    )
    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
    .reply_to(msg)
    .await?;
    Ok(())
}
