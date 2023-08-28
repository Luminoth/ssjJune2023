use std::fmt;

use aws_config::SdkConfig;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::auth;
use crate::aws;
use crate::itchio;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
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

    #[allow(dead_code)]
    pub fn get_api_key(&self) -> &String {
        &self.api_key
    }

    pub fn set_api_key(&mut self, api_key: impl Into<String>) {
        self.api_key = api_key.into()
    }
}

pub async fn validate_user(
    aws_config: &SdkConfig,
    bearer_token: impl AsRef<str>,
) -> anyhow::Result<String> {
    let secret = aws::get_jwt_secret(aws_config).await?;
    let user_id = auth::validate_user_access_token(bearer_token, secret)?;

    info!("validating user {} ...", user_id);

    // TODO:

    Ok(user_id)
}

pub async fn get_user(
    aws_config: &SdkConfig,
    bearer_token: impl AsRef<str>,
) -> anyhow::Result<User> {
    let user_id = validate_user(aws_config, bearer_token).await?;

    info!("getting user for {} ...", user_id);

    aws::get_user(aws_config, user_id.parse()?)
        .await?
        .ok_or_else(|| anyhow::anyhow!("no such user"))
}
