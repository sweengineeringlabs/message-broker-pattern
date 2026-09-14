# message-broker-pattern

> **TLDR:** The reusable message-broker pattern — every primitive of this domain,
> `MessageBroker` and `TaskQueue` alike, zero implementation, zero knowledge of any
> backend technology. See [Architecture](docs/3-design/architecture.md) for the full
> design.

A single crate, not a workspace, not a broker deployment. Implement `MessageBroker`/
`TaskQueue` to plug in any backend; this repo ships neither — see
[`message-broker-svc`](https://github.com/sweengineeringlabs/message-broker-svc) for a
no-op reference broker and real NATS/Kafka/Postgres backends. A consumer depends on this
crate plus `message-broker-svc` and defines no primitives of its own.

## Quick Start

```rust
use message_broker_pattern::{HealthCheckRequest, MessageBroker};

async fn check(broker: &dyn MessageBroker) -> Result<(), message_broker_pattern::BrokerError> {
    broker.health_check(HealthCheckRequest).await
}
```

## What's Here

| Module | What it is |
|--------|------------|
| `traits/` | `MessageBroker`, `TaskQueue`, `TaskQueueFactoryContract`, `Validator` (backend config validation), `PayloadValidator` (generic self-validation) |
| `vo/` | `Message`, `Task`, `TaskHandle`, `TaskHandleBuilder`, `TaskId` |
| `dto/` | `*Request`/`*Response` types |
| `error/` | `BrokerError`, `QueueError`, `ValidationError` |
| `types/` | `BrokerFuture`, `MessageStream`, marker constants (`VALIDATOR_SVC`, `TASK_QUEUE_FACTORY_CONTRACT_ID`, `MAX_TASK_PAYLOAD_BYTES`, `TASK_ID_HEADER_KEY`) |

Extracted from [`edge-message-broker`](https://github.com/sweengineeringlabs/edge-message-broker)
per [edge-message-broker#6](https://github.com/sweengineeringlabs/edge-message-broker/issues/6).
Originally a three-crate `contract`/`core`/`saf` split mirroring
[`wasm-capability-pattern`](https://github.com/sweengineeringlabs/wasm-capability-pattern);
flattened to this single crate, with `core`/`saf` moved to `message-broker-svc` and all
backend-selection vocabulary (`BackendKind`, `MessageBrokerConfig`,
`ApplicationConfig`/`BrokerBackendConfig`/`ConfigProvider`) deleted outright, not
relocated — a contract crate declares what a backend *is*, never how a specific
application chooses or configures one. `TaskQueue` and its value types were initially
left behind in `edge-runtime`'s own local copy of this contract; that was incomplete
migration, not a scope decision — they belong here alongside `MessageBroker`, so a
consumer gets the whole domain from one place instead of half of it redefined
downstream. `cargo test`, `cargo fmt --check`, and `cargo clippy --all-targets -- -D
warnings` all clean; dependency footprint is `bytes` + `futures` + `thiserror` + `uuid`.

## Documentation

| Document | Description |
|----------|--------------|
| [Docs index](docs/README.md) | Full documentation index |
| [Architecture](docs/3-design/architecture.md) | Component diagram, dependency rationale |
| [ADR-001](docs/3-design/adr/ADR-001-contract-core-saf-split.md) | Why this repo is a single crate, why BackendKind isn't here |
| [Developer Guide](docs/4-development/developer_guide.md) | Repo layout, branching, working on this crate |

## License

MIT OR Apache-2.0
