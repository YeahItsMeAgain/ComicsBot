pub mod register;

#[derive(Debug, Clone)]
pub enum ChannelCommand {
    Register,
}

impl ChannelCommand {
    pub fn as_cmd_str(&self) -> &'static str {
        match self {
            ChannelCommand::Register => "/register",
        }
    }
}
