use aws_config::SdkConfig;
use tracing::info;

pub async fn get_queue_url(aws_config: &SdkConfig) -> anyhow::Result<String> {
    info!("getting queue URL ...");

    let client = aws_sdk_sqs::Client::new(aws_config);
    let result = client.get_queue_url().queue_name("ssj2023").send().await?;
    Ok(result
        .queue_url()
        .ok_or(anyhow::anyhow!("missing queue URL"))?
        .to_owned())
}

pub async fn save_user(aws_config: &SdkConfig) -> anyhow::Result<()> {
    let client = aws_sdk_dynamodb::Client::new(aws_config);
    client
        .put_item()
        .table_name("ssj2023")
        //.item()
        .send()
        .await?;

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
        .unwrap_or_default()
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
