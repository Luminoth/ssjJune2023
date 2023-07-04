pub mod duel;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use duel::Duel;

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Invalid,
    Duel(Duel),
}

impl From<Message> for String {
    fn from(message: Message) -> String {
        serde_json::to_string(&message).unwrap_or(String::new())
    }
}

impl From<String> for Message {
    fn from(message: String) -> Self {
        serde_json::from_str(&message).unwrap_or(Message::invalid())
    }
}

impl From<&String> for Message {
    fn from(message: &String) -> Self {
        serde_json::from_str(message).unwrap_or(Message::invalid())
    }
}

impl Message {
    pub fn invalid() -> Self {
        Self::Invalid
    }

    pub fn new_duel(
        user_id: impl Into<String>,
        character_id: Uuid,
        opponent_user_id: impl Into<String>,
        opponent_character_id: Uuid,
    ) -> Self {
        Self::Duel(Duel::new(
            user_id,
            character_id,
            opponent_user_id,
            opponent_character_id,
        ))
    }
}
