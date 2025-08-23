use teloxide::types::Message;

use crate::config::CONFIG;

pub fn is_from_admin(msg: Message) -> bool {
    let user_id = msg.from.clone();
    if user_id.is_none() {
        return false;
    }

    let user_id = user_id.unwrap().id.0;
    CONFIG.admins.contains(&user_id)
}
