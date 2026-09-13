//! Integration tests for [`MessageStream`] type alias.

use message_broker_pattern_contract::MessageStream;

/// @covers: MessageStream — type alias is usable as a function argument
#[test]
fn test_message_stream_type_alias_is_usable() {
    fn _accepts_stream(_: &MessageStream) {}
}
