use std::collections::HashMap;

use aws_config::SdkConfig;
use aws_sdk_dynamodb::{primitives::Blob, types::AttributeValue};
use tracing::info;

use crate::user::User;

/// Helper trait for converting dynomite types to AWS SDK types
pub trait ToSdk<T> {
    fn to_sdk(self) -> T;
}

impl ToSdk<AttributeValue> for dynomite::AttributeValue {
    fn to_sdk(self) -> AttributeValue {
        if let Some(v) = self.b {
            AttributeValue::B(Blob::new(v.as_ref()))
        } else if let Some(v) = self.bool {
            AttributeValue::Bool(v)
        } else if let Some(mut v) = self.bs {
            AttributeValue::Bs(v.drain(..).map(|v| Blob::new(v.as_ref())).collect())
        } else if let Some(mut v) = self.l {
            AttributeValue::L(v.drain(..).map(|v| v.to_sdk()).collect())
        } else if let Some(mut v) = self.m {
            AttributeValue::M(v.drain().map(|(k, v)| (k, v.to_sdk())).collect())
        } else if let Some(v) = self.n {
            AttributeValue::N(v)
        } else if let Some(v) = self.ns {
            AttributeValue::Ns(v)
        } else if let Some(v) = self.null {
            AttributeValue::Null(v)
        } else if let Some(v) = self.s {
            AttributeValue::S(v)
        } else if let Some(v) = self.ss {
            AttributeValue::Ss(v)
        } else {
            unreachable!();
        }
    }
}

impl ToSdk<HashMap<String, AttributeValue>> for dynomite::Attributes {
    fn to_sdk(mut self) -> HashMap<String, AttributeValue> {
        self.drain().map(|(k, v)| (k, v.to_sdk())).collect()
    }
}

/// Helper trait for converting from AWS SDK types to dynomite types
pub trait ToRusoto<T> {
    fn to_rusoto(self) -> T;
}

impl ToRusoto<dynomite::AttributeValue> for AttributeValue {
    fn to_rusoto(self) -> dynomite::AttributeValue {
        match self {
            AttributeValue::B(v) => dynomite::dynamodb::AttributeValue {
                b: Some(v.into_inner().into()),
                ..Default::default()
            },
            AttributeValue::Bool(v) => dynomite::dynamodb::AttributeValue {
                bool: Some(v),
                ..Default::default()
            },
            AttributeValue::Bs(mut v) => dynomite::dynamodb::AttributeValue {
                bs: Some(v.drain(..).map(|v| v.into_inner().into()).collect()),
                ..Default::default()
            },
            AttributeValue::L(mut v) => dynomite::dynamodb::AttributeValue {
                l: Some(v.drain(..).map(|v| v.to_rusoto()).collect()),
                ..Default::default()
            },
            AttributeValue::M(mut v) => dynomite::dynamodb::AttributeValue {
                m: Some(v.drain().map(|(k, v)| (k, v.to_rusoto())).collect()),
                ..Default::default()
            },
            AttributeValue::N(v) => dynomite::dynamodb::AttributeValue {
                n: Some(v),
                ..Default::default()
            },
            AttributeValue::Ns(v) => dynomite::dynamodb::AttributeValue {
                ns: Some(v),
                ..Default::default()
            },
            AttributeValue::Null(v) => dynomite::dynamodb::AttributeValue {
                null: Some(v),
                ..Default::default()
            },
            AttributeValue::S(v) => dynomite::dynamodb::AttributeValue {
                s: Some(v),
                ..Default::default()
            },
            AttributeValue::Ss(v) => dynomite::dynamodb::AttributeValue {
                ss: Some(v),
                ..Default::default()
            },
            //AttributeValue::Unknown => unreachable!(),
            _ => unreachable!(),
        }
    }
}

impl ToRusoto<dynomite::Attributes> for HashMap<String, AttributeValue> {
    fn to_rusoto(mut self) -> dynomite::Attributes {
        self.drain().map(|(k, v)| (k, v.to_rusoto())).collect()
    }
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

#[allow(dead_code)]
pub async fn get_user(_aws_config: &SdkConfig, _user_id: u64) -> anyhow::Result<User> {
    // TODO: gotta figure out how to do this bit, ugh
    /*let client = aws_sdk_dynamodb::Client::new(aws_config);
    client
        .get_item()
        .table_name("ssj2023")
       ...
        .send()
        .await?;*/

    anyhow::bail!("unsupported")
}

pub async fn save_user(_aws_config: &SdkConfig, _user: &User) -> anyhow::Result<()> {
    // TODO: gotta figure out how to do this bit, ugh
    /*let client = aws_sdk_dynamodb::Client::new(aws_config);
    client
        .put_item()
        .table_name("ssj2023")
        .set_item(Some(user.key().into().to_sdk()))
        .send()
        .await?;*/

    Ok(())
}

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

#[allow(dead_code)]
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
