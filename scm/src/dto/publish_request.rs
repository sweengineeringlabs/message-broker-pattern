//! Request for [`crate::MessageBroker::publish`].

use std::sync::Arc;

use crate::Message;

/// Request for [`crate::MessageBroker::publish`].
#[derive(Debug, Clone)]
pub struct PublishRequest {
    /// The topic to publish to.
    pub topic: String,
    /// The message to deliver.
    pub message: Arc<Message>,
}
