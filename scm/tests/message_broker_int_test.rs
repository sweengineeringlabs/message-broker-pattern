//! Integration tests for [`MessageBroker`]'s zero-cost shape.

use message_broker_pattern::MessageBroker;

/// @covers: MessageBroker — usable as a generic bound (the RPITIT-compatible
/// replacement for the old object-safety check; `MessageBroker` is no
/// longer object-safe by design, see docs/3-design/architecture.md).
#[allow(dead_code)]
fn _assert_usable_as_generic_bound<T: MessageBroker>() {}
