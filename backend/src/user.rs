use std::fmt;

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
    user_id: String,

    display_name: String,
    api_key: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.user_id, self.display_name)
    }
}

impl From<itchio::User> for User {
    fn from(user: itchio::User) -> Self {
        let display_name = user.display_name.unwrap_or_else(|| user.username.clone());

        Self {
            r#type: "user".to_owned(),
            user_id: user.id.to_string(),
            display_name,
            api_key: String::default(),
        }
    }
}

impl User {
    pub fn get_user_id(&self) -> &String {
        &self.user_id
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
