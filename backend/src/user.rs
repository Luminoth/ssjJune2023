use serde::{Deserialize, Serialize};

use crate::itchio;

#[derive(Debug, Clone, Serialize, Deserialize, dynomite::Item)]
pub struct User {
    #[serde(skip)]
    #[dynomite(partition_key)]
    #[dynomite(rename = "type")]
    r#type: String,

    #[dynomite(sort_key)]
    #[dynomite(rename = "id")]
    user_id: u64,

    username: String,
    display_name: String,
    api_key: String,
}

impl From<itchio::User> for User {
    fn from(user: itchio::User) -> Self {
        Self {
            r#type: "user".to_owned(),
            user_id: user.id,
            username: user.username,
            display_name: user.display_name,
            api_key: String::default(),
        }
    }
}

impl User {
    pub fn get_user_id(&self) -> u64 {
        self.user_id
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

    pub fn set_api_key(&mut self, api_key: impl Into<String>) {
        self.api_key = api_key.into()
    }
}
