use crate::{
    bot::{
        channel_commands::{self, ChannelCommand},
        commands::{self, Command},
    },
    config::CONFIG,
};
use teloxide::{filter_command, prelude::*};

pub struct ComicsBot {
    pub bot: Bot,
}

impl Default for ComicsBot {
    fn default() -> Self {
        Self::new()
    }
}

impl ComicsBot {
    fn new() -> Self {
        ComicsBot {
            bot: Bot::new(CONFIG.bot_token.clone()),
        }
    }

    pub async fn start(self) {
        let handler = dptree::entry()
            .branch(
                Update::filter_message()
                    .branch(filter_command::<Command, _>().endpoint(Self::commands_handler))
                    .branch(dptree::endpoint(Self::handle_random_messages)),
            )
            .branch(
                Update::filter_channel_post()
                    .branch(dptree::endpoint(Self::channel_command_handler)),
            );

        Dispatcher::builder(self.bot, handler)
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }

    async fn commands_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
        match cmd {
            Command::Start => commands::start::handler(bot, msg).await?,
        }
        Ok(())
    }

    async fn channel_command_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
        match msg.text() {
            Some(cmd) if cmd == ChannelCommand::Register.as_cmd_str() => {
                channel_commands::register::handler(bot, msg).await?
            }
            Some(_) => {
                bot.send_message(
                    msg.chat.id,
                    "Sorry, I couldn't understand that. Try using /help.",
                )
                .await?;
            }
            None => {}
        }
        Ok(())
    }

    async fn handle_random_messages(_: Bot, msg: Message) -> ResponseResult<()> {
        Ok(())
    }
}
