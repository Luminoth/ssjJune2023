use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub display_name: String,
    pub cover_url: String,
    pub url: String,
    pub gamer: bool,
    pub press_user: bool,
    pub developer: bool,
}

#[derive(Debug, Deserialize)]
pub struct MeResponse {
    pub user: User,
}

pub async fn get_user(access_token: impl AsRef<str>) -> anyhow::Result<User> {
    let response = reqwest::get(format!(
        "https://itch.io/api/1/{}/me",
        access_token.as_ref()
    ))
    .await?
    .json::<MeResponse>()
    .await?;

    Ok(response.user)
}
