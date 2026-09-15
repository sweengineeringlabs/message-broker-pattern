//! `message-broker-pattern` — the reusable message-broker pattern.
//!
//! Provides this domain's one primitive: the [`MessageBroker`] trait
//! (fan-out/broadcast — every subscriber gets every message), [`Validator`]
//! (backend config validation, with `validate_config`/`validator_response`
//! as default methods), the message/stream/error value types, and the
//! `*Request`/`*Response` DTOs. Zero implementation of any kind — a
//! single-responsibility interface package, not a "contract" layer bundled
//! alongside a reference implementation or a facade. Both the no-op
//! reference implementation and the technology-specific backends (NATS,
//! Kafka, Postgres), along with the facade that selects among them, live
//! in `message-broker-svc` instead. A consumer depends on this crate plus
//! `message-broker-svc` and defines no primitives of its own.
//!
//! `TaskQueue` (competing-consumer work queue — a different delivery
//! semantic, different consumers, different evolution drivers) lives in a
//! separate crate, [`task-queue-pattern`](https://github.com/sweengineeringlabs/task-queue-pattern)
//! — split out of this crate for SRP; see `docs/3-design/adr/ADR-002` for
//! the full reasoning and `../template-engine`'s `pattern_svc_workflow.md`
//! for the general principle this split follows.
//!
//! Ported from `edge-message-broker` per edge-message-broker#6.
//!
//! `MessageBroker`'s async methods return `impl Future` (RPITIT), not a
//! boxed future — zero-cost, no heap allocation or vtable dispatch per
//! call, at the cost of object safety (no `Box<dyn MessageBroker>`). See
//! `docs/3-design/architecture.md`'s "Why `MessageBroker` returns `impl
//! Future`, not `BrokerFuture`" section.

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
pub use types::MessageStream;
pub use vo::Message;
