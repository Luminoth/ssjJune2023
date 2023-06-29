#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ItchIoUser {
    pub username: String,
    pub display_name: String,
    pub cover_url: String,
    pub url: String,
    pub gamer: bool,
    pub press_user: bool,
    pub developer: bool,
    pub id: u64,
}

#[derive(Debug, Deserialize)]
pub struct ItchIoMeResponse {
    pub user: ItchIoUser,
}

pub async fn get_user(access_token: impl AsRef<str>) -> anyhow::Result<ItchIoUser> {
    let response = reqwest::get(format!(
        "https://itch.io/api/1/{}/me",
        access_token.as_ref()
    ))
    .await?
    .json::<ItchIoMeResponse>()
    .await?;

    Ok(response.user)
}
