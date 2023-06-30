use serde::Serialize;

use crate::itchio;

#[derive(Debug, Serialize, dynomite::Item)]
pub struct User {
    #[dynomite(partition_key)]
    id: u64,
    username: String,
    display_name: String,
    api_key: String,
}

impl From<itchio::User> for User {
    fn from(user: itchio::User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            display_name: user.display_name,
            api_key: String::default(),
        }
    }
}

// TODO: from AWS attribute values

impl User {
    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_display_name(&self) -> &String {
        &self.display_name
    }

    pub fn get_api_key(&self) -> &String {
        &self.api_key
    }
}
