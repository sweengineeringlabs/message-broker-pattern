//! `message-broker-pattern` — the reusable message-broker pattern.
//!
//! Provides every primitive of this domain: the [`MessageBroker`]/[`TaskQueue`]
//! traits, [`Validator`] (backend config validation) and [`PayloadValidator`]
//! (generic self-validation), the message/task/stream/error value types, the
//! `*Request`/`*Response` DTOs, and the [`TaskQueueFactoryContract`]
//! construction contract. Zero implementation of any kind — a
//! single-responsibility interface package, not a "contract" layer bundled
//! alongside a reference implementation or a facade. Both the no-op reference
//! implementation and the technology-specific backends (NATS, Kafka, Postgres),
//! along with the facade that selects among them, live in `message-broker-svc`
//! instead. A consumer depends on this crate plus `message-broker-svc` and
//! defines no primitives of its own — `MessageBroker` and `TaskQueue` are not
//! split across repos.
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
pub use error::{BrokerError, QueueError, ValidationError};
pub use traits::{MessageBroker, PayloadValidator, TaskQueue, TaskQueueFactoryContract, Validator};
pub use types::{
    BrokerFuture, MessageStream, MAX_TASK_PAYLOAD_BYTES, TASK_ID_HEADER_KEY,
    TASK_QUEUE_FACTORY_CONTRACT_ID, VALIDATOR_SVC,
};
pub use vo::{Message, Task, TaskHandle, TaskHandleBuilder, TaskId};
