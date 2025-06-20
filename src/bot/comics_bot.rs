use crate::{
    bot::commands::{self, Command},
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
        let handler = dptree::entry().branch(
            Update::filter_message()
                .branch(filter_command::<Command, _>().endpoint(Self::commands_handler))
                .branch(dptree::endpoint(Self::handle_random_messages)),
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

    async fn handle_random_messages(_: Bot, _: Message) -> ResponseResult<()> {
        Ok(())
    }
}
