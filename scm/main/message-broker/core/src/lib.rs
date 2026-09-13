//! `message-broker-pattern-core` — this pattern's own default implementation.
//!
//! Ships `NoopMessageBroker`/`NoopValidator` (no named external technology
//! dependency of their own) and `MessageBrokerConfig` (the `[message_broker]`
//! config vocabulary, plus its cross-field validation). A technology-specific
//! backend (NATS, Kafka, Postgres) belongs in `message-broker-svc` instead.
//!
//! Ported from `edge-message-broker` per edge-message-broker#6.

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod message_broker_config;
mod noop_message_broker;
mod noop_validator;

pub use message_broker_config::MessageBrokerConfig;
pub use noop_message_broker::NoopMessageBroker;
pub use noop_validator::NoopValidator;
