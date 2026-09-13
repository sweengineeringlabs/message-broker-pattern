# message-broker-pattern

> **TLDR:** The reusable message-broker pattern — `MessageBroker`/`Validator` traits and
> the minimal vocabulary they require, zero implementation, zero knowledge of any
> backend technology. See [Architecture](docs/3-design/architecture.md) for the full
> design.

A single crate, not a workspace, not a broker deployment. Implement `MessageBroker` to
plug in any backend; this repo ships none — see
[`message-broker-svc`](https://github.com/sweengineeringlabs/message-broker-svc) for a
no-op reference broker and real NATS/Kafka/Postgres backends.

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
| `traits/` | `MessageBroker`, `Validator` |
| `vo/` | `Message` |
| `dto/` | `*Request`/`*Response` types |
| `error/` | `BrokerError`, `ValidationError` |
| `types/` | `BrokerFuture`, `MessageStream` |

Extracted from [`edge-message-broker`](https://github.com/sweengineeringlabs/edge-message-broker)
per [edge-message-broker#6](https://github.com/sweengineeringlabs/edge-message-broker/issues/6).
Originally a three-crate `contract`/`core`/`saf` split mirroring
[`wasm-capability-pattern`](https://github.com/sweengineeringlabs/wasm-capability-pattern);
flattened to this single crate, with `core`/`saf` moved to `message-broker-svc` and all
backend-selection vocabulary (`BackendKind`, `MessageBrokerConfig`) deleted outright, not
relocated — see ADR-001's amendments. `cargo test`, `cargo fmt --check`, and `cargo
clippy --all-targets -- -D warnings` all clean; dependency footprint is exactly
`futures` + `thiserror`.

## Documentation

| Document | Description |
|----------|--------------|
| [Docs index](docs/README.md) | Full documentation index |
| [Architecture](docs/3-design/architecture.md) | Component diagram, dependency rationale |
| [ADR-001](docs/3-design/adr/ADR-001-contract-core-saf-split.md) | Why this repo is a single crate, why BackendKind isn't here |
| [Developer Guide](docs/4-development/developer_guide.md) | Repo layout, branching, working on this crate |

## License

MIT OR Apache-2.0
