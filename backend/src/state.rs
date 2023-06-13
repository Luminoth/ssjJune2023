#![allow(dead_code)]

use aws_config::SdkConfig;

#[derive(Debug, Clone)]
pub struct AwsState {
    config: SdkConfig,
    queue_url: String,
}

impl AwsState {
    pub fn new(config: SdkConfig, queue_url: String) -> Self {
        Self { config, queue_url }
    }

    pub fn get_config(&self) -> &SdkConfig {
        &self.config
    }

    pub fn get_queue_url(&self) -> &String {
        &self.queue_url
    }
}
