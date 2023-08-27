use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::config::ClientConfig;
use crate::user::User;

pub async fn get_jwt_secret(aws_config: &SdkConfig) -> anyhow::Result<String> {
    let client = aws_sdk_secretsmanager::Client::new(aws_config);
    Ok(client
        .get_secret_value()
        .secret_id("ssj2023-jwt")
        .send()
        .await?
        .secret_string()
        .ok_or_else(|| anyhow::anyhow!("missing secret"))?
        .to_owned())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DbUser {
    #[serde(rename = "type")]
    r#type: String,

    #[serde(flatten)]
    user: User,
}

impl From<User> for DbUser {
    fn from(user: User) -> Self {
        Self {
            r#type: "user".to_owned(),
            user,
        }
    }
}

pub async fn get_user(aws_config: &SdkConfig, user_id: u64) -> anyhow::Result<Option<User>> {
    let client = aws_sdk_dynamodb::Client::new(aws_config);
    let output = client
        .get_item()
        .table_name("ssj2023")
        .key("type".to_owned(), AttributeValue::S("user".to_owned()))
        .key("id".to_owned(), AttributeValue::S(user_id.to_string()))
        .send()
        .await?;

    Ok(match output.item {
        Some(item) => {
            let user: DbUser = serde_dynamo::from_item(item)?;
            Some(user.user)
        }
        None => None,
    })
}

pub async fn save_user(aws_config: &SdkConfig, user: User) -> anyhow::Result<()> {
    let item = serde_dynamo::to_item(DbUser::from(user))?;

    let client = aws_sdk_dynamodb::Client::new(aws_config);
    client
        .put_item()
        .table_name("ssj2023")
        .set_item(Some(item))
        .send()
        .await?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DbClientConfig {
    #[serde(rename = "type")]
    r#type: String,

    #[serde(flatten)]
    client_config: ClientConfig,
}

impl From<ClientConfig> for DbClientConfig {
    fn from(client_config: ClientConfig) -> Self {
        Self {
            r#type: "config".to_owned(),
            client_config,
        }
    }
}

pub async fn get_client_config(aws_config: &SdkConfig) -> anyhow::Result<Option<ClientConfig>> {
    let client = aws_sdk_dynamodb::Client::new(aws_config);
    let output = client
        .get_item()
        .table_name("ssj2023")
        .key("type".to_owned(), AttributeValue::S("config".to_owned()))
        .key("id".to_owned(), AttributeValue::S("client".to_owned()))
        .send()
        .await?;

    Ok(match output.item {
        Some(item) => {
            let client_config: DbClientConfig = serde_dynamo::from_item(item)?;
            Some(client_config.client_config)
        }
        None => None,
    })
}

pub async fn get_queue_url(aws_config: &SdkConfig) -> anyhow::Result<String> {
    info!("getting queue URL ...");

    let client = aws_sdk_sqs::Client::new(aws_config);
    let result = client.get_queue_url().queue_name("ssj2023").send().await?;
    Ok(result
        .queue_url()
        .ok_or(anyhow::anyhow!("missing queue URL"))?
        .to_owned())
}

pub async fn post_message(
    aws_config: &SdkConfig,
    queue_url: impl Into<String>,
    message: impl Into<String>,
) -> anyhow::Result<()> {
    let client = aws_sdk_sqs::Client::new(aws_config);
    client
        .send_message()
        .queue_url(queue_url)
        .message_body(message)
        .send()
        .await?;

    Ok(())
}
