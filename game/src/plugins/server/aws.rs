use bevy::prelude::*;

use crate::components::server::aws::*;
use crate::systems::server::aws::*;

pub struct AwsTaskPlugin;

impl Plugin for AwsTaskPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    start_aws_requests::<LoadAwsConfigRequest>,
                    poll_aws_tasks::<<LoadAwsConfigRequest as AwsTaskRequest>::Output>,
                ),
                (
                    (
                        start_aws_requests::<SQSGetQueueUrlRequest>,
                        poll_aws_tasks::<<SQSGetQueueUrlRequest as AwsTaskRequest>::Output>,
                    ),
                    (
                        start_aws_requests::<SQSReceiveMessageRequest>,
                        poll_aws_tasks::<<SQSReceiveMessageRequest as AwsTaskRequest>::Output>,
                    ),
                    (
                        start_aws_requests::<SQSDeleteMessageRequest>,
                        poll_aws_tasks::<<SQSDeleteMessageRequest as AwsTaskRequest>::Output>,
                    ),
                ),
            ),
        );
    }
}
