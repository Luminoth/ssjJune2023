use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Duel {
    user_id: Uuid,
    character_id: Uuid,

    opponent_user_id: Uuid,
    opponent_character_id: Uuid,
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
