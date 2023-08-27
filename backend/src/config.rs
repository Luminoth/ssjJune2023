use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub max_characters: i32,
    pub max_character_name_len: i32,
}
