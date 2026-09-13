//! Coverage for the no-op reference broker
//! (`message-broker-pattern-core::NoopMessageBroker`), exercised through the
//! public `BrokerSvc::noop_broker()` factory.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use futures::StreamExt;
use message_broker_pattern_contract::{
    HealthCheckRequest, Message, PublishRequest, SubscribeRequest, ValidatorRequest,
};
use message_broker_pattern_saf::BrokerSvc;

/// @covers: noop_message_broker — publishing always succeeds (fire-and-forget).
#[tokio::test]
async fn test_noop_publish_returns_ok() {
    let broker = BrokerSvc::noop_broker();
    let result = broker
        .publish(PublishRequest {
            topic: "any.topic".to_string(),
            message: std::sync::Arc::new(Message::new(b"payload".as_ref())),
        })
        .await;
    assert!(result.is_ok(), "noop publish must succeed");
}

/// @covers: noop_message_broker — subscribing yields an immediately-empty stream.
#[tokio::test]
async fn test_noop_subscribe_yields_empty_stream() {
    let broker = BrokerSvc::noop_broker();
    let mut response = broker
        .subscribe(SubscribeRequest {
            topic: "any.topic".to_string(),
        })
        .await
        .unwrap();
    assert!(
        response.stream.next().await.is_none(),
        "noop subscribe must yield no messages"
    );
}

/// @covers: noop_message_broker — health_check always reports healthy.
#[tokio::test]
async fn test_noop_health_check_returns_ok() {
    let broker = BrokerSvc::noop_broker();
    assert!(broker.health_check(HealthCheckRequest).await.is_ok());
}

/// @covers: noop_message_broker — validator returns a usable handle.
#[tokio::test]
async fn test_noop_validator_returns_always_valid_handle() {
    use message_broker_pattern_contract::ValidationRequest;

    let broker = BrokerSvc::noop_broker();
    let response = broker.validator(ValidatorRequest).unwrap();
    assert_eq!(response.validator.validate(ValidationRequest), Ok(()));
}
