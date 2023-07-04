use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Duel {
    user_id: String,
    character_id: Uuid,

    opponent_user_id: String,
    opponent_character_id: Uuid,
}

impl Duel {
    pub fn new(
        user_id: impl Into<String>,
        character_id: Uuid,
        opponent_user_id: impl Into<String>,
        opponent_character_id: Uuid,
    ) -> Self {
        Self {
            user_id: user_id.into(),
            character_id,
            opponent_user_id: opponent_user_id.into(),
            opponent_character_id,
        }
    }
}
