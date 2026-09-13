//! `message-broker-pattern-saf` — the sole consumer-facing construction seam
//! for this pattern's own default `MessageBroker`/`Validator` implementation.
//! A downstream consumer depends on this crate (plus
//! `message-broker-pattern-contract`) alone — never on
//! `message-broker-pattern-core` directly — and never constructs
//! `NoopMessageBroker`/`NoopValidator` itself.
//!
//! Ported from `edge-message-broker` per edge-message-broker#6.

mod broker_svc;

pub use broker_svc::BrokerSvc;
