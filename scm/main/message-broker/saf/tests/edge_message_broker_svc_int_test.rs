//! Public-API integration tests for the message broker SAF surface.
//!
//! Backend factories (`in_memory_broker`, `nats_broker`, `from_config`) live in
//! `message-broker-svc`; this pattern's SAF exposes only the no-op reference
//! broker and the config-builder/validate helpers.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use message_broker_pattern_contract::HealthCheckRequest;
use message_broker_pattern_saf::BrokerSvc;

/// @covers: noop_broker
#[tokio::test]
async fn test_noop_broker_health_check_returns_ok() {
    assert!(BrokerSvc::noop_broker()
        .health_check(HealthCheckRequest)
        .await
        .is_ok());
}

/// @covers: noop_broker
#[tokio::test]
async fn test_noop_broker_publish_then_subscribe_is_inert() {
    use futures::StreamExt as _;
    use message_broker_pattern_contract::{Message, PublishRequest, SubscribeRequest};

    let broker = BrokerSvc::noop_broker();
    broker
        .publish(PublishRequest {
            topic: "svc-test".to_string(),
            message: std::sync::Arc::new(Message::new(b"ping".as_ref())),
        })
        .await
        .unwrap();
    let mut response = broker
        .subscribe(SubscribeRequest {
            topic: "svc-test".to_string(),
        })
        .await
        .unwrap();
    assert!(
        response.stream.next().await.is_none(),
        "noop broker delivers nothing"
    );
}
