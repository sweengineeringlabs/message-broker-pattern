//! [`MessageBroker`] — runtime-agnostic cross-process pub/sub contract.

use crate::{
    BrokerError, BrokerFuture, HealthCheckRequest, PublishRequest, SubscribeRequest,
    SubscribeResponse,
};

/// Cross-process publish/subscribe broker contract.
///
/// Implement this trait to plug in any broker backend — an in-process broker,
/// a network message bus, or a custom transport. `message-broker-svc` ships
/// both the no-op reference broker and production backends.
///
/// # Publish semantics
///
/// Publishing to a topic with no active subscribers silently succeeds (fire and
/// forget).  Implementations that require a subscriber to exist before publishing
/// should document this constraint explicitly.
///
/// # Subscribe semantics
///
/// Each call to [`subscribe`] returns an independent stream.  All active
/// subscribers receive every message published after the subscription was
/// established.  Messages published before [`subscribe`] is called are not
/// delivered.
///
/// [`subscribe`]: MessageBroker::subscribe
pub trait MessageBroker: Send + Sync {
    /// Publish `request.message` to `request.topic`, delivering it to all
    /// active subscribers.
    fn publish<'a>(&'a self, request: PublishRequest) -> BrokerFuture<'a, Result<(), BrokerError>>;

    /// Subscribe to `request.topic`, returning a stream of incoming messages.
    fn subscribe<'a>(
        &'a self,
        request: SubscribeRequest,
    ) -> BrokerFuture<'a, Result<SubscribeResponse, BrokerError>>;

    /// Probe broker connectivity. Returns `Ok(())` if the broker is reachable.
    fn health_check(
        &self,
        request: HealthCheckRequest,
    ) -> BrokerFuture<'_, Result<(), BrokerError>>;
}
