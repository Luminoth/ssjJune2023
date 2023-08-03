use std::sync::Arc;

use aws_config::SdkConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    aws_config: Arc<SdkConfig>,
}

impl AppState {
    pub fn new(aws_config: SdkConfig) -> Self {
        Self {
            aws_config: Arc::new(aws_config),
        }
    }

    pub fn get_aws_config(&self) -> &SdkConfig {
        &self.aws_config
    }
}
