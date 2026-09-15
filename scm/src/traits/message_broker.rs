//! [`MessageBroker`] — runtime-agnostic cross-process pub/sub contract.

use std::future::Future;

use crate::{
    BrokerError, HealthCheckRequest, PublishRequest, SubscribeRequest, SubscribeResponse,
    ValidatorRequest, ValidatorResponse,
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
/// # Zero-cost by construction
///
/// `publish`/`subscribe`/`health_check` return `impl Future` (RPITIT), not a
/// boxed future — no heap allocation, no vtable dispatch, on every call.
/// This means `MessageBroker` is not object-safe: there is no
/// `Box<dyn MessageBroker>`/`&dyn MessageBroker`. `message-broker-svc`
/// resolves the resulting need for one uniform, runtime-selectable broker
/// type via a static-dispatch enum (`AnyMessageBroker`), not by clawing
/// object safety back onto this trait — see that repo's own
/// `docs/3-design/architecture.md`.
///
/// [`subscribe`]: MessageBroker::subscribe
pub trait MessageBroker: Send + Sync {
    /// Publish `request.message` to `request.topic`, delivering it to all
    /// active subscribers.
    fn publish(
        &self,
        request: PublishRequest,
    ) -> impl Future<Output = Result<(), BrokerError>> + Send + '_;

    /// Subscribe to `request.topic`, returning a stream of incoming messages.
    fn subscribe(
        &self,
        request: SubscribeRequest,
    ) -> impl Future<Output = Result<SubscribeResponse, BrokerError>> + Send + '_;

    /// Probe broker connectivity. Returns `Ok(())` if the broker is reachable.
    fn health_check(
        &self,
        request: HealthCheckRequest,
    ) -> impl Future<Output = Result<(), BrokerError>> + Send + '_;

    /// Return a handle to this broker's own config validator, so a caller can
    /// revalidate a live broker's configuration (e.g. for health dashboards or
    /// hot-reload checks) without naming the concrete config type.
    fn validator(&self, request: ValidatorRequest) -> Result<ValidatorResponse, BrokerError>;
}
