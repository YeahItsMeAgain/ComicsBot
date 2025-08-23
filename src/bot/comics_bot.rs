use crate::{
    bot::{
        admin::{self, helpers::is_from_admin, LIST_SUBSCRIPTIONS_COMMAND, UPDATE_NOW_COMMAND},
        channel_commands::{self},
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
                    .branch(
                        dptree::filter(Self::filter_allowed_requests)
                            .branch(filter_command::<Command, _>().endpoint(Self::commands_handler))
                            .branch(
                                dptree::filter(is_from_admin)
                                    .endpoint(Self::admin_msg_commands_handler),
                            ),
                    )
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
        let text = msg.text().unwrap_or("");
        match text.split_once(' ').unwrap_or((text, "")) {
            ("/help", _) => channel_commands::help::handler(bot, msg.clone()).await?,
            ("/register", _) => channel_commands::register::handler(bot, msg.clone()).await?,
            ("/list", _) => channel_commands::list::handler(bot, msg.clone()).await?,
            ("/info", _) => channel_commands::info::handler(bot, msg.clone()).await?,
            ("/subscribe", comics_provider) => {
                channel_commands::subscribe::handler(bot, msg.clone(), comics_provider).await?
            }
            ("/unsubscribe", comics_provider) => {
                channel_commands::unsubscribe::handler(bot, msg.clone(), comics_provider).await?
            }
            _ => {
                bot.send_message(
                    msg.chat.id,
                    "Sorry, I couldn't understand that. Try using /help.",
                )
                .await?;
            }
        }
        Ok(())
    }

    fn filter_allowed_requests(msg: Message) -> bool {
        let user_id = msg.clone().from;
        if user_id.is_none() {
            return false;
        }

        let user_id = user_id.unwrap().id.0;
        if !CONFIG.users.contains(&user_id) {
            return false;
        }

        let chat_id = msg.chat.id.0;
        if let Some(text) = msg.text() {
            log::info!(
                "<{}:{}:{}> used: {}",
                user_id,
                msg.chat.username().unwrap_or(""),
                chat_id,
                text
            );
        }
        true
    }

    async fn admin_msg_commands_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
        if msg.text().is_none() {
            return Ok(());
        }

        match msg.text().unwrap() {
            LIST_SUBSCRIPTIONS_COMMAND => admin::list_subscriptions::handler(bot, msg).await?,
            UPDATE_NOW_COMMAND => admin::update_now::handler(bot, msg).await?,
            _ => {}
        }
        Ok(())
    }

    async fn handle_random_messages(_: Bot, _: Message) -> ResponseResult<()> {
        Ok(())
    }
}
