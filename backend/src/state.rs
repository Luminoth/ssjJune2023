use std::sync::Arc;

use aws_config::SdkConfig;

use crate::config::ClientConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    aws_config: Arc<SdkConfig>,
    client_config: Arc<ClientConfig>,
}

impl AppState {
    pub fn new(aws_config: SdkConfig, client_config: ClientConfig) -> Self {
        Self {
            aws_config: Arc::new(aws_config),
            client_config: Arc::new(client_config),
        }
    }

    pub fn get_aws_config(&self) -> &SdkConfig {
        &self.aws_config
    }

    pub fn get_client_config(&self) -> &ClientConfig {
        &self.client_config
    }
}
