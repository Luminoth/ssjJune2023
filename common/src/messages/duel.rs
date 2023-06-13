use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Message;

#[derive(Debug, Serialize, Deserialize)]
pub struct Duel {
    user_id: Uuid,
    character_id: Uuid,

    opponent_user_id: Uuid,
    opponent_character_id: Uuid,
}

impl Message for Duel {}

impl From<Duel> for String {
    fn from(message: Duel) -> String {
        serde_json::to_string(&message).unwrap_or(String::new())
    }
}

impl Duel {
    pub fn new(
        user_id: Uuid,
        character_id: Uuid,
        opponent_user_id: Uuid,
        opponent_character_id: Uuid,
    ) -> Self {
        Self {
            user_id,
            character_id,
            opponent_user_id,
            opponent_character_id,
        }
    }
}
