pub mod start;

use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Debug, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Start,
}
