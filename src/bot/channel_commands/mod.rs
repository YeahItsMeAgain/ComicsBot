use sea_orm::EnumIter;
pub mod help;
pub mod register;

#[derive(Debug, Clone, EnumIter)]
pub enum ChannelCommand {
    Register,
    Help,
}

impl ChannelCommand {
    pub fn as_cmd_str(&self) -> &'static str {
        match self {
            ChannelCommand::Register => "/register",
            ChannelCommand::Help => "/help",
        }
    }
}
