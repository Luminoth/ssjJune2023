use bevy::prelude::*;

use crate::systems::server::aws::*;

pub struct AwsTaskPlugin;

impl Plugin for AwsTaskPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    start_load_aws_config_requests,
                    poll_load_aws_config_requests,
                ),
                (
                    (start_sqs_get_queue_url_requests, poll_sqs_get_url_requests),
                    (
                        start_sqs_receive_message_requests,
                        poll_sqs_receive_message_requests,
                    ),
                    (
                        start_sqs_delete_message_requests,
                        poll_sqs_delete_message_requests,
                    ),
                ),
            ),
        );
    }
}
