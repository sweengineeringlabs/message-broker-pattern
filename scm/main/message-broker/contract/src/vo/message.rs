//! [`Message`] — the unit of exchange for [`crate::MessageBroker`].

use std::collections::HashMap;

use bytes::Bytes;

/// A message payload with optional metadata headers.
///
/// `Message` is the currency passed between producers and consumers.  It
/// carries raw bytes and an optional key-value header map for routing hints,
/// content-type annotations, or correlation IDs.
///
/// # Examples
///
/// ```rust
/// use message_broker_pattern_contract::Message;
///
/// // Simple payload — no headers.
/// let msg = Message::new(b"order.created:{\"id\":1}".as_ref());
/// assert_eq!(msg.payload.as_slice(), b"order.created:{\"id\":1}");
/// assert!(msg.headers.is_empty());
///
/// // With routing metadata.
/// let msg = Message::with_headers(
///     b"hello".as_ref(),
///     [("content-type".to_string(), "text/plain".to_string())].into(),
/// );
/// assert_eq!(msg.headers.get("content-type").map(String::as_str), Some("text/plain"));
/// ```
#[derive(Debug, Clone)]
pub struct Message {
    /// Payload bytes.
    pub payload: Vec<u8>,
    /// Optional key-value metadata headers.
    pub headers: HashMap<String, String>,
}

impl Message {
    /// Construct a message from raw bytes with no headers.
    pub fn new(payload: impl Into<Bytes>) -> Self {
        Self {
            payload: payload.into().to_vec(),
            headers: HashMap::new(),
        }
    }

    /// Construct a message with headers.
    pub fn with_headers(payload: impl Into<Bytes>, headers: HashMap<String, String>) -> Self {
        Self {
            payload: payload.into().to_vec(),
            headers,
        }
    }
}
