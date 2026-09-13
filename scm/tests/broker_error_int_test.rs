//! Integration tests for [`BrokerError`].

use message_broker_pattern::BrokerError;

/// @covers: BrokerError::Publish
#[test]
fn test_publish_error_includes_topic_and_reason() {
    let e = BrokerError::Publish {
        topic: "orders".into(),
        reason: "no receivers".into(),
    };
    let msg = e.to_string();
    assert!(msg.contains("orders"), "expected topic in message: {msg}");
    assert!(
        msg.contains("no receivers"),
        "expected reason in message: {msg}"
    );
}

/// @covers: BrokerError::StreamLagged
#[test]
fn test_stream_lagged_includes_count() {
    let e = BrokerError::StreamLagged { count: 42 };
    assert!(e.to_string().contains("42"));
}

/// @covers: BrokerError::Connection
#[test]
fn test_connection_error_displays_reason() {
    let e = BrokerError::Connection("refused".into());
    assert!(e.to_string().contains("refused"));
}

/// @covers: BrokerError::Subscribe
#[test]
fn test_subscribe_error_includes_topic_and_reason() {
    let e = BrokerError::Subscribe {
        topic: "events".into(),
        reason: "timeout".into(),
    };
    let msg = e.to_string();
    assert!(msg.contains("events"), "expected topic in message: {msg}");
    assert!(msg.contains("timeout"), "expected reason in message: {msg}");
}
