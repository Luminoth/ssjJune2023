use std::fmt;

use serde::Deserialize;
use tracing::debug;

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,

    pub display_name: Option<String>,
    pub cover_url: Option<String>,
    pub url: Option<String>,

    pub gamer: bool,
    pub press_user: bool,
    pub developer: bool,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} ({:?}))",
            self.id, self.username, self.display_name
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct MeResponse {
    pub user: User,
}

pub async fn get_user(access_token: impl AsRef<str>) -> anyhow::Result<User> {
    debug!(
        "requesting itchio user with token '{}'",
        access_token.as_ref()
    );

    let response = reqwest::get(format!(
        "https://itch.io/api/1/{}/me",
        access_token.as_ref()
    ))
    .await?
    .json::<MeResponse>()
    .await?;

    Ok(response.user)
}
