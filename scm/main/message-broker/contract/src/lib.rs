//! `message-broker-pattern-contract` — runtime-agnostic message-broker contract.
//!
//! Provides the [`MessageBroker`]/[`Validator`] traits, the message/stream/error
//! value types, and the `*Request`/`*Response` DTOs. Zero implementation of any
//! named backend — see `message-broker-pattern-core` for this pattern's own
//! default (no-op) implementation, and `message-broker-svc` for technology-specific
//! backends (NATS, Kafka, Postgres).
//!
//! Ported from `edge-message-broker` per edge-message-broker#6.

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod dto;
mod error;
mod traits;
mod types;
mod vo;

pub use dto::{
    HealthCheckRequest, PublishRequest, SubscribeRequest, SubscribeResponse, ValidationRequest,
    ValidatorRequest, ValidatorResponse,
};
pub use error::{BrokerError, ValidationError};
pub use traits::{MessageBroker, Validator};
pub use types::{BrokerFuture, MessageStream};
pub use vo::{BackendKind, Message};
