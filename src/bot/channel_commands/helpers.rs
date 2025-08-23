use sea_orm::EntityTrait;
use teloxide::{prelude::*, sugar::request::RequestReplyExt, types::Message};

use crate::db::get_db;

pub async fn one_or_notify<E, M>(
    query: sea_orm::Select<E>,
    bot: &Bot,
    msg: &Message,
    not_found_msg: &str,
) -> Option<M>
where
    E: EntityTrait<Model = M>,
    M: Send + Sync + 'static,
{
    match query.one(get_db()).await {
        Ok(Some(model)) => Some(model),
        Ok(None) => {
            let _ = bot
                .send_message(msg.chat.id, not_found_msg)
                .reply_to(msg.id)
                .await;
            None
        }
        Err(err) => {
            log::error!("{err}");
            None
        }
    }
}
