//! [`NoopMessageBroker`] — the pattern's reference no-op broker.
//!
//! Publishing discards the message; subscribing yields an immediately-empty
//! stream. Real backends live in `message-broker-svc` — this crate ships only
//! the no-op so the `MessageBroker` trait has an in-tree implementation for
//! contract tests.

use std::sync::Arc;

use message_broker_pattern_contract::{
    BrokerError, BrokerFuture, HealthCheckRequest, MessageBroker, MessageStream, PublishRequest,
    SubscribeRequest, SubscribeResponse, ValidatorRequest, ValidatorResponse,
};

use crate::noop_validator::NoopValidator;

/// No-op [`MessageBroker`]: `publish` succeeds without delivery, `subscribe`
/// returns an empty stream, `health_check` always reports healthy.
pub struct NoopMessageBroker;

impl MessageBroker for NoopMessageBroker {
    fn publish<'a>(
        &'a self,
        _request: PublishRequest,
    ) -> BrokerFuture<'a, Result<(), BrokerError>> {
        BrokerFuture::new(async { Ok(()) })
    }

    fn subscribe<'a>(
        &'a self,
        _request: SubscribeRequest,
    ) -> BrokerFuture<'a, Result<SubscribeResponse, BrokerError>> {
        BrokerFuture::new(async {
            let stream: MessageStream = Box::pin(futures::stream::empty());
            Ok(SubscribeResponse { stream })
        })
    }

    fn health_check(
        &self,
        _request: HealthCheckRequest,
    ) -> BrokerFuture<'_, Result<(), BrokerError>> {
        BrokerFuture::new(async { Ok(()) })
    }

    fn validator(&self, _request: ValidatorRequest) -> Result<ValidatorResponse, BrokerError> {
        Ok(ValidatorResponse {
            validator: Arc::new(NoopValidator),
        })
    }
}
