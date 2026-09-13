//! Request for [`crate::MessageBroker::subscribe`].

/// Request for [`crate::MessageBroker::subscribe`].
#[derive(Debug, Clone)]
pub struct SubscribeRequest {
    /// The topic to subscribe to.
    pub topic: String,
}
